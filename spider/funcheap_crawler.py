import json
import logging
import random
import re
import time
from threading import Thread

import requests
from bs4 import BeautifulSoup as bs

from ai_parser import AIParser
from client import Client
from crawler import Crawler

logging.basicConfig(level=logging.INFO,
                    format='%(filename)s %(asctime)s - %(name)s - %(levelname)s - %(message)s',
                    filename='crawler.log', filemode='a')


class FunCheapCrawler(Crawler, Thread):
    """
    A thread of spider to scrape data from https://sf.funcheap.com/events/san-francisco/
    """
    def __init__(self):
        super().__init__(name="FunCheapCrawler")
        self.front_url = "https://sf.funcheap.com/events/san-francisco/"
        self.ls_url = "https://sf.funcheap.com/events/san-francisco/page/{page_num}/"
        self.headers = {
            "Referer": "https://sf.funcheap.com/events/san-francisco/",
            "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"
        }
        self.detail_urls = []

        self.__ai = AIParser()
        self.client = Client()

        self.__logger = logging.getLogger(__name__)

    def list_gen(self):
        """
        Iterate all list pages, save detail pages' urls into memory.
        """
        # get total page count
        front_res = requests.get(url=self.front_url, headers=self.headers)
        front_soup = bs(front_res.text, 'lxml')
        page_count = int(re.findall("of\s(.*)",  front_soup.select(".pages")[0].text)[0])

        # generate list urls
        for i in range(1, page_count+1):
            self.__logger.info(f"Requesting List Page: [{i}]")
            url = self.ls_url.format(page_num=i)

            # request list url and get corresponding detail urls
            page_res = requests.get(url=url, headers=self.headers)
            page_soup = bs(page_res.text, 'lxml')
            for item in page_soup.select('#paged-list')[0].select('.entry-title a'):
                self.detail_urls.append(item['href'])
            time.sleep(random.uniform(0.1, 0.3))

    def detail_gen(self, url):
        """
        Scrape single detail page and parse core contents.

        Args:
            url(str): url of the detail page

        Return:
            content(str): detail page content in string format
        """
        retry_count = 0
        while retry_count < 3:
            try:
                self.__logger.info(f"Requesting Detail Page: [{url}]")
                res = requests.get(url=url, headers=self.headers)
                if res.status_code != 200:
                    retry_count += 1
                    continue
                break
            except Exception as e:
                self.__logger.error(f"Request Error: {e}")
                retry_count += 1

        if res.status_code != 200:
            return ""

        soup = bs(res.text, 'lxml')
        content = soup.select('#content')

        # decompose data unneeded
        if content:
            try:
                content[0].findAll(id="other-events-day-list")[0].decompose()
            except IndexError as e:
                self.__logger.error(e)

            content = content[0].text.strip() if content else ""
        return f"source url:{url}\t"+ content

    def run(self):
        """
        Override the run() in Thread, initiate a new thread for the spider and start scraping.
        """
        self.list_gen()
        for url in self.detail_urls:
            try:
                content = self.detail_gen(url)
                events = json.loads(self.__ai.parse(content=content))['events']
                self.__logger.info(f"[{self.name}]Pushing events from source: [{url}]")
                for idx, event in events.items():
                    self.__logger.info(f"[{self.name}]Adding event [{idx}][{event}]")
                    self.client.push(event)
                time.sleep(random.uniform(0.1, 0.3))
            except Exception as e:
                self.__logger.error(e)
        self.client.fetch()
