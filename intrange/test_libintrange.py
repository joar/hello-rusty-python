import pytest

from libintrange import IntRange


def test_intrange():
    assert list(IntRange(0, 10)) == list(range(0, 10))


@pytest.mark.parametrize(
    "args, expected",
    [
        pytest.param(
            (-1, 10),
            None,
            marks=pytest.mark.xfail(raises=OverflowError, strict=True),
        ),
        pytest.param((1, 1), [1]),
        pytest.param((1, 0), [1]),
    ],
    ids=repr,
)
def test_bad_input(args, expected):
    result = list(IntRange(*args))
    assert result == expected
