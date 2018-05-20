include!(concat!(env!("OUT_DIR"), "/constructors.rs"));

use std::io::Cursor;
use serialize::Serialize;
use serialize::SerializeVector;
use std::io;
include!(concat!(env!("OUT_DIR"), "/constructors_serialize.rs"));

/*
//rpc_result#f35c6d01 req_msg_id:long result:Object = RpcResult;
struct RpcResult {
    req_msg_id: i64,
    result: TlObject,
}

//future_salt#0949d9dc valid_since:int valid_until:int salt:long = FutureSalt;
struct FutureSalt {
    valid_since: i32,
    valid_until: i32,
    salt: i64,
}

//future_salts#ae500895 req_msg_id:long now:int salts:vector<future_salt> = FutureSalts;
struct FutureSalts {
    req_msg_id: i64,
    now: i32,
    salts: Vec<FutureSalt>, // No Id
}


//message msg_id:long seqno:int bytes:int body:Object = Message;
//msg_container#73f1f8dc messages:vector<message> = MessageContainer;
//msg_copy#e06046b2 orig_message:Message = MessageCopy;
//gzip_packed#3072cfa1 packed_data:string = Object;
//
//ipPort ipv4:int port:int = IpPort;
//help.configSimple#d997c3c5 date:int expires:int dc_id:int ip_port_list:Vector<ipPort> = help.ConfigSimple


//get_future_salts#b921bd04 num:int = FutureSalts;

// invokeAfterMsg#cb9f372d {X:Type} msg_id:long query:!X = X;
// invokeAfterMsgs#3dc4b4f0 {X:Type} msg_ids:Vector<long> query:!X = X;
// initConnection#c7481da6 {X:Type} api_id:int device_model:string system_version:string app_version:string system_lang_code:string lang_pack:string lang_code:string query:!X = X;
// invokeWithLayer#da9b0d0d {X:Type} layer:int query:!X = X;
// invokeWithoutUpdates#bf9459b7 {X:Type} query:!X = X;
*/


