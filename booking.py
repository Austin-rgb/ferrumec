from time import sleep
from attr import dataclass
from auth import User
from ferrumec import AbstractUser, rand_desc, rand_name
from worker import WorkPool


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
        self.BASE_URL += ":8080"

    def post_bookable(self, bookable: Bookable):
        res = self.post("/bookables", json=bookable.__dict__)
        return res.json()

    def get_bookables(self):
        res = self.get("/bookables")
        return res.json()

    def get_availability(self, bookable, availability: Availability):
        res = self.get(
            f"/bookables/{bookable}/availability", json=availability.__dict__
        )
        return res.json()

    def get_bookings(self):
        res = self.get("/bookings")
        return res.json()

    def create_booking(self, booking: Booking):
        res = self.post("/create_booking", json=booking.__dict__)
        return res.json()


user = User("alice")
bu = BookingUser(user)
bu.post_bookable(
    Bookable(capacity=10, name="apartment", description="my cute apartment")
)
print(bu.get_bookables())
print("load testing")
bookables = [(Bookable(5, rand_name(7), rand_desc(6)),) for _ in range(900)]
wp = WorkPool(8, bu.post_bookable, bookables)
wp.start()
sleep(10)
wp.pause()
out = wp.output
print("finished", len(bu.get_bookables()))
