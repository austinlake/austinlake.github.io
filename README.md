# website

[austinlake04.com](https://austinlake04.com)

[![Website Status][website-badge]][website-url]
[![Latest Release][release-badge]][release-url]
[![License][license-badge]](LICENSE)
[![CI Status][ci-badge]][ci-url]

[website-badge]: https://img.shields.io/website/https/austinlake04.com.svg
[website-url]: https://austinlake04.com

[release-badge]: https://img.shields.io/github/v/release/austinlake04/website
[release-url]: https://github.com/austinlake04/website/releases/latest

[license-badge]: https://img.shields.io/github/license/austinlake04/website

[ci-badge]: https://github.com/austinlake04/website/actions/workflows/ci.yml/badge.svg
[ci-url]: https://github.com/austinlake04/website/actions

## Local Deployment

Part of the build process makes uses of the standalone TailwindCSS CLI executable. Follow [these instructions](https://tailwindcss.com/blog/standalone-cli) to install in locally.

Assuming that the TailwindCSS CLI has been installed to this repo's local directory, you can run the following to deploy the site:

`trunk serve --open`
