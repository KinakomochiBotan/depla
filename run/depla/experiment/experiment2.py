from abc import ABC
from wthor import LoadOption
from . import Experiment, DatasetSetting, AISetting


class Experiment2(Experiment, ABC):
    def __init__(self):
        super(Experiment2, self).__init__(2)

    @property
    def _datasets(self):
        return (
            DatasetSetting(2010, 2018, LoadOption().unique().augmentation().win().draw()),
            DatasetSetting(2019, 2019, LoadOption().win().draw().lose())
        )

    @property
    def _ais(self):
        return AISetting(8, 0, 1, 8, True),
