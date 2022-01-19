from abc import ABC
from wthor import LoadOption
from . import Experiment, DatasetSetting, AISetting


class Experiment3(Experiment, ABC):
    def __init__(self):
        super(Experiment3, self).__init__(3)

    @property
    def _datasets(self):
        return (
            DatasetSetting(2001, 2018, LoadOption().unique().augmentation().win().draw().lose()),
            DatasetSetting(2019, 2019, LoadOption().unique().augmentation().win().draw().lose())
        )

    @property
    def _ais(self):
        return AISetting(8, 0, 1, 8, True),
