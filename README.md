# Noumead 🏝️ (WIP)

I always forget the number of arguments to pass to a Nomad parameterized job. This CLI allow to dispatch a parameterized job to Nomad and follow the log of the dispatched parameterized job.

## Usage

For the time being, no release has been done. In order to run the CLI please clone this repo and run one of the following command. By default Noumead will look for the `NOMAD_ADDR` & `NOMAD_TOKEN` environment variable

### Only dispatch

```sh
cargo run --  dispatch
```

### Dispatch and follow

```sh
cargo run --  dispatch --follow
```

### Passing var

You can pass the nomad server address & token with this command

```sh
cargo run -- --nomad-url="<url>" --token="<token>" dispatch --follow
```
