// Copyright 2015-2017 Intecture Developers. See the COPYRIGHT file at the
// top-level directory of this distribution and at
// https://intecture.io/COPYRIGHT.
//
// Licensed under the Mozilla Public License 2.0 <LICENSE or
// https://www.tldrlegal.com/l/mpl-2.0>. This file may not be copied,
// modified, or distributed except according to those terms.

use czmq::{ZMsg, ZSock};
use error::Result;
use inapi::Host;
use zdaemon::ZMsgExtended;

pub struct TelemetryApi;

impl TelemetryApi {
    pub fn get(sock: &mut ZSock, host: &mut Host, router_id: &[u8]) -> Result<()> {
        let json = host.data().to_string();
        let msg = ZMsg::new_ok()?;
        msg.pushstr("")?;
        msg.pushbytes(router_id)?;
        msg.addstr(&json)?;
        msg.send(sock)?;
        Ok(())
    }
}
