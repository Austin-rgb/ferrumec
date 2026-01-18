from auth import User
from ferrumec import AbstractUser


class OrdersUser(AbstractUser):
    def __init__(self, user: User) -> None:
        super().__init__(user)
        self.BASE_URL = "http://localhost:8080"

    def create(self):
        pass

    def get(self):
        pass
