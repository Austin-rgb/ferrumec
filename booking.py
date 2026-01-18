from attr import dataclass
from auth import User
from ferrumec import AbstractUser


@dataclass
class Bookable:
    capacity: int
    name: str
    description: str


@dataclass
class Booking:
    bookable: int
    start_time: int
    end_time: int


@dataclass
class Availability:
    start_time: int
    end_time: int


class BookingUser(AbstractUser):
    def __init__(self, user: User) -> None:
        super().__init__(user)
        self.BASE_URL += ":8082"

    def post_bookable(self, bookable: Bookable):
        res = self.post("/bookables", json=bookable)
        return res.json()

    def get_bookables(self):
        res = self.get("/bookables")
        return res.json()

    def get_availability(self, bookable, availability: Availability):
        res = self.get(f"/bookables/{bookable}/availability", json=availability)
        return res.json()

    def get_bookings(self):
        res = self.get("/bookings")
        return res.json()

    def create_booking(self, booking: Booking):
        res = self.post("/create_booking", json=booking)
        return res.json()


user = User("alice")
bu = BookingUser(user)
bu.post_bookable(
    Bookable(capacity=10, name="apartment", description="my cute apartment")
)
