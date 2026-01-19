from auth import User, requests
from ferrumec import AbstractUser


class CreateItemError(Exception):
    pass


class GetItemsError(Exception):
    pass


class CommitException(Exception):
    pass


class ReserveException(Exception):
    pass


class ReleaseException(Exception):
    pass


class InventoryUser(AbstractUser):
    def __init__(self, user: User):  # user: User) -> None:
        super().__init__(user)
        self.BASE_URL += "/inventory"

    def post_item(self, id, sku, quantity):
        res = self.session.post(
            f"{self.BASE_URL}/items",
            json={"id": str(id), "sku": sku, "quantity": quantity},
            timeout=10,
        )
        if not res.ok:
            raise CreateItemError(f"{res.status_code}:{res.content}")

    def get_items(self):
        res = self.session.get(f"{self.BASE_URL}/items")
        if not res.ok:
            raise GetItemsError(f"{res.status_code}:{res.content}")

        return res.json()

    def commit(self, item, quantity):
        res = self.session.post(f"{self.BASE_URL}/items/{item}/commit/{quantity}")
        if not res.ok:
            raise CommitException(f"{res.status_code}:{res.content}")

    def reserve(self, item, quantity):
        res = self.session.post(f"{self.BASE_URL}/items/{item}/reserve/{quantity}")
        if not res.ok:
            raise ReserveException(f"{res.status_code}:{res.content}")

    def release(self, item, quantity):
        res = self.session.post(f"{self.BASE_URL}/items/{item}/release/{quantity}")
        if not res.ok:
            raise ReleaseException(f"{res.status_code}:{res.content}")


if __name__ == "__main__":
    user = User("alice")
    inventory_user = InventoryUser(user)
    inventory_user.post_item(2, "my item", 25)
    print(inventory_user.get_items())
    inventory_user.reserve(2, 10)
    print(inventory_user.get_items())
    inventory_user.commit(2, 5)
    print(inventory_user.get_items())
