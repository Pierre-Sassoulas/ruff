with A() as a:  # SIM117
    with B() as b:
        print("hello")

with A():  # SIM117
    with B():
        with C():
            print("hello")

with A() as a:
    a()
    with B() as b:
        print("hello")

with A() as a:
    with B() as b:
        print("hello")
    a()
