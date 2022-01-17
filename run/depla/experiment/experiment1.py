from abc import ABC
from wthor import LoadOption
from . import Experiment, DatasetSetting, AISetting


class Experiment1(Experiment, ABC):
    def __init__(self):
        super(Experiment1, self).__init__(1)

    @property
    def _datasets(self):
        return (
            DatasetSetting(2010, 2018, LoadOption().win().draw().lose()),
            DatasetSetting(2019, 2019, LoadOption().win().draw().lose())
        )

    @property
    def _ais(self):
        return AISetting(4, 0, 1, 4, True),
