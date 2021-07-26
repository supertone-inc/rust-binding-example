import hello

def test_concat():
    assert hello.concat([1, 2], [3, 4, 5]) == [1, 2, 3, 4, 5]
