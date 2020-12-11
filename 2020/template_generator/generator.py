import json
import datetime
import requests
import os
import re
import sys
import pickle
from jinja2 import Template
from bs4 import BeautifulSoup

now = datetime.datetime.now()
CURRENT_YEAR = now.year
CURRENT_YEAR_FOLDER = "./{}".format(CURRENT_YEAR)
CHALLENGES_DIR = '../'  # relative to generator
LINK_FORMAT = "https://adventofcode.com/{}/day/{{}}".format(CURRENT_YEAR)
title_extraction_regex = re.compile(r': (.*) -')


def load_cache():
    try:
        with open("./cache.bin", "rb") as f:
            return pickle.load(f)
    except:
        pass

    return {}


def save_cache(cache):
    try:
        with open("./cache.bin", "wb") as f:
            return pickle.dump(cache, f)
    except Exception as e:
        print(e)


def save_readme(data):
    with open("../README.md", "wt") as f:
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
    ALLOWED_EXTENSTIONS = ['.py']
    for fname in os.listdir('../{}'.format(chall_id)):
        if fname.endswith('.txt'):
            continue

        if any(fname.endswith(x) for x in ALLOWED_EXTENSTIONS):
            return fname


def download_data(chall_id):
    link = LINK_FORMAT.format(chall_id)
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
        './{}/{}'.format(chall_id, filename)
    )


def build_challenges(dir, cached):
    new_cache = {}
    challenge_folders = [name for name in os.listdir(dir) if os.path.isdir(os.path.join(dir, name))]
    challenge_folders = list(filter(lambda s: s.isnumeric(), challenge_folders))
    challenge_folders = sorted(list(map(int, challenge_folders)))
    out_challs = []
    for chall_id in challenge_folders:
        chall = cached.get(chall_id)
        if chall:
            out_challs.append(chall)
        else:
            chall = download_data(chall_id)
            out_challs.append(chall)

        new_cache[chall_id] = chall

    save_cache(new_cache)
    return out_challs


def main():
    cached = load_cache()
    template = load_template()

    challs = build_challenges(CHALLENGES_DIR, cached)

    rendered = template.render(
        year=CURRENT_YEAR,
        solved_challs=challs,
        last_updated=now.strftime("%d/%m/%Y, %H:%M:%S")
    )

    save_readme(rendered)
    print("Readme generated!")


if __name__ == '__main__':
    main()
