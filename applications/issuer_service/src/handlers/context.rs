// Copyright 2024 The Tari Project
// SPDX-License-Identifier: BSD-3-Clause
//   Copyright 2023 The Tari Project
//   SPDX-License-Identifier: BSD-3-Clause

use crate::handlers::auth::Keys;
use stable_coin_store::SqliteStore;

#[derive(Clone)]
pub struct HandlerContext {
    store: SqliteStore,
    jwt_secret: Keys,
}

impl HandlerContext {
    pub fn new(store: SqliteStore, jwt_secret: String) -> Self {
        Self {
            store,
            jwt_secret: Keys::new(jwt_secret.as_bytes()),
        }
    }

    pub fn store(&self) -> &SqliteStore {
        &self.store
    }

    pub fn jwt_secret(&self) -> &Keys {
        &self.jwt_secret
    }
}
