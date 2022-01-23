from . import Experiment


class Experiment21(Experiment):
    def __init__(self):
        super(Experiment21, self).__init__(
            '2.1',
            (
                2010,
                2019,
                True,
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


class Experiment22(Experiment):
    def __init__(self):
        super(Experiment22, self).__init__(
            '2.2',
            (
                2010,
                2019,
                False,
                True,
                True,
                True,
                True
            ),
            (
                4,
                4
            )
        )


class Experiment23(Experiment):
    def __init__(self):
        super(Experiment23, self).__init__(
            '2.3',
            (
                2010,
                2019,
                True,
                True,
                True,
                True,
                True
            ),
            (
                4,
                4
            )
        )
