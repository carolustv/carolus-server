[![Build Status](https://travis-ci.org/carolustv/carolus-server.svg?branch=master)](https://travis-ci.org/carolustv/carolus-server)

# Carolus

Currently only runs with nightly.

Quick start:

```bash
echo 'CAROLUS_MOVIES_PATH="/my/movies/path"' > .env
cargo run &
curl http://localhost:3000/api/movies
```

Currently only supports mp4 files.

## License

This Source Code Form is subject to the terms of the Mozilla Public
License, v. 2.0. If a copy of the MPL was not distributed with this
file, You can obtain one at http://mozilla.org/MPL/2.0/.
