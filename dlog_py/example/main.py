import os
from dlog_py import with_dlog


@with_dlog(os.environ['DLOG_API_KEY'])
def example(logger):
    logger.warning("Hello World! 1111-2a222-3333-4444")


example()