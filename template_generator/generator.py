import json
import datetime
import requests
import os
import re
import sys
import pickle
from jinja2 import Template
from bs4 import BeautifulSoup

config = None
title_extraction_regex = re.compile(r': (.*) -')


def load_cache():
    try:
        with open(config.cache, "rb") as f:
            return pickle.load(f)
    except:
        return {}


def save_cache(cache):
    try:
        with open(config.cache, "wb") as f:
            return pickle.dump(cache, f)
    except Exception as e:
        print(e)


def save_readme(data):
    with open("{}/README.md".format(config.folder), "wt") as f:
        return f.write(data)


def load_template():
    with open('./template.md.jinja') as f:
        return Template(f.read())


class Challenge:
    def __init__(self, d, n, l, s):
        self.day = d
        self.name = n
        self.link = l
        self.solution = s

    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__, sort_keys=True, indent=4)


def find_solve_file(chall_id):
    ALLOWED_EXTENSIONS = ['.py']
    IGNORED_EXTENSIONS = ['.txt', '.pyc', '.md']
    for fname in os.listdir('{}/day{:02}'.format(config.folder, chall_id)):
        if any(fname.endswith(x) for x in IGNORED_EXTENSIONS):
            continue

        if any(fname.endswith(x) for x in ALLOWED_EXTENSIONS):
            return fname


def create_challenge_entry(chall_id):
    link = config.format_link.format(chall_id)
    print("Downloading title from: " + link)
    request = requests.get(link)
    soup = BeautifulSoup(request.content, 'html.parser')
    article = soup.find('article', {"class": "day-desc"})
    header = article.findChildren("h2", recursive=False)
    if not len(header):
        print("downloading data error! h2 not found for: " + link)
        sys.exit(1)

    header_text = header[0].text
    match = re.search(title_extraction_regex, header_text)
    title = match.group(1)
    filename = find_solve_file(chall_id)

    return Challenge(
        chall_id,
        title,
        link,
        './day{:02}/{}'.format(chall_id, filename)
    )


def build_challenges():
    cached = load_cache()
    challenge_folders = [name for name in os.listdir(config.folder) if os.path.isdir(os.path.join(config.folder, name))]
    challenge_folders = sorted(list(filter(lambda s: s.startswith('day'), challenge_folders)))
    out_challs = []
    for day_number, challenge_id in enumerate(challenge_folders):
        day_number += 1
        chall = cached.get(day_number)
        if chall:
            out_challs.append(chall)
        else:
            chall = create_challenge_entry(day_number)
            out_challs.append(chall)
            cached[day_number] = chall


    save_cache(cached)
    return out_challs


def main():
    challs = build_challenges()

    rendered = load_template().render(
        year=config.year,
        solved_challs=challs,
        last_updated=datetime.datetime.now().strftime("%d/%m/%Y, %H:%M:%S")
    )

    save_readme(rendered)
    print("Readme {} generated!".format(config.year))


class Config:
    def __init__(self, year):
        self.year = year
        self.folder = "../{}".format(self.year)
        self.format_link = "https://adventofcode.com/{}/day/{{}}".format(self.year)
        self.cache = "{}/.cache{}.bin".format(self.folder, self.year)



if __name__ == '__main__':
    if len(sys.argv) == 1:
        config = Config(datetime.datetime.now().year)
    else:
        config = Config(int(sys.argv[1]))

    main()
