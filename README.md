# Rust-React-Docker

Monorepo for full-stack development using Rust+React+Docker

## Using
### Backend (Rust)
Actix-Web
Diesel

### Frontend (React)
Pnpm
Gatsby

## Setup

Make you have Docker Compose V2 installed (details [here](https://www.docker.com/blog/announcing-compose-v2-general-availability/)).

Copy the `.env.sample` file to `.env`:

```shell
$ cp .env.sample .env
```

Create a volume named `example_postgres`:

```shell
$ docker volume create --name=example_postgres
```

Bring up the containers using Compose:

```shell
$ docker compose up -d
```

You will also need to add entries to your hosts file for `app-dev.example.com` and
`api-dev.example.com`:

```
127.0.0.1     app-dev.example.com
127.0.0.1     api-dev.example.com
```

Then start coding! The frontend is available at at [app-dev.example.com](https://app-dev.example.com)
and the backend is available at [api-dev.example.com](https://api-dev.example.com).

## Migrations

To generate a new migration, run the following to open a shell inside the `backend` container:

```shell
$ docker compose exec backend /bin/bash
```

Once you are inside the container, you can generate a new migration using:

```shell
$ diesel migration generate <migration_name>
```

This will create a directory in the `backend/migrations/` folder named with the
timestamp and the migration name provided. Inside this folder will be two files,
`up.sql` and `down.sql`. Edit both of these files to provide the migration (`up.sql`)
and rollback (`down.sql`) logic for Diesel.

> **Note:** Because Docker runs as root, migration files will be owned by root.
> You will need to take ownership of them using:
> ```shell
> $ sudo chown -R $USER:$USER backend/migrations
> ```

Once you have written the migration files, you can apply your migration using
the following:

```shell
$ diesel migration run
```

It is also a good idea to test your rollback logic, with the `redo` option:

```shell
$ diesel migration redo
```

## Compile Times

Rust compile times were the biggest issue in this project. To that end, the
backend container uses `sccache` to cache build artifacts, backed by the
`memcached` container that is included in the `docker-compose.yml` file.

This should all work out-of-the-box. If you encounter issues with `rust-analyzer`
failing, this is usually caused by the backend container and `rust-analyzer`
fighting over who owns the `backend/target` folder (spoiler alert: Docker wins).
You can update the target directory for `rust-analyzer` using the config options
for whatever LSP is using it. Example for `coc-nvim`:

```json
{
  ...,
  "rust-analyzer.check.extraArgs": ["--target-dir", "/tmp/rust-analyzer-checks"]
}
```
