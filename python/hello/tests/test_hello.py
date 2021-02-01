import hello

def test_print_string():
    hello.print_string("world")

def test_get_string():
    assert hello.get_string("world") == "Hello world!"
