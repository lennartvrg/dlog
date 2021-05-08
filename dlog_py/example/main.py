import os

from dlog_py import with_dlog


@with_dlog(os.environ['DLOG_API_KEY'], sanitize_credit_cards=True)
def example(logger):
    logger.warning(f"Hello World!")


example()
