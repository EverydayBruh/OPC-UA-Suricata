/* Copyright (C) 2024 Open Information Security Foundation
 *
 * You can copy, redistribute or modify this Program under the terms of
 * the GNU General Public License version 2 as published by the Free
 * Software Foundation.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * version 2 along with this program; if not, write to the Free Software
 * Foundation, Inc., 51 Franklin Street, Fifth Floor, Boston, MA
 * 02110-1301, USA.
 */

use super::opcua::{OpcuaTransaction, ALPROTO_OPCUA};
use crate::core::Direction;
use crate::detect::{
    DetectBufferSetActiveList, DetectHelperBufferMpmRegister, DetectHelperGetData,
    DetectHelperKeywordRegister, DetectSignatureSetAppProto, SCSigTableElmt,
    SIGMATCH_INFO_STICKY_BUFFER, SIGMATCH_NOOPT,
};
use std::os::raw::{c_int, c_void};

static mut G_OPCUA_OPCUABUF_BUFFER_ID: c_int = 0;

unsafe extern "C" fn opcua_opcuabuf_setup(
    de: *mut c_void, s: *mut c_void, _raw: *const std::os::raw::c_char,
) -> c_int {
    if DetectSignatureSetAppProto(s, ALPROTO_OPCUA) != 0 {
        return -1;
    }
    if DetectBufferSetActiveList(de, s, G_OPCUA_OPCUABUF_BUFFER_ID) < 0 {
        return -1;
    }
    return 0;
}

/// Get the request/response buffer for a transaction from C.
unsafe extern "C" fn opcua_opcuabuf_get_data(
    tx: *const c_void, flags: u8, buf: *mut *const u8, len: *mut u32,
) -> bool {
    let tx = cast_pointer!(tx, OpcuaTransaction);
    if flags & Direction::ToClient as u8 != 0 {
        if let Some(ref response) = tx.response {
            *len = response.len() as u32;
            *buf = response.as_ptr();
            return true;
        }
    } else if let Some(ref request) = tx.request {
        *len = request.len() as u32;
        *buf = request.as_ptr();
        return true;
    }
    return false;
}

unsafe extern "C" fn opcua_opcuabuf_get(
    de: *mut c_void, transforms: *const c_void, flow: *const c_void, flow_flags: u8,
    tx: *const c_void, list_id: c_int,
) -> *mut c_void {
    return DetectHelperGetData(
        de,
        transforms,
        flow,
        flow_flags,
        tx,
        list_id,
        opcua_opcuabuf_get_data,
    );
}

#[no_mangle]
pub unsafe extern "C" fn ScDetectOpcuaRegister() {
    // TODO create a suricata-verify test
    // Setup a keyword structure and register it
    let kw = SCSigTableElmt {
        name: b"opcua.opcuabuf\0".as_ptr() as *const libc::c_char,
        desc: b"Opcua content modifier to match on the opcua buffer\0".as_ptr()
            as *const libc::c_char,
        // TODO use the right anchor for url and write doc
        url: b"/rules/opcua-keywords.html#buffer\0".as_ptr() as *const libc::c_char,
        Setup: opcua_opcuabuf_setup,
        flags: SIGMATCH_NOOPT | SIGMATCH_INFO_STICKY_BUFFER,
        AppLayerTxMatch: None,
        Free: None,
    };
    let _g_opcua_opcuabuf_kw_id = DetectHelperKeywordRegister(&kw);
    G_OPCUA_OPCUABUF_BUFFER_ID = DetectHelperBufferMpmRegister(
        b"opcua.opcuabuf\0".as_ptr() as *const libc::c_char,
        b"opcua.opcuabuf intern description\0".as_ptr() as *const libc::c_char,
        ALPROTO_OPCUA,
        true, //toclient
        true, //toserver
        opcua_opcuabuf_get,
    );
}
