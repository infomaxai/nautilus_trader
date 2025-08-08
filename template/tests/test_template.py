from template import greet


def test_greet():
    assert greet() == "Hello from Rust!"
