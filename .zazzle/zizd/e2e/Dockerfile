# The `e2e` step of for-fucks-sake's zizd pipeline: the keyboard-only Playwright tests in Chromium (the end of the `web`
# job of .github/workflows/ci.yml), against the engine the `build` step made. The image is the Playwright of
# web/package-lock.json (1.65.1), with its browsers and Node.
FROM mcr.microsoft.com/playwright:v1.65.1-noble
COPY run.sh /run.sh
CMD ["sh", "/run.sh"]
