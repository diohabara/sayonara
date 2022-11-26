# sayonara

Twitter Bot to delete your tweets periodically

## how to run

```bash
RUST_LOG=info cargo r
```

## .env

Fill in the following `???` fields.

```text
TWITTER_CONSUMER_KEY=???
TWITTER_CONSUMER_SECRET=???
TWITTER_ACCESS_TOKEN=???
TWITTER_ACCESS_TOKEN_SECRET=???
TWITTER_ACCOUNT_NAME=@???
```

Execute the command

```bash
gh secret set -f .env
```
