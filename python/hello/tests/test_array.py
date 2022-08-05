import hello


def test_concat():
    assert hello.array.concat([1, 2], [3, 4, 5]) == [1, 2, 3, 4, 5]
