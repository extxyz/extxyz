from collections.abc import Iterator
import os


class Frame(object):
    """class to refresent a structure in a frame"""
    def natoms(self) -> int:
        """number of atoms of the frame"""

    def info(self) -> dict[str, str | int | float | bool | list[float | int]]:
        """the info line as a dict"""

    def arrs(self) -> dict[str, list[float | int | bool | str]]:
        """the xyz store for columns"""

def read_frame() -> Frame:
    """read frame from a stream-like obj"""
    
def read_frames() -> Iterator[Frame]:
    """read frames as Iterator from a stream-like obj"""

def read_frame_from_file(inp: str | bytes | os.PathLike[str], /) -> Frame: 
    """read frame from file-like object"""

def read_frames_from_file(inp: str | bytes | os.PathLike[str], /) -> Iterator[Frame]: 
    """read frames from file-like object, return an iterator of frames"""

def write_frame(frame: Frame, /) -> str | bytes | os.PathLike[str]: 
    """write frame"""

def write_frames(inp: str | bytes | os.PathLike[str], /) -> Iterator[Frame]: 
    """read frames from file-like object, return an iterator of frames"""
