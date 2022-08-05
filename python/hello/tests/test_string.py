import hello


def test_to_uppercase():
    assert hello.string.to_uppercase("Hello World!") == "HELLO WORLD!"
