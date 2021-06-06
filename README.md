# Web chess

## Installation

**TO DO**

## Deploy on heroku
```bash
heroku create --buildpack emk/rust
heroku buildpacks:add --index 1 heroku/nodejs
heroku config:set NPM_CONFIG_PRODUCTION=false
```
Then push a branch on heroku
