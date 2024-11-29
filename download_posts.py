#!/usr/bin/env python3

import os
import requests
import re
import html

# Define the subreddit and URL
url = 'https://old.reddit.com/r/dailyprogrammer/.json'

# Set up headers to mimic a browser request
headers = {'User-Agent': 'Mozilla/5.0 (X11; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0'}

# The directory name in which posts are to be saved
dir_name = 'posts'

def main():
    # Create the directory if it doesn't exist
    os.makedirs(dir_name, exist_ok=True)

    after = None
    while True:
        # Set up parameters for pagination
        params = {'after': after} if after else {}
        response = requests.get(url, headers=headers, params=params)
        
        # Check if the request was successful
        if response.status_code != 200:
            print(f"Failed to retrieve posts: {response.status_code}")
            break

        # Parse the JSON response
        data = response.json()['data']
        posts = data['children']
        
        # Break the loop if there are no more posts
        if not posts:
            break

        for post in posts:
            post_data = post['data']
            post_id = post_data['id']
            post_title = post_data['title']
            post_url = post_data['url']
            post_body = post_data['selftext']
            post_permalink = post_data['permalink'].split('/')[-2]

            print(f'Downloading post: {post_title}')

            # Create a file for each post
            post_filename = f'{post_permalink}-{post_id}.md'
            with open(f'{dir_name}/{post_filename}', 'w', encoding='utf-8') as file:
                # Write post metadata and content to the file
                file.write(f"---\n")
                file.write(f"title: \"{post_title}\"\n")
                file.write(f"url: \"{post_url}\"\n")
                file.write(f"---\n\n")
                file.write(html.unescape(post_body))

        # Update the 'after' parameter for the next page of posts
        after = data['after']
        if not after:
            break

    print("Posts downloaded successfully.")

if __name__ == '__main__':
    main()
