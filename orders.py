from dataclasses import dataclass
from auth import User
from ferrumec import AbstractUser


@dataclass
class OrderItem:
    product: str
    quantity: int

    def into_dict(self):
        return {"product_id": self.product, "quantity": self.quantity}


@dataclass
class Order:
    items: list[OrderItem]

    def into_dict(self):
        return {"items": [i.into_dict() for i in self.items]}


class OrdersUser(AbstractUser):
    def __init__(self, user: User) -> None:
        super().__init__(user)
        self.BASE_URL = self.BASE_URL + "/orders"

    def create(self, items: list[tuple[str, int]]):
        _items: list[OrderItem] = [OrderItem(p, q) for p, q in items]
        order = Order(_items)
        return self.post("", json=order.into_dict())

    def get_order(self, order_id):
        return self.get("/" + order_id)

    def get_orders(
        self,
    ):
        return self.get("")
