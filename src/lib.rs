// boolFalse#bc799737 = Bool;
// boolTrue#997275b5 = Bool;

// true#3fedd339 = True;

// vector#1cb5c415 {t:Type} # [ t ] = Vector t;

// error#c4b9f9bb code:int text:string = Error;

// null#56730bcc = Null;

// inputPeerEmpty#7f3b18ea = InputPeer;
// inputPeerSelf#7da07ec9 = InputPeer;
// inputPeerChat#179be863 chat_id:int = InputPeer;
// inputPeerUser#7b8e7de6 user_id:int access_hash:long = InputPeer;
// inputPeerChannel#20adaef8 channel_id:int access_hash:long = InputPeer;

// long = i64
// int  = i32

enum Bool {
    False, // #bc799737
    True, // #997275b5
}

struct True; // #3fedd339

// vector#1cb5c415 {t:Type} # [ t ] = Vector t;

struct Null; // #56730bcc

enum InputPeer {
    Empty, // #7f3b18ea
    PeerSelf, // #7da07ec9
    Chat { // #179be863
        chat_id: i32,
    },
    User { // #7b8e7de6
        user_id: i32,
        access_hash: i64,
    },
    Channel { // #20adaef8
        channel_id: i32,
        access_hash: i64,
    },
}

// get serde and make a custom thing

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
