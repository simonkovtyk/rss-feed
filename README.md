<div align="center">

  <img width="64" height="64" src="./docs/logo.svg" />

  # Feeder
  A fast, reliable, and robust RSS feed synchronizer written in Rust. Keep your feeds up-to-date effortlessly, with minimal resource usage, maximum performance, and built-in support for Discord webhooks.

  [![Last Release Badge](https://img.shields.io/github/v/release/simonkovtyk/feeder?sort=semver&display_name=release&color=7300ff&labelColor=27272a)](../../releases/latest)
  [![GitHub License Badge](https://img.shields.io/github/license/simonkovtyk/feeder?color=7300ff&labelColor=27272a)](./LICENSE)
  [![Contributions Welcomed Badge](https://img.shields.io/badge/contributions-welcomed-7300ff?labelColor=27272a)](#contributing)
  [![Sponsor Hint Badge](https://img.shields.io/badge/❤️-Sponsor_it-%23dc2626?style=flat&labelColor=27272a)](https://github.com/sponsors/simonkovtyk/)

</div>

## Introduction
This Rust-based RSS synchronizer offers fast, reliable, and standards-compliant feed aggregation with full configurability, logging, and runtime safety.

✅ **Efficient RSS Handling:** Fully respects RSS standards, including proper TTL (time-to-live) handling to ensure feeds are refreshed at appropriate intervals.  
✅ **Bandwidth Optimization:** Supports HTTP 304 (Not Modified) responses to avoid unnecessary downloads of unchanged feeds.  
✅ **Extensive Logging:** Tracks feed updates, errors, and network activity to provide full transparency and easier debugging.  
✅ **Highly Configurable:** Customize fetch intervals, output formats and more to fit your workflow.  
✅ **Performance-Oriented:** Designed for speed and minimal resource usage without compromising correctness.  
✅ **Error Resilience:** Robust handling of network failures, malformed feeds, and other unexpected issues.

## Integrations
Feeder can be easily integrated into your workflows and applications.

✅ **Discord:** Configure and call webhooks with your personal styled messages (no branding is done by me).

*Currently only Discord is supported, but I am working on Telegram. If you are still missing a integration, [consider creating a feature-request or pull-request](#contributing)*

## Installation

First, you need a build either from the [releases](../../releases/latest) or by building it from source.

A PostgreSQL Database is also required.

Feeder will take care of the database and creates everything in there for you, so no maintainance needed.

## Usage
### Configuration
Feeder is configured by using a simple json-file.

The configuration is on a per RSS-Channel basis, to provide more flexibility for each RSS-Channel.

```json5
{
  "db": "<<<YOUR POSTGRESQL CONNECTION STRING HERE>>",
  "rss": [
    {
      "url": "http://newsfeed.zeit.de/index",
      "discord": {
        "webhook": {
          "id": 1234, // YOUR WEBHOOK ID
          "token": "your-token" // YOUR WEBHOOK TOKEN
        },
        "style": {
          "embed": "7300ff" // CUSTOM EMBED STYLING (IT'S THE LEFT-SIDE BORDER OF THE EMBED)
        }
      }
    },
    // ... add as many RSS-Channels as you want
  ]
}
```
### Starting
To start Feeder with your configuration from the step before, simply run:
```shell
./feeder --config path-to-your-config.json
```

You could also improve the uptime by creating a [systemd-Unit](https://wiki.archlinux.org/title/Systemd).

## Roadmap
- [x] Add Discord integration
- [ ] Add Telegram integration
- [ ] Improve docs to cover edge-cases like overriding RSS-Channel ttl's and the fallback ttl
- [ ] Provide a Docker solution

## License
The MIT License (MIT) - Please have a look at the [LICENSE file](./LICENSE) for more details.

## Contributing
Contributions are always welcome and greatly appreciated. Whether you want to report a bug, suggest a new feature, or improve the documentation, your input helps make the project better for everyone.

Feel free to submit a pull request, issue or feature request.

### Issues and Feature Requests
Reporting an issue or creating a feature request is made by creating a new issue on this repository.

You can create a [new issue or feature request here](../../issues/new/choose).

### Pull Requests
GitHub offers a solid guideline for contributing to open source projects through pull requests, covering key practices. These best practices provide a reliable starting point for making effective contributions.

You can find the [guidelines here](https://docs.github.com/get-started/exploring-projects-on-github/contributing-to-a-project).

### Code Of Conduct
We are committed to keeping a welcoming, inclusive, and respectful community for everyone. To help us achieve this, we kindly ask that you adhere to our [Code of Conduct](./CODE_OF_CONDUCT.md).

## Legal

All trademarks and registered trademarks mentioned are property of their respective owners and are used for identification purposes only. Use of these names does not imply endorsement or affiliation.

This project is a trademark of Simon Kovtyk. The License does not grant rights to use the trademark without permission.

© 2025 — present by Maintainers & Simon Kovtyk
