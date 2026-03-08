from random import choice
import string
from auth import User, requests, resp


def rand_name(len):
    return "".join([choice(string.ascii_lowercase) for _ in range(len)])


def rand_desc(len):
    return " ".join([rand_name(6) for _ in range(len)])


def print_error(func):
    def handler(*args, **kwargs):
        res: requests.Response = func(*args, **kwargs)
        if not res.ok:
            print(res.status_code, res.content)

        return res

    return handler


class AbstractUser:
    def __init__(self, user: User) -> None:
        self.user = user
        self.BASE_URL = "http://localhost:8080"
        self.session = requests.Session()
        self.session.headers["Authorization"] = f"Bearer {user.access}"

    @resp
    def post(self, url, **kwargs):
        url = f"{self.BASE_URL}{url}"
        return self.session.post(url, **kwargs)

    @resp
    def get(self, url, **kwargs):
        return self.session.get(f"{self.BASE_URL}{url}", **kwargs)
