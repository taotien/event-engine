from funcheap_crawler import FunCheapCrawler
from usfca_crawler import USFCrawler


if __name__ == '__main__':
    """
    Entry of all crawlers.

    Create instances for all crawlers. Initiate threads for everyone.
    """
    for crawler in [FunCheapCrawler, USFCrawler]:
        c = crawler()
        c.start()
