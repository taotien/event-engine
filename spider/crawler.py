class Crawler:
    """
    Crawlers interface, work as a guideline for crawler implementation.
    """
    def list_gen(self):
        """
        request list pages and collect detail page urls
        """
        pass

    def detail_gen(self, url):
        """
        request detail pages and parse contents
        """
        pass
