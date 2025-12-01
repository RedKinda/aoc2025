import os
import sys
import requests
import datetime

print("Starting scraper.py")

cookie = os.getenv("AOC_SESSION_COOKIE")
assert cookie is not None, "Please set the AOC_SESSION_COOKIE environment variable"

sess = requests.session()
sess.cookies["session"] = cookie

now = datetime.datetime.now()
day = sys.argv[1] if len(sys.argv) > 1 else str(now.day)
if len(day) == 1:
    day = "0" + day
hour = now.hour

if hour >= 6:
    input_files = os.listdir("input")
    fname = "{}.txt".format(day)
    if fname not in input_files:
        print("Downloading input file for day {}".format(day))
        with open("input/" + fname, "w") as f:
            data = sess.get(f"https://adventofcode.com/2025/day/{int(day)}/input")
            f.write(data.text)

    fname_sample = "{}_sample.txt".format(day)
    if fname_sample not in input_files:
        buff = ""
        newline_count = 0
        print("Copypaste the sample input for day {}: ".format(day))
        while True:
            try:
                line = input()
            except EOFError:
                break

            if line == "":
                newline_count += 1
                if newline_count >= 4:
                    break
            else:
                newline_count = 0

            buff += line + "\n"

        print("Writing sample input to file {}".format(fname_sample))

        with open("input/" + fname_sample, "w") as f:
            f.write(buff)

    sol_filename_py_a = f"{day}a.py"
    sol_filename_py_b = f"{day}b.py"
    sol_filename_rs_a = f"a{day}.rs"
    sol_filename_rs_b = f"b{day}.rs"

    if sol_filename_py_a not in os.listdir("aoc"):
        print("Creating python sol file for day {} a".format(day))
        with open("aoc/" + sol_filename_py_a, "w") as f:
            with open("template.py") as t:
                f.write(
                    t.read().replace(
                        "# scraper pastes the day var here",
                        f'day = "{day}"',
                    )
                )

    if sol_filename_rs_a not in os.listdir("src"):
        print("Creating rust sol file for day {} a".format(day))
        with open("src/" + sol_filename_rs_a, "w") as f:
            with open("template.rs") as t:
                f.write(t.read())

    part_id = "a"

    front_page = sess.get(f"https://adventofcode.com/2025/day/{int(day)}").text
    if "--- Part Two ---" in front_page:
        part_id = "b"
        # copy sol a to sol b
        if sol_filename_py_b not in os.listdir("aoc"):
            print("Creating python sol file for day {} b".format(day))
            with open("aoc/" + sol_filename_py_b, "w") as f:
                with open("aoc/" + sol_filename_py_a) as t:
                    sol_a = t.read()
                    f.write(sol_a)

        if sol_filename_rs_b not in os.listdir("src"):
            print("Creating rust sol file for day {} b".format(day))
            with open("src/" + sol_filename_rs_b, "w") as f:
                with open("src/" + sol_filename_rs_a) as t:
                    f.write(t.read())

    # modify main.rs to run todays sol
    with open("src/main.rs") as f:
        main = f.readlines()
    with open("src/main.rs", "w") as f:
        f.write(f"pub mod {part_id}{day};\n")
        f.write(f"pub use {part_id}{day}::{{run, SAMPLE_OUTPUT}};\n")
        f.write(f'pub const DAY: &str = "{day}";\n')
        f.write(f'pub const PART: &str = "{part_id}";\n')
        f.write("".join(main[4:]))
