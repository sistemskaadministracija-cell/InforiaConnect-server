# InforiaConnect Server Rules

## Scope

* This repository contains the custom InforiaConnect hbbs/hbbr server image.
* Preserve compatibility with the deployed 1.1.15 server and client protocol.
* The production data directory is `/home/inforiaadmin/rustdesk` on the VM and
  is mounted at `/root` in both containers.
* Never commit the server private key, database, or production blocklist.

## Delivery

* GitHub Actions is the authoritative build environment.
* Manually triggered workflows must use a descriptive `run-name`.
* Every repository change must be documented in `logging/revisions.txt`
  before commit and push.
* Confirm the workflow and published image before providing deployment steps.
* Preserve host networking, restart policy `always`, and the shared data mount.

## Editing

* Keep changes minimal and compatible with upstream RustDesk Server.
* Avoid `unwrap()` and `expect()` in production code.
* Do not add dependencies unless required.
* Do not change firewall, router, DNS, or VM network settings automatically.
