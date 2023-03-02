export $(grep -v '^#' ../.env.url.local | xargs)
npm run serve