from auth import User, requests


class AbstractUser:
    def __init__(self, user: User) -> None:
        self.user = user
        self.BASE_URL = "http://localhost"
        self.session = requests.Session()
        self.session.headers["Authorization"] = f"Bearer {user.access}"

    def post(self, url, **kwargs):
        url = f"{self.BASE_URL}{url}"
        print(url)
        return self.session.post(url, **kwargs)

    def get(self, url, **kwargs):
        return self.session.get(f"{self.BASE_URL}{url}", **kwargs)
