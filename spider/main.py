from funcheap_crawler import FunCheapCrawler
from usfca_crawler import USFCrawler


if __name__ == '__main__':
    for crawler in [FunCheapCrawler, USFCrawler]:
        c = crawler()
        c.start()
