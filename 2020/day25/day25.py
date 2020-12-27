with open("./input.txt") as f:
    data = f.read().splitlines()

card_public = int(data[0])
door_public = int(data[1])

print(card_public, door_public)

subj = 1
for i in range(1, 20000000):
    subj *= 7
    subj %= 20201227
    if subj == card_public:
        print("found card_loop_size", i)
        print("enc key", pow(door_public, i, 20201227))

    if subj == door_public:
        print("found door_loop_size", i)
        print("enc key", pow(card_public, i, 20201227))


