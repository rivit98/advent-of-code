#!/usr/bin/env python3
import json
import datetime
from dataclasses import dataclass
from pathlib import Path
from pygments.lexers import get_lexer_for_filename

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
    Path(path).write_text(data, newline='\n')


def load_template(path):
    return Template(Path(path).read_text())

@dataclass
class Challenge:
    day: int
    name: str
    lang: str
    link: str
    solution: str

    def toJSON(self):
        return json.dumps(self, default=lambda o: o.__dict__, sort_keys=True, indent=4)


class Edition:
    def __init__(self, y, completed, total_challs):
        self.year = y
        self.completed = completed
        self.total = total_challs
        self.percent = round((100 * self.completed) / self.total, 2)


def find_solve_file(chall_id):
    ALLOWED_EXTENSIONS = ['.py', '.rs', '.go', '.c', '.cpp']
    for fname in os.listdir(f'{config.folder}/day{chall_id:02}'):
        if any(fname.endswith(x) for x in ALLOWED_EXTENSIONS):
            return fname, get_lexer_for_filename(fname).name

def scrap_chall_name(link):
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
    return title


def build_challenges():
    cached = load_cache()
    challenge_folders = [directory for directory in Path(config.folder).iterdir() if directory.is_dir() and directory.name.startswith('day')]
    challenge_folders = list(map(lambda s: int(s.name.removeprefix('day')), challenge_folders))
    out_challs = []
    for day_number in sorted(challenge_folders):
        try:
            link = config.format_link.format(day_number)
            filename, lang = find_solve_file(day_number)
            name = cached.get(day_number) or scrap_chall_name(link)
            chall = Challenge(
                day_number,
                name,
                lang,
                link,
                f'./day{day_number:02}/{filename}'
            )
        except Exception as e:
            break

        out_challs.append(chall)
        cached[day_number] = name

    save_cache(cached)
    return out_challs


def build_editions():
    year_folders = [directory for directory in Path('.').iterdir() if directory.is_dir() and directory.name.isnumeric()]
    editions = []
    for yf_path in year_folders:
        challenge_folders = [directory for directory in yf_path.iterdir() if directory.is_dir() and directory.name.startswith("day") and any(directory.iterdir())]
        editions.append(
            Edition(int(yf_path.name), len(challenge_folders), 25)
        )

    return sorted(editions, key=lambda e: e.year, reverse=True)


def main():
    challs = build_challenges()
    rendered = load_template('./templates/template.md.jinja').render(
        year=config.year,
        solved_challs=challs,
    )

    save_file(config.folder / "README.md", rendered)
    print(f"Readme {config.year} generated!")

    editions = build_editions()
    rendered = load_template('./templates/template_main.md.jinja').render(
        editions=editions,
    )

    save_file("README.md", rendered)
    print(f"Main Readme {config.year} generated!")


class Config:
    def __init__(self, year):
        self.year = year
        self.folder = Path(f"./{self.year}")
        if not self.folder.exists():
            self.folder.mkdir(parents=True, exist_ok=True)

        self.format_link = f"https://adventofcode.com/{self.year}/day/{{}}"
        self.cache = self.folder / ".cache.bin"


if __name__ == '__main__':
    if len(sys.argv) == 1:
        config = Config(datetime.datetime.now().year)
    else:
        config = Config(int(sys.argv[1]))

    main()
