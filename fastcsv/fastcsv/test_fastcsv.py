import tempfile

import pytest

from fastcsv import CSVReader


@pytest.mark.parametrize(
    "csv_content, expected", [(b"x\x01y\x01z\x02\n", [["x", "y", "z\x02"]])]
)
def test_reader(csv_content, expected):
    with tempfile.NamedTemporaryFile("w+b") as writable_csv_fd:
        writable_csv_fd.write(b"x\x01y\x01z\x02\n")
        writable_csv_fd.flush()
        r = CSVReader(writable_csv_fd.name)
        assert list(r) == expected
