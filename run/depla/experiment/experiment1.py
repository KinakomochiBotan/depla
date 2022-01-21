from . import Experiment


class Experiment1(Experiment):
    def __init__(self):
        super(Experiment1, self).__init__(
            '1',
            (
                2001,
                2019,
                True,
                True,
                True,
                False,
                False
            ),
            (
                8,
                8
            )
        )
