import logging
from .dlog_py import PythonLogger as _PythonLogger


class DlogLogger(logging.StreamHandler):
    def __init__(self, api_key, sanitize_emails=True, sanitize_credit_cards=True):
        logging.StreamHandler.__init__(self)
        try:
            self.instance = _PythonLogger(api_key, sanitize_emails, sanitize_credit_cards)
        except ValueError as ex:
            print(ex)

    def emit(self, record):
        if hasattr(self, 'instance'):
            self.instance.log(record.levelno, self.format(record))

    def flush(self) -> None:
        if hasattr(self, 'instance'):
            self.instance.flush()

    def close(self):
        if hasattr(self, 'instance'):
            self.instance.clean_up()


def with_dlog(api_key, level=None, sanitize_emails=False, sanitize_credit_cards=False):
    if level is None:
        level = logging.WARNING

    dlog = DlogLogger(api_key, sanitize_emails, sanitize_credit_cards)

    logger = logging.getLogger('log')
    logger.setLevel(level)
    logger.addHandler(dlog)

    def wrapper(handler):
        def inner(*args):
            res = handler(logger, *args)
            dlog.flush()
            return res

        return inner

    return wrapper
