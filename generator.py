#!/usr/bin/env python3
import json
import datetime
from collections import defaultdict
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

title_extraction_regex = re.compile(r': (.*) -')
cached = None
cache_file = ".cache.bin"

def load_cache():
    try:
        return pickle.loads(Path(cache_file).read_bytes())
    except:
        return defaultdict(dict)


def save_cache(cache):
    Path(cache_file).write_bytes(pickle.dumps(cache))


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
    def __init__(self, edition_dir, y, solution_files):
        self.dir = edition_dir
        self.year = y
        self.completed = len(set([c.day for c in solution_files]))
        self.total = 25 if y < 2025 else 12
        self.percent = round((100 * self.completed) / self.total, 2)
        self.challenges = solution_files


def scrap_chall_name(link):
    print("Downloading title from: " + link)
    request = requests.get(link)
    soup = BeautifulSoup(request.content, 'html.parser')
    article = soup.find('article', {"class": "day-desc"})
    header = article.find_all("h2", recursive=False)
    if not len(header):
        print("downloading data error! h2 not found for: " + link)
        sys.exit(1)

    header_text = header[0].text
    match = re.search(title_extraction_regex, header_text)
    title = match.group(1)
    return title

def is_valid_solution_file(f):
    sf = str(f)
    for i in range(1, 26):
        if f'day{i:02}' in sf or f'day{i}.' in sf or f'd{i}.' in sf:
            return i

    return None

def build_solution(year, f):
    global cached

    day_number = is_valid_solution_file(f)

    if day_number is None:
        raise Exception("Cannot find day number for file: " + str(f))

    try:
        lang = get_lexer_for_filename(f).name
        link = f"https://adventofcode.com/{year}/day/{day_number}"
        name = cached.get(year, {}).get(day_number) or scrap_chall_name(link)
        chall = Challenge(
            day_number,
            name,
            lang,
            link,
            f.relative_to(Path(str(year)))
        )
        cached[year][day_number] = name
        return chall
    except Exception as e:
        raise Exception(f"Error building solution for file: {f}: {e}")


def build_editions():
    year_folders = [directory for directory in Path('.').iterdir() if directory.is_dir() and directory.name.isnumeric()]
    editions = []
    for yf_path in year_folders:
        year = int(yf_path.name)
        files = [f for f in yf_path.glob('**/*') if f.is_file()]
        files = list(filter(lambda f: f.suffix in ['.py', '.rs', '.go', '.c', '.cpp'], files))
        files = list(filter(is_valid_solution_file, files))

        solutions = [build_solution(year, f) for f in files]
        editions.append(
            Edition(yf_path, year, sorted(solutions, key=lambda c: c.day))
        )

    return sorted(editions, key=lambda e: e.year, reverse=True)


def main():
    global cached
    cached = load_cache()

    editions = build_editions()
    for edition in editions:
        rendered = load_template('./templates/template.md.jinja').render(
            year=edition.year,
            solved_challs=edition.challenges,
        )

        save_file(edition.dir / "README.md", rendered)
        print(f"Readme {edition.year} generated!")

    rendered = load_template('./templates/template_main.md.jinja').render(
        editions=editions,
    )

    save_file("README.md", rendered)
    print(f"Main Readme generated!")
    save_cache(cached)



if __name__ == '__main__':
    main()
