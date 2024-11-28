#!/usr/bin/env python3

import os
import requests
import re
import html

# Define the subreddit and URL
url = 'https://www.reddit.com/r/dailyprogrammer/.json'

# Set up headers to mimic a browser request
headers = {'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0'}

# Create a directory to save the posts
dir_name = 'posts'

def main():
    os.makedirs(dir_name, exist_ok=True)

    # Download posts
    after = None
    while True:
        params = {'after': after} if after else {}
        response = requests.get(url, headers=headers, params=params)
        if response.status_code != 200:
            print(f"Failed to retrieve posts: {response.status_code}")
            break

        data = response.json()['data']
        posts = data['children']
        if not posts:
            break

        for post in posts:
            post_data = post['data']
            post_id = post_data['id']
            post_title = post_data['title']
            post_body = post_data['selftext']

            print(f'Downloading post: {post_title}')

            # Create a file for each post
            post_filename = f'{post_title}-{post_id}.txt'
            with open(f'{dir_name}/{sanitise_filename(post_filename)}', 'w', encoding='utf-8') as file:
                file.write(f"Title: {post_title}\n\n")
                file.write(html.unescape(post_body))

        after = data['after']
        if not after:
            break

    print("Posts downloaded successfully.")

def sanitise_filename(filename):
    sanitised = html.unescape(filename)
    sanitised = re.sub(r'\d+/\d+/\d+', lambda x: x.group(0).replace('/', '-'), sanitised)
    sanitised = re.sub(r'[\\/:*?"<>|]', '_', sanitised)
    return sanitised

if __name__ == '__main__':
    main()
