from . import Experiment


class Experiment1(Experiment):
    def __init__(self):
        super(Experiment1, self).__init__(
            '1',
            (
                2010,
                2019,
                False,
                False,
                True,
                True,
                True
            ),
            (
                4,
                4
            )
        )
