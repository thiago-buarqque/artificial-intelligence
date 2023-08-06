from abc import abstractmethod, ABC


class DTO(ABC):
    @abstractmethod
    def to_dict(self) -> dict:
        pass
