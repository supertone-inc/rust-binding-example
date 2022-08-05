import hello


def test_map():
    assert hello.callback.map([1, 2, 3], lambda item: item * 2) == [2, 4, 6]
