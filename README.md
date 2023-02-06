# HTW mensa telegram bot

A bot that scrapes the HTW mensa website and posts the menu to a telegram channel.
Scraping is done once a week and the menu is posted every day at 6am.

## Setup
- You can use cargo to install dependencies.
- You need to have a telegram bot token and a channel id.
- You need to have a postgres database running.

## Usage
The simplest way to run the bot is to use docker-compose.
```bash
docker-compose up -d
```

Fill out the required environment variables in the docker-compose.yml file and you are good to go.
The database migration is done automatically. 