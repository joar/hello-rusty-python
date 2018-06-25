from libintrange import IntRange

def test_intrange():
    assert list(IntRange(0, 10)) == list(range(0, 10))
