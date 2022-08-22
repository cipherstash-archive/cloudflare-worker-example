# Cloudflare Worker + CipherStash Demo

This repo contains a Cloudflare Worker written in Rust that uses CipherStash as a data-store.

## About CipherStash

[CipherStash](https://cipherstash.com) is a Queryable Encrypted Datastore and implements a form of QuALE (Queryable Application Level Encryption).
It has clients in [JavaScript](https://github.com/cipherstash/cipherstash.js) and
[Ruby](https://github.com/cipherstash/ruby-client)/[ActiveRecord](https://github.com/cipherstash/activestash) with
a WebAssembly compilable Rust SDK in preview.

## Why?

Encryption is a powerful technique to protect sensitive data.
However, encrypting data at the application level using traditional techniques means that queries over that
data can no longer be performed.
CipherStash's Queryable encryption technology means that data can be protected using encryption but remain queryable.

Our philosophy is to encrypt the data "as close to the user's eyeballs as possible".
Running CipherStash in a Cloudflare worker get's the encryption step closer than ever before.
The next stage would be to encrypt in the browser but then key management becomes more challenging
so we believe performing encryption at the edge is a good option (at least until eyeball ciphers are a thing!).

## How it works

CipherStash performs encryption and _vectorisation_ (the process of generating indexable encrypted vectors)
at the client.
In this case, the worker is the client.

![Architecture of the worker](/assets/arch.png)

### Encryption Key

An encryption key is managed using a [worker
secret](https://developers.cloudflare.com/workers/wrangler/commands/#secret).

### Authentication

Authentication isn't managed by the worker.
Instead, the client application (say a web-browser) must get a JSON Web Token (JWT)
and include it in any requests to the worker (via a Bearer Token).
The worker passes the JWT through to the CipherStash data-service which validates the token and scopes.

## Running the Demo

### Prerequisites

* Rust 1.62.0 via [Rustup](https://rustup.rs/) (earlier versions may work but haven't been tested)
* [jq](https://stedolan.github.io/jq/)
* Node.js 16+ (used for the CipherStash CLI tool)
* curl (available on most systems)
* [Wrangler](https://developers.cloudflare.com/workers/wrangler/get-started/#install)
* [Cloudflare account](https://dash.cloudflare.com/sign-up)

### Get a CipherStash Workspace

To store data using CipherStash, you'll need a workspace.
You can get one by [creating an account](https://cipherstash.com/signup/start?type=activestash).

Creating an account gets you a workspace (or you can create a new one).
Take a note of the workspace ID - you'll need that shortly.

![Workspace Screenshot](/assets/workspace.png)

### Install the Stash CLI

```sh
npm i -g @cipherstash/stash-cli
```

[More info here.](https://docs.cipherstash.com/reference/stash-cli/stash-install-cli.html#step-1-install-dependencies).

### Login to your workspace

Use the stash CLI to log into the workspace you created above.

```
stash login --workspace <WORKSPACE-ID>
```
*Note: Check the Region is correct*

If your workspace is not in ap-southeast-2 you will need to change the `CIPHERSTASH_HOST` value in `wrangler.toml`.
It's of the form `https://<region>.aws.stashdata.net`.
Note that only `ap-southeast-2` and `us-east-1` are currently available.

### Create a Collection

Records in CipherStash are stored in collections which can also have indexes defined.
The easiest way to create a collection is with the stash CLI.
We've included a collection schema JSON file in the demo.

```
stash create-collection users --schema users.schema.json
```

Here's the full [schema reference](https://docs.cipherstash.com/reference/schema-definition.html).

### Add the Collection Schema to the worker

We now need to tell the worker how to use the collection we've created:

```
stash export-schema users > users.annotated.json
```

*Note: The worker looks for a file called `users.annotated.json` so make sure you use this file name*

The annotated schema includes the IDs and keys used for indexing.
Normally these are stored encrypted in the CipherStash data-service, but the Rust SDK currently requires these as config. This step will be removed in a future version of the Rust SDK.

### Login to Wrangler

```
wrangler login
```

[More info here.](https://developers.cloudflare.com/workers/wrangler/get-started/#authenticate)

### Create a Cloudflare worker

1. Go to the [Cloudflare dashboard](https://dash.cloudflare.com/).
2. Select `Workers` from the left side menu.
3. If this is your first time creating a worker, you will be prompted to create a subdomain.
4. Follow the prompts to create a subdomain if applicable.
5. Within `Workers`, click `Create Service`.
6. Name the service as `cipherstash-demo`.
7. Select `HTTP Router`.
8. Click `Create service`.

### Set an Encryption Key

The key must be a 32-byte, hex encoded, cryptographically strong value.
You can use `node` to generate one for you.

Back in your terminal, run the below:

```
node -e "let { randomBytes } = require('crypto'); console.log(randomBytes(32).toString('hex'))" \
| wrangler secret put CIPHERSTASH_KEY
```

### Publish the demo

You should be ready to run the demo!

```
wrangler publish
```

This will start the worker and make it available on your workers domain.

Since the project is called `cipherstash-demo` the worker should be available at `https://cipherstash-demo.<your subdomain>.workers.dev`.

Note: it's possible to access the worker without publishing by calling `wrangler dev`.
However, due to execution limits on the free tier some of our example scripts won't complete when run against the local worker.

### Load some data

To load some data into your collection via the worker:

```
./bulk.sh https://cipherstash-demo.<your subdomain>.workers.dev
```

The bulk script inserts each record individually so it can take a couple minutes to complete.

### Run Some Queries

The `run.sh` script is used to fetch a record and run some queries.
It provides a few examples but feel free to change it and experiment:

```
./run.sh https://cipherstash-demo.<your subdomain>.workers.dev
```

## Issues

If you have any issues, please [open an issue](https://github.com/cipherstash/cloudflare-worker-example/issues/new).
You can also get help on the [CipherStash Support Forum](https://discuss.cipherstash.com/).

