# Second State SSVM runtime for the Oasis platform

This repo contains the Second State SSVM runtime for the
[Oasis platform](https://github.com/oasislabs/oasis-core). The runtime implements
an Ethereum flavored WebAssembly(ewasm) execution environment defined [here](https://github.com/ewasm/design).

## How to build and run oasis-ssvm-runtime `make test-e2e`

Use second-state/oasis-ssvm-runtime (official oasis-ssvm-runtime will build fail because Cargo.lock outdated).

In the following instructions, the top-level directory is the directory
where the code has been checked out.

We use official docker image(`oasisprotocol/oasis-core-dev`) for developing and testing.

If you want to setup the environment manually, please refer to
[the instructions from oasis-core repository](https://github.com/oasislabs/oasis-core/blob/master/README.md).


### Get latest version of `oasis-ssvm-runtime`

```bash
> git clone --single-branch --branch master git@github.com:second-state/oasis-ssvm-runtime.git
```

### The working folder structure

Folder struct
```bash
./
└── oasis-ssvm-runtime
```

### Run docker instance to get the working environment

Inside official build environment
> command come from oasis-core/Makefile target `docker-shell`

```bash
> docker run -t -i \
  --name oasis-core \
  --security-opt apparmor:unconfined \
  --security-opt seccomp=unconfined \
  -v $(pwd):/code \
  -w /code \
  oasisprotocol/oasis-core-dev:master \
  bash
```

### Download pre-built oasis core tools

We need prepare the `oasis-core-tools` before run oasis-ssvm-runtime

```bash
(docker) cd /code/oasis-ssvm-runtime && make download-artifacts
```

### Building the runtime

To build everything required for running the runtime, simply execute in the
top-level directory:

```bash
(docker) cd /code/oasis-ssvm-runtime && make
```

### Running end-to-end tests

To run a local Oasis network "cluster," deploy the runtime, and run the test client, run:
Run `test-e2e`

```bash
(docker) cd /code/oasis-ssvm-runtime && make test-e2e
```

### Video: How to build and run tests

[![oasis-ssvm-runtime: How to build and run tests](https://asciinema.org/a/DLCfdb668JcXQMjxLyXQmDRaD.svg)](https://asciinema.org/a/DLCfdb668JcXQMjxLyXQmDRaD)
