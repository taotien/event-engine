import json
import logging
import random
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


class USFCrawler(Crawler, Thread):
    """
    A thread of spider to scrape data from https://www.usfca.edu/life-usf/events.
    """
    def __init__(self):
        super().__init__(name="USFCrawler")
        self.__domain = "https://www.usfca.edu"
        self.__ls_url = "https://www.usfca.edu/life-usf/events?viewsreference[enabled_settings][argument]=argument&page={page_num}"
        self.__headers = {"User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"}
        self.__detail_urls = []
        self.__ai = AIParser()
        self.__client = Client()

        self.__logger = logging.getLogger(__name__)

    def list_gen(self):
        """
        Iterate all list pages, save detail pages' urls into memory.
        """
        page_num = 1
        while True:
            url = self.__ls_url.format(page_num=page_num)
            ls_res = requests.get(url=url, headers=self.__headers)
            ls_soup = bs(ls_res.text, 'lxml')

            # register forms are skipped
            detail_urls = [self.__domain + item['href'] for item in ls_soup.select('.text-container a') if 'register' not in item['href']]
            if len(detail_urls) < 5:
                break
            for url in detail_urls[5:]:
                self.__detail_urls.append(url)
            time.sleep(random.uniform(0.1, 0.3))
            page_num += 1

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
            self.__logger.info(f"Requesting Detail Page: [{url}]")
            res = requests.get(url=url, headers=self.__headers)
            if res.status_code != 200:
                retry_count += 1
                continue
            time.sleep(random.uniform(0.1, 0.3))
            break

        if res.status_code != 200:
            return ""

        # parse details
        soup = bs(res.text, 'lxml')

        content1 = soup.select('.content')
        content2 = soup.select('#content')
        content1 = content1[0].text if content1 else ""
        content2 = content2[0].text if content2 else ""
        content = f"source_url: {url}\t" + content1.strip() + content2.strip()

        return content

    def run(self):
        """
        Override the run() in Thread, initiate a new thread for the spider and start scraping.
        """
        self.list_gen()
        for url in self.__detail_urls:
            try:
                content = self.detail_gen(url)

                events = json.loads(self.__ai.parse(content=content))['events']
                self.__logger.info(f"[{self.name}]Pushing events for source: [{url}]")
                for idx, event in events.items():
                    self.__logger.info(f"[{self.name}]Adding event [{idx}][{event}]")
                    self.__client.push(event)
                time.sleep(random.uniform(0.1, 0.3))
            except Exception as e:
                self.__logger.error(e)

        self.__client.fetch()
