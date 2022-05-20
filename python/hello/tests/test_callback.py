import hello

def test_map():
    def mapper(item):
        return item * 2

    assert hello.callback.map([1, 2, 3], mapper) == [2, 4, 6]
