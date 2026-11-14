# MVP non-functional requirements

Full list: `../non-functional-requirements.md`. The MVP keeps only what protects the fun and the ability to ship.

| Area | Requirement | Measured by |
| --- | --- | --- |
| Run length | 8-15 minutes median | playtests + small bot |
| Determinism | seed + commands = same state hash, in browser and CLI | replay corpus (>= 10 runs) in CI |
| Fit | no scrollbars at 1440x790; readable message when too small | one Playwright test |
| Keyboard | whole game playable without a mouse | Playwright happy path, keyboard only |
| Privacy | nothing leaves the browser; no cookies, no analytics | review + CSP |
| Hosting cost | ~$0 (static site) | n/a |
| Browsers | latest Chromium and Firefox; Safari best-effort | manual check, Playwright on Chromium |
| Accessibility | reduced-motion honoured, ARIA live region for toasts | quick manual check |
| Safety | no real people/companies, no violence, no actionable crime | curation checklist |

Cut for now: GM latency and download size, load test, render-time budget, localisation, accessibility audit, multi-browser CI matrix.
