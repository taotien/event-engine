import json
import random
import re
import time

import requests
from bs4 import BeautifulSoup as bs

from ai_parser import AIParser
from client import Client
from crawler import Crawler


class funcheapCrawler(Crawler):
    def __init__(self):
        self.front_url = "https://sf.funcheap.com/events/san-francisco/"
        self.ls_url = "https://sf.funcheap.com/events/san-francisco/page/{page_num}/"
        self.headers = {
            "Referer": "https://sf.funcheap.com/events/san-francisco/",
            "User-Agent": "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/123.0.0.0 Safari/537.36"
        }
        self.detail_urls = []

        self.ai = AIParser()
        self.client = Client()

    def list_gen(self):
        # get total page count
        front_res = requests.get(url=self.front_url, headers=self.headers)
        front_soup = bs(front_res.text, 'lxml')
        page_count = int(re.findall("of\s(.*)",  front_soup.select(".pages")[0].text)[0])

        # generate list urls
        for i in range(1, page_count+1):
            url = self.ls_url.format(page_num=i)
            page_res = requests.get(url=url, headers=self.headers)
            page_soup = bs(page_res.text, 'lxml')
            for item in page_soup.select('#paged-list')[0].select('.entry-title a'):
                self.detail_urls.append(item['href'])
            time.sleep(random.uniform(0.1, 0.5))

    def detail_gen(self, url):
        retry_count = 0
        while retry_count < 3:
            try:
                res = requests.get(url=url, headers=self.headers)
                if res.status_code != 200:
                    retry_count += 1
                    continue
                break
            except Exception as e:
                print(f"Request Error: {e}")
                retry_count += 1
                continue

        if res.status_code != 200:
            return ""

        soup = bs(res.text, 'lxml')
        content = soup.select('#content')

        # decompose data unneeded
        if content:
            try:
                content[0].findAll(id="other-events-day-list")[0].decompose()
            except IndexError as e:
                print(e)

            content = content[0].text.strip() if content else ""
        return f"source url:{url}\t"+ content

    def run(self):
        self.list_gen()
        for url in self.detail_urls:
            try:
                content = self.detail_gen(url)
                events = json.loads(self.ai.parse(content=content))['events']
                print(f"Pushing events for source: [{url}]")
                for idx, event in events.items():
                    print(f">Adding event [{idx}][{event}]")
                    self.client.push(event)
                time.sleep(random.uniform(0.1, 0.5))
            except Exception as e:
                print(e)
        self.client.fetch()


if __name__ == '__main__':
    fc = funcheapCrawler()
    fc.run()
