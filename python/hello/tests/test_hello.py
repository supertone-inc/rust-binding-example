import hello

def test_to_uppercase():
    assert hello.to_uppercase("Hello World!") == "HELLO WORLD!"

def test_concat():
    assert hello.concat([1, 2], [3, 4, 5]) == [1, 2, 3, 4, 5]
