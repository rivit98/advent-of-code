import json
import datetime
from pathlib import Path

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
        return pickle.loads(Path(config.cache).read_bytes())
    except:
        return {}


def save_cache(cache):
    Path(config.cache).write_bytes(pickle.dumps(cache))


def save_file(path, data):
    Path(path).write_text(data)


def load_template(path):
    return Template(Path(path).read_text())


class Challenge:
    def __init__(self, d, n, l, s):
        self.day = d
        self.name = n
        self.link = l
        self.solution = s

    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__, sort_keys=True, indent=4)


class Edition:
    def __init__(self, y, completed, total_challs):
        self.year = y
        self.completed = completed
        self.total = total_challs
        self.percent = round((100 * self.completed) / self.total, 2)


def find_solve_file(chall_id):
    ALLOWED_EXTENSIONS = ['.py']
    for fname in os.listdir('{}/day{:02}'.format(config.folder, chall_id)):
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
    challenge_folders = sorted(filter(lambda s: s.startswith('day'), challenge_folders))
    out_challs = []
    for day_number, challenge_id in enumerate(challenge_folders, 1):
        chall = cached.get(day_number)
        if chall:
            out_challs.append(chall)
        else:
            chall = create_challenge_entry(day_number)
            out_challs.append(chall)
            cached[day_number] = chall

    save_cache(cached)
    return out_challs


def build_editions():
    year_folders = [name for name in os.listdir('..') if
                    os.path.isdir(os.path.join(f'../{name}')) and name.isnumeric()]
    editions = []
    for yf in year_folders:
        yf_path = f'../{yf}'
        challenge_folders = [name for name in os.listdir(yf_path) if os.path.isdir(os.path.join(yf_path, name))]
        editions.append(
            Edition(int(yf), len(challenge_folders), 25)
        )


    return sorted(editions, key=lambda e: e.year, reverse=True)


def main():
    cur_date = datetime.datetime.now().strftime("%d/%m/%Y, %H:%M:%S")
    challs = build_challenges()
    rendered = load_template('./template.md.jinja').render(
        year=config.year,
        solved_challs=challs,
        last_updated=cur_date
    )

    save_file(f"{config.folder}/README.md", rendered)
    print(f"Readme {config.year} generated!")

    editions = build_editions()
    rendered = load_template('./template_main.md.jinja').render(
        editions=editions,
        last_updated=cur_date
    )

    save_file(f"../README.md", rendered)
    print(f"Main Readme {config.year} generated!")


class Config:
    def __init__(self, year):
        self.year = year
        self.folder = f"../{self.year}"
        self.format_link = f"https://adventofcode.com/{self.year}/day/{{}}"
        self.cache = f"{self.folder}/.cache.bin"


if __name__ == '__main__':
    if len(sys.argv) == 1:
        config = Config(datetime.datetime.now().year)
    else:
        config = Config(int(sys.argv[1]))

    main()
