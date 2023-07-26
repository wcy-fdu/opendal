// Licensed to the Apache Software Foundation (ASF) under one
// or more contributor license agreements.  See the NOTICE file
// distributed with this work for additional information
// regarding copyright ownership.  The ASF licenses this file
// to you under the Apache License, Version 2.0 (the
// "License"); you may not use this file except in compliance
// with the License.  You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing,
// software distributed under the License is distributed on an
// "AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
// KIND, either express or implied.  See the License for the
// specific language governing permissions and limitations
// under the License.

use std::sync::Arc;

use async_trait::async_trait;
use bytes::Bytes;
use http::StatusCode;

use super::core::AzblobCore;
use super::error::parse_error;
use crate::raw::*;
use crate::*;

pub struct AzblobWriter {
    core: Arc<AzblobCore>,

    op: OpWrite,
    path: String,
    block_id: Option<String>,
    buffer: oio::VectorCursor,
}

impl AzblobWriter {
    pub fn new(core: Arc<AzblobCore>, op: OpWrite, path: String) -> Self {
        AzblobWriter {
            core,
            op,
            path,
            block_id: None,
            buffer: oio::VectorCursor::new(),
        }
    }

    async fn write_oneshot(&self, size: u64, body: AsyncBody) -> Result<()> {
        let mut req = self.core.azblob_put_blob_request(
            &self.path,
            Some(size),
            self.op.content_type(),
            self.op.cache_control(),
            body,
        )?;

        self.core.sign(&mut req).await?;

        let resp = self.core.send(req).await?;

        let status = resp.status();

        match status {
            StatusCode::CREATED | StatusCode::OK => {
                resp.into_body().consume().await?;
                Ok(())
            }
            _ => Err(parse_error(resp).await?),
        }
    }
}

#[async_trait]
impl oio::Write for AzblobWriter {
    async fn write(&mut self, bs: Bytes) -> Result<()> {
        self.write_oneshot(bs.len() as u64, AsyncBody::Bytes(bs))
            .await
    }

    async fn sink(&mut self, size: u64, s: oio::Streamer) -> Result<()> {
        self.write_oneshot(size, AsyncBody::Stream(s)).await
    }

    async fn abort(&mut self) -> Result<()> {
        Ok(())
    }

    async fn close(&mut self) -> Result<()> {
        Ok(())
    }
}
