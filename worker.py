from threading import Thread
class WorkPool:
    def __init__(self, nworkers, handler, work: list = []) -> None:
        self.handler = handler
        self.nworkers = nworkers
        self.proceed = False
        self._outputs: list[tuple] = [None for i in work]  # type: ignore
        self.threads: list[Thread] = []
        self.work: list = work

    def worker_handler(self):
        while self.proceed and len(self.work) > 0:
            i = len(self.work)
            inp = self.work.pop()
            self._outputs[i - 1] = (inp, self.handler(*inp))

    def resume(self):
        if not self.proceed:
            self.threads = [
                Thread(target=self.worker_handler) for i in range(self.nworkers)
            ]
            self.proceed = True
            for th in self.threads:
                th.start()

    def start(self):
        self.resume()
        return self

    @property
    def output(self):
        return self._outputs

    def pause(self):

        self.proceed = False
        for th in self.threads:
            th.join()

    def wait(self):
        for th in self.threads:
            th.join()

