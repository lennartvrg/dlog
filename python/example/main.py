import os
from dlog_python import with_dlog


@with_dlog(os.environ['DLOG_API_KEY'])
def example(logger):
    logger.warning("Johannes")


example()