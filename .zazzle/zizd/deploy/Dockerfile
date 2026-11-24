# The `deploy` step of for-fucks-sake's zizd pipeline (deploys: true), in the `release` environment only: deploys the
# game and the Worker the `build` step made to https://forfuckssake.today, from /zazzle/input/artifacts/build. It never
# rebuilds. Needs CLOUDFLARE_API_TOKEN. wrangler is release.yml's version.
FROM node:22.21.1-bookworm-slim
RUN npm install -g --no-audit --no-fund wrangler@4.156.0 && npm cache clean --force
COPY run.sh /run.sh
CMD ["sh", "/run.sh"]
