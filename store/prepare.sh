#!/bin/env bash
# Copyright 2024 The Tari Project
# SPDX-License-Identifier: BSD-3-Clause

set -e

rm -fr `pwd`/data

export DATABASE_URL=sqlite:///`pwd`/data/__tmp_store.db
mkdir -p `pwd`/data

sqlx database create
sqlx migrate run
cargo sqlx prepare
