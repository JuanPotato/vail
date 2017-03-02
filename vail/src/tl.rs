#[derive(Debug, TlType)]
enum Bool {
    #[tl_id = "bc799737"]
    False,

    #[tl_id = "997275b5"]
    True,
}

#[derive(Debug, TlType)]
#[tl_id = "3fedd339"]
struct True;

#[derive(Debug, TlType)]
#[tl_id = "c4b9f9bb"]
struct Error {
    code: i32,
    text: String,
}

#[derive(Debug, TlType)]
#[tl_id = "56730bcc"]
struct Null;

#[derive(Debug, TlType)]
enum InputPeer {
    #[tl_id = "7f3b18ea"]
    Empty,

    #[tl_id = "7da07ec9"]
    SSelf,

    #[tl_id = "179be863"]
    Chat {
        chat_id: i32,
    },

    #[tl_id = "7b8e7de6"]
    User {
        user_id: i32,
        access_hash: i64,
    },

    #[tl_id = "20adaef8"]
    Channel {
        channel_id: i32,
        access_hash: i64,
    },
}

#[derive(Debug, TlType)]
enum InputUser {
    #[tl_id = "b98886cf"]
    Empty,

    #[tl_id = "f7c1b13f"]
    SSelf,

    #[tl_id = "d8292816"]
    User {
        user_id: i32,
        access_hash: i64,
    },
}

#[derive(Debug, TlType)]
enum InputContact {
    #[tl_id = "f392b7f4"]
    Phone {
        client_id: i64,
        phone: String,
        first_name: String,
        last_name: String,
    },
}

#[derive(Debug, TlType)]
enum InputFile {
    #[tl_id = "f52ff27f"]
    File {
        id: i64,
        parts: i32,
        name: String,
        md5_checksum: String,
    },

    #[tl_id = "fa4f0bb5"]
    Big {
        id: i64,
        parts: i32,
        name: String,
    },
}

#[derive(Debug, TlType)]
enum InputMedia {
    #[tl_id = "9664f57f"]
    Empty,

    #[tl_id = "630c9af1"]
    UploadedPhoto {
        flags: u32,
        file: Box<InputFile>,
        caption: String,
        stickers: Vec<InputDocument>,
    },

    #[tl_id = "e9bfb4f3"]
    Photo {
        id: Box<InputPhoto>,
        caption: String,
    },

    #[tl_id = "f9c44144"]
    GeoPoint {
        geo_point: Box<InputGeoPoint>,
    },

    #[tl_id = "a6e45987"]
    Contact {
        phone_number: String,
        first_name: String,
        last_name: String,
    },

    #[tl_id = "d070f1e9"]
    UploadedDocument {
        flags: u32,
        file: Box<InputFile>,
        mime_type: String,
        attributes: Vec<DocumentAttribute>,
        caption: String,
        stickers: Vec<InputDocument>,
    },

    #[tl_id = "50d88cae"]
    UploadedThumbDocument {
        flags: u32,
        file: Box<InputFile>,
        thumb: Box<InputFile>,
        mime_type: String,
        attributes: Vec<DocumentAttribute>,
        caption: String,
        stickers: Vec<InputDocument>,
    },

    #[tl_id = "1a77f29c"]
    Document {
        id: Box<InputDocument>,
        caption: String,
    },

    #[tl_id = "2827a81a"]
    Venue {
        geo_point: Box<InputGeoPoint>,
        title: String,
        address: String,
        provider: String,
        venue_id: String,
    },

    #[tl_id = "4843b0fd"]
    GifExternal {
        url: String,
        q: String,
    },

    #[tl_id = "b55f4f18"]
    PhotoExternal {
        url: String,
        caption: String,
    },

    #[tl_id = "e5e9607c"]
    DocumentExternal {
        url: String,
        caption: String,
    },

    #[tl_id = "d33f43f3"]
    Game {
        id: Box<InputGame>,
    },
}

#[derive(Debug, TlType)]
enum InputChatPhoto {
    #[tl_id = "1ca48f57"]
    Empty,

    #[tl_id = "927c55b4"]
    Uploaded {
        file: Box<InputFile>,
    },

    #[tl_id = "8953ad37"]
    Photo {
        id: Box<InputPhoto>,
    },
}

#[derive(Debug, TlType)]
enum InputGeoPoint {
    #[tl_id = "e4c123d6"]
    Empty,

    #[tl_id = "f3b7acc9"]
    Point {
        lat: f64,
        long: f64,
    },
}

#[derive(Debug, TlType)]
enum InputPhoto {
    #[tl_id = "1cd7bf0d"]
    Empty,

    #[tl_id = "fb95c6c4"]
    Photo {
        id: i64,
        access_hash: i64,
    },
}

#[derive(Debug, TlType)]
enum InputFileLocation {
    #[tl_id = "14637196"]
    Location {
        volume_id: i64,
        local_id: i32,
        secret: i64,
    },

    #[tl_id = "f5235d55"]
    Encrypted {
        id: i64,
        access_hash: i64,
    },

    #[tl_id = "430f0724"]
    Document {
        id: i64,
        access_hash: i64,
        version: i32,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "770656a8"]
struct InputAppEvent {
    time: f64,
    ttype: String,
    peer: i64,
    data: String,
}

#[derive(Debug, TlType)]
enum Peer {
    #[tl_id = "9db1bc6d"]
    User {
        user_id: i32,
    },

    #[tl_id = "bad0e5bb"]
    Chat {
        chat_id: i32,
    },

    #[tl_id = "bddde532"]
    Channel {
        channel_id: i32,
    },
}

#[derive(Debug, TlType)]
enum StorageFileType {
    #[tl_id = "aa963b05"]
    Unknown,

    #[tl_id = "7efe0e"]
    Jpeg,

    #[tl_id = "cae1aadf"]
    Gif,

    #[tl_id = "a4f63c0"]
    Png,

    #[tl_id = "ae1e508d"]
    Pdf,

    #[tl_id = "528a0677"]
    Mp3,

    #[tl_id = "4b09ebbc"]
    Mov,

    #[tl_id = "40bc6f52"]
    Partial,

    #[tl_id = "b3cea0e4"]
    Mp4,

    #[tl_id = "1081464c"]
    Webp,
}

#[derive(Debug, TlType)]
enum FileLocation {
    #[tl_id = "7c596b46"]
    Unavailable {
        volume_id: i64,
        local_id: i32,
        secret: i64,
    },

    #[tl_id = "53d69076"]
    Location {
        dc_id: i32,
        volume_id: i64,
        local_id: i32,
        secret: i64,
    },
}

#[derive(Debug, TlType)]
enum User {
    #[tl_id = "200250ba"]
    Empty {
        id: i32,
    },

    #[tl_id = "d10d979a"]
    User {
        flags: u32,
        #[flag_bit = "10"]
        sself: Option<bool>,
        #[flag_bit = "11"]
        contact: Option<bool>,
        #[flag_bit = "12"]
        mutual_contact: Option<bool>,
        #[flag_bit = "13"]
        deleted: Option<bool>,
        #[flag_bit = "14"]
        bot: Option<bool>,
        #[flag_bit = "15"]
        bot_chat_history: Option<bool>,
        #[flag_bit = "16"]
        bot_nochats: Option<bool>,
        #[flag_bit = "17"]
        verified: Option<bool>,
        #[flag_bit = "18"]
        restricted: Option<bool>,
        #[flag_bit = "20"]
        min: Option<bool>,
        #[flag_bit = "21"]
        bot_inline_geo: Option<bool>,
        id: i32,
        access_hash: i64,
        #[flag_bit = "1"]
        first_name: Option<String>,
        #[flag_bit = "2"]
        last_name: Option<String>,
        #[flag_bit = "3"]
        username: Option<String>,
        #[flag_bit = "4"]
        phone: Option<String>,
        #[flag_bit = "5"]
        photo: Option<Box<UserProfilePhoto>>,
        #[flag_bit = "6"]
        status: Option<Box<UserStatus>>,
        #[flag_bit = "14"]
        bot_info_version: Option<i32>,
        #[flag_bit = "18"]
        restriction_reason: Option<String>,
        #[flag_bit = "19"]
        bot_inline_placeholder: Option<String>,
    },
}

#[derive(Debug, TlType)]
enum UserProfilePhoto {
    #[tl_id = "4f11bae1"]
    Empty,

    #[tl_id = "d559d8c8"]
    Photo {
        photo_id: i64,
        photo_small: Box<FileLocation>,
        photo_big: Box<FileLocation>,
    },
}

#[derive(Debug, TlType)]
enum UserStatus {
    #[tl_id = "9d05049"]
    Empty,

    #[tl_id = "edb93949"]
    Online {
        expires: i32,
    },

    #[tl_id = "8c703f"]
    Offline {
        was_online: i32,
    },

    #[tl_id = "e26f42f1"]
    Recently,

    #[tl_id = "7bf09fc"]
    LastWeek,

    #[tl_id = "77ebc742"]
    LastMonth,
}

#[derive(Debug, TlType)]
enum Chat {
    #[tl_id = "9ba2d800"]
    Empty {
        id: i32,
    },

    #[tl_id = "d91cdd54"]
    Chat {
        flags: u32,
        creator: bool,
        #[flag_bit = "1"]
        kicked: Option<bool>,
        #[flag_bit = "2"]
        left: Option<bool>,
        #[flag_bit = "3"]
        admins_enabled: Option<bool>,
        #[flag_bit = "4"]
        admin: Option<bool>,
        #[flag_bit = "5"]
        deactivated: Option<bool>,
        id: i32,
        title: String,
        photo: Box<ChatPhoto>,
        participants_count: i32,
        date: i32,
        version: i32,
        #[flag_bit = "6"]
        migrated_to: Option<Box<InputChannel>>,
    },

    #[tl_id = "7328bdb"]
    Forbidden {
        id: i32,
        title: String,
    },

    #[tl_id = "a14dca52"]
    Channel {
        flags: u32,
        creator: bool,
        #[flag_bit = "1"]
        kicked: Option<bool>,
        #[flag_bit = "2"]
        left: Option<bool>,
        #[flag_bit = "3"]
        editor: Option<bool>,
        #[flag_bit = "4"]
        moderator: Option<bool>,
        #[flag_bit = "5"]
        broadcast: Option<bool>,
        #[flag_bit = "7"]
        verified: Option<bool>,
        #[flag_bit = "8"]
        megagroup: Option<bool>,
        #[flag_bit = "9"]
        restricted: Option<bool>,
        #[flag_bit = "10"]
        democracy: Option<bool>,
        #[flag_bit = "11"]
        signatures: Option<bool>,
        #[flag_bit = "12"]
        min: Option<bool>,
        id: i32,
        #[flag_bit = "13"]
        access_hash: Option<i64>,
        title: String,
        #[flag_bit = "6"]
        username: Option<String>,
        photo: Box<ChatPhoto>,
        date: i32,
        version: i32,
        #[flag_bit = "9"]
        restriction_reason: Option<String>,
    },

    #[tl_id = "8537784f"]
    ChannelForbidden {
        flags: u32,
        #[flag_bit = "5"]
        broadcast: Option<bool>,
        #[flag_bit = "8"]
        megagroup: Option<bool>,
        id: i32,
        access_hash: i64,
        title: String,
    },
}

#[derive(Debug, TlType)]
enum ChatFull {
    #[tl_id = "2e02a614"]
    Full {
        id: i32,
        participants: Box<ChatParticipants>,
        chat_photo: Box<Photo>,
        notify_settings: Box<PeerNotifySettings>,
        exported_invite: Box<ExportedChatInvite>,
        bot_info: Vec<BotInfo>,
    },

    #[tl_id = "c3d5512f"]
    ChannelFull {
        flags: u32,
        #[flag_bit = "3"]
        can_view_participants: Option<bool>,
        #[flag_bit = "6"]
        can_set_username: Option<bool>,
        id: i32,
        about: String,
        participants_count: i32,
        #[flag_bit = "1"]
        admins_count: Option<i32>,
        #[flag_bit = "2"]
        kicked_count: Option<i32>,
        read_inbox_max_id: i32,
        read_outbox_max_id: i32,
        unread_count: i32,
        chat_photo: Box<Photo>,
        notify_settings: Box<PeerNotifySettings>,
        exported_invite: Box<ExportedChatInvite>,
        bot_info: Vec<BotInfo>,
        #[flag_bit = "4"]
        migrated_from_chat_id: Option<i32>,
        #[flag_bit = "4"]
        migrated_from_max_id: Option<i32>,
        #[flag_bit = "5"]
        pinned_msg_id: Option<i32>,
    },
}

#[derive(Debug, TlType)]
enum ChatParticipant {
    #[tl_id = "c8d7493e"]
    Participant {
        user_id: i32,
        inviter_id: i32,
        date: i32,
    },

    #[tl_id = "da13538a"]
    Creator {
        user_id: i32,
    },

    #[tl_id = "e2d6e436"]
    Admin {
        user_id: i32,
        inviter_id: i32,
        date: i32,
    },
}

#[derive(Debug, TlType)]
enum ChatParticipants {
    #[tl_id = "fc900c2b"]
    Forbidden {
        flags: u32,
        chat_id: i32,
        self_participant: Box<ChatParticipant>,
    },

    #[tl_id = "3f460fed"]
    Participants {
        chat_id: i32,
        participants: Vec<ChatParticipant>,
        version: i32,
    },
}

#[derive(Debug, TlType)]
enum ChatPhoto {
    #[tl_id = "37c1011c"]
    Empty,

    #[tl_id = "6153276a"]
    Photo {
        photo_small: Box<FileLocation>,
        photo_big: Box<FileLocation>,
    },
}

#[derive(Debug, TlType)]
enum Message {
    #[tl_id = "83e5de54"]
    Empty {
        id: i32,
    },

    #[tl_id = "c09be45f"]
    Message {
        flags: u32,
        #[flag_bit = "1"]
        out: Option<bool>,
        #[flag_bit = "4"]
        mentioned: Option<bool>,
        #[flag_bit = "5"]
        media_unread: Option<bool>,
        #[flag_bit = "13"]
        silent: Option<bool>,
        #[flag_bit = "14"]
        post: Option<bool>,
        id: i32,
        #[flag_bit = "8"]
        from_id: Option<i32>,
        to_id: Box<Peer>,
        #[flag_bit = "2"]
        fwd_from: Option<Box<MessageFwdHeader>>,
        #[flag_bit = "11"]
        via_bot_id: Option<i32>,
        #[flag_bit = "3"]
        reply_to_msg_id: Option<i32>,
        date: i32,
        message: String,
        #[flag_bit = "9"]
        media: Option<Box<MessageMedia>>,
        #[flag_bit = "6"]
        reply_markup: Option<Box<ReplyMarkup>>,
        #[flag_bit = "7"]
        entities: Option<Vec<MessageEntity>>,
        #[flag_bit = "10"]
        views: Option<i32>,
        #[flag_bit = "15"]
        edit_date: Option<i32>,
    },

    #[tl_id = "9e19a1f6"]
    Service {
        flags: u32,
        #[flag_bit = "1"]
        out: Option<bool>,
        #[flag_bit = "4"]
        mentioned: Option<bool>,
        #[flag_bit = "5"]
        media_unread: Option<bool>,
        #[flag_bit = "13"]
        silent: Option<bool>,
        #[flag_bit = "14"]
        post: Option<bool>,
        id: i32,
        #[flag_bit = "8"]
        from_id: Option<i32>,
        to_id: Box<Peer>,
        #[flag_bit = "3"]
        reply_to_msg_id: Option<i32>,
        date: i32,
        action: Box<MessageAction>,
    },
}

#[derive(Debug, TlType)]
enum MessageMedia {
    #[tl_id = "3ded6320"]
    Empty,

    #[tl_id = "3d8ce53d"]
    Photo {
        photo: Box<Photo>,
        caption: String,
    },

    #[tl_id = "56e0d474"]
    Geo {
        geo: Box<GeoPoint>,
    },

    #[tl_id = "5e7d2f39"]
    Contact {
        phone_number: String,
        first_name: String,
        last_name: String,
        user_id: i32,
    },

    #[tl_id = "9f84f49e"]
    Unsupported,

    #[tl_id = "f3e02ea8"]
    Document {
        document: Box<Document>,
        caption: String,
    },

    #[tl_id = "a32dd600"]
    WebPage {
        webpage: Box<WebPage>,
    },

    #[tl_id = "7912b71f"]
    Venue {
        geo: Box<GeoPoint>,
        title: String,
        address: String,
        provider: String,
        venue_id: String,
    },

    #[tl_id = "fdb19008"]
    Game {
        game: Box<Game>,
    },
}

#[derive(Debug, TlType)]
enum MessageAction {
    #[tl_id = "b6aef7b0"]
    Empty,

    #[tl_id = "a6638b9a"]
    ChatCreate {
        title: String,
        users: Vec<i32>,
    },

    #[tl_id = "b5a1ce5a"]
    ChatEditTitle {
        title: String,
    },

    #[tl_id = "7fcb13a8"]
    ChatEditPhoto {
        photo: Box<Photo>,
    },

    #[tl_id = "95e3fbef"]
    ChatDeletePhoto,

    #[tl_id = "488a7337"]
    ChatAddUser {
        users: Vec<i32>,
    },

    #[tl_id = "b2ae9b0c"]
    ChatDeleteUser {
        user_id: i32,
    },

    #[tl_id = "f89cf5e8"]
    ChatJoinedByLink {
        inviter_id: i32,
    },

    #[tl_id = "95d2ac92"]
    ChannelCreate {
        title: String,
    },

    #[tl_id = "51bdb021"]
    ChatMigrateTo {
        channel_id: i32,
    },

    #[tl_id = "b055eaee"]
    ChannelMigrateFrom {
        title: String,
        chat_id: i32,
    },

    #[tl_id = "94bd38ed"]
    PinMessage,

    #[tl_id = "9fbab604"]
    HistoryClear,

    #[tl_id = "92a72876"]
    GameScore {
        game_id: i64,
        score: i32,
    },

    #[tl_id = "80e11a7f"]
    PhoneCall {
        flags: u32,
        call_id: i64,
        reason: Box<PhoneCallDiscardReason>,
        #[flag_bit = "1"]
        duration: Option<i32>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "66ffba14"]
struct Dialog {
    flags: u32,
    #[flag_bit = "2"]
    pinned: Option<bool>,
    peer: Box<Peer>,
    top_message: i32,
    read_inbox_max_id: i32,
    read_outbox_max_id: i32,
    unread_count: i32,
    notify_settings: Box<PeerNotifySettings>,
    pts: i32,
    #[flag_bit = "1"]
    draft: Option<Box<DraftMessage>>,
}

#[derive(Debug, TlType)]
enum Photo {
    #[tl_id = "2331b22d"]
    Empty {
        id: i64,
    },

    #[tl_id = "9288dd29"]
    Photo {
        flags: u32,
        has_stickers: bool,
        id: i64,
        access_hash: i64,
        date: i32,
        sizes: Vec<PhotoSize>,
    },
}

#[derive(Debug, TlType)]
enum PhotoSize {
    #[tl_id = "e17e23c"]
    Empty {
        ttype: String,
    },

    #[tl_id = "77bfb61b"]
    Size {
        ttype: String,
        location: Box<FileLocation>,
        w: i32,
        h: i32,
        size: i32,
    },

    #[tl_id = "e9a734fa"]
    Cached {
        ttype: String,
        location: Box<FileLocation>,
        w: i32,
        h: i32,
        bytes: Vec<u8>,
    },
}

#[derive(Debug, TlType)]
enum GeoPoint {
    #[tl_id = "1117dd5f"]
    Empty,

    #[tl_id = "2049d70c"]
    Point {
        long: f64,
        lat: f64,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "811ea28e"]
struct AuthCheckedPhone {
    phone_registered: bool,
}

#[derive(Debug, TlType)]
#[tl_id = "5e002502"]
struct AuthSentCode {
    flags: u32,
    phone_registered: bool,
    ttype: Box<AuthSentCodeType>,
    phone_code_hash: String,
    #[flag_bit = "1"]
    next_type: Option<Box<AuthCodeType>>,
    #[flag_bit = "2"]
    timeout: Option<i32>,
}

#[derive(Debug, TlType)]
#[tl_id = "cd050916"]
struct AuthAuthorization {
    flags: u32,
    tmp_sessions: i32,
    user: Box<User>,
}

#[derive(Debug, TlType)]
#[tl_id = "df969c2d"]
struct AuthExportedAuthorization {
    id: i32,
    bytes: Vec<u8>,
}

#[derive(Debug, TlType)]
enum InputNotifyPeer {
    #[tl_id = "b8bc5b0c"]
    Peer {
        peer: Box<InputPeer>,
    },

    #[tl_id = "193b4417"]
    Users,

    #[tl_id = "4a95e84e"]
    Chats,

    #[tl_id = "a429b886"]
    All,
}

#[derive(Debug, TlType)]
enum InputPeerNotifyEvents {
    #[tl_id = "f03064d8"]
    Empty,

    #[tl_id = "e86a2c74"]
    All,
}

#[derive(Debug, TlType)]
#[tl_id = "38935eb2"]
struct InputPeerNotifySettings {
    flags: u32,
    show_previews: bool,
    #[flag_bit = "1"]
    silent: Option<bool>,
    mute_until: i32,
    sound: String,
}

#[derive(Debug, TlType)]
enum PeerNotifyEvents {
    #[tl_id = "add53cb3"]
    Empty,

    #[tl_id = "6d1ded88"]
    All,
}

#[derive(Debug, TlType)]
enum PeerNotifySettings {
    #[tl_id = "70a68512"]
    Empty,

    #[tl_id = "9acda4c0"]
    Settings {
        flags: u32,
        show_previews: bool,
        #[flag_bit = "1"]
        silent: Option<bool>,
        mute_until: i32,
        sound: String,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "818426cd"]
struct PeerSettings {
    flags: u32,
    report_spam: bool,
}

#[derive(Debug, TlType)]
enum WallPaper {
    #[tl_id = "ccb03657"]
    Paper {
        id: i32,
        title: String,
        sizes: Vec<PhotoSize>,
        color: i32,
    },

    #[tl_id = "63117f24"]
    Solid {
        id: i32,
        title: String,
        bg_color: i32,
        color: i32,
    },
}

#[derive(Debug, TlType)]
enum ReportReason {
    #[tl_id = "58dbcab8"]
    InputSpam,

    #[tl_id = "1e22c78d"]
    InputViolence,

    #[tl_id = "2e59d922"]
    InputPornography,

    #[tl_id = "e1746d0a"]
    InputOther {
        text: String,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "f220f3f"]
struct UserFull {
    flags: u32,
    blocked: bool,
    #[flag_bit = "4"]
    phone_calls_available: Option<bool>,
    user: Box<User>,
    #[flag_bit = "1"]
    about: Option<String>,
    link: Box<ContactsLink>,
    #[flag_bit = "2"]
    profile_photo: Option<Box<Photo>>,
    notify_settings: Box<PeerNotifySettings>,
    #[flag_bit = "3"]
    bot_info: Option<Box<BotInfo>>,
    common_chats_count: i32,
}

#[derive(Debug, TlType)]
#[tl_id = "f911c994"]
struct Contact {
    user_id: i32,
    mutual: bool,
}

#[derive(Debug, TlType)]
#[tl_id = "d0028438"]
struct ImportedContact {
    user_id: i32,
    client_id: i64,
}

#[derive(Debug, TlType)]
#[tl_id = "561bc879"]
struct ContactBlocked {
    user_id: i32,
    date: i32,
}

#[derive(Debug, TlType)]
#[tl_id = "d3680c61"]
struct ContactStatus {
    user_id: i32,
    status: Box<UserStatus>,
}

#[derive(Debug, TlType)]
#[tl_id = "3ace484c"]
struct ContactsLink {
    my_link: Box<ContactLink>,
    foreign_link: Box<ContactLink>,
    user: Box<User>,
}

#[derive(Debug, TlType)]
enum ContactsContacts {
    #[tl_id = "b74ba9d2"]
    ContactsNotModified,

    #[tl_id = "6f8b8cb2"]
    Contacts {
        contacts: Vec<Contact>,
        users: Vec<User>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "ad524315"]
struct ContactsImportedContacts {
    imported: Vec<ImportedContact>,
    retry_contacts: Vec<i64>,
    users: Vec<User>,
}

#[derive(Debug, TlType)]
enum ContactsBlocked {
    #[tl_id = "1c138d15"]
    Blocked {
        blocked: Vec<ContactBlocked>,
        users: Vec<User>,
    },

    #[tl_id = "900802a1"]
    Slice {
        count: i32,
        blocked: Vec<ContactBlocked>,
        users: Vec<User>,
    },
}

#[derive(Debug, TlType)]
enum MessagesDialogs {
    #[tl_id = "15ba6c40"]
    Dialogs {
        dialogs: Vec<Dialog>,
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },

    #[tl_id = "71e094f3"]
    Slice {
        count: i32,
        dialogs: Vec<Dialog>,
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },
}

#[derive(Debug, TlType)]
enum MessagesMessages {
    #[tl_id = "8c718e87"]
    Messages {
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },

    #[tl_id = "b446ae3"]
    MessagesSlice {
        count: i32,
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },

    #[tl_id = "99262e37"]
    ChannelMessages {
        flags: u32,
        pts: i32,
        count: i32,
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },
}

#[derive(Debug, TlType)]
enum MessagesChats {
    #[tl_id = "64ff9fd5"]
    Chats {
        chats: Vec<Chat>,
    },

    #[tl_id = "9cd81144"]
    Slice {
        count: i32,
        chats: Vec<Chat>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "e5d7d19c"]
struct MessagesChatFull {
    full_chat: Box<ChatFull>,
    chats: Vec<Chat>,
    users: Vec<User>,
}

#[derive(Debug, TlType)]
#[tl_id = "b45c69d1"]
struct MessagesAffectedHistory {
    pts: i32,
    pts_count: i32,
    offset: i32,
}

#[derive(Debug, TlType)]
enum MessagesFilter {
    #[tl_id = "57e2f66c"]
    InputEmpty,

    #[tl_id = "9609a51c"]
    InputPhotos,

    #[tl_id = "9fc00e65"]
    InputVideo,

    #[tl_id = "56e9f0e4"]
    InputPhotoVideo,

    #[tl_id = "d95e73bb"]
    InputPhotoVideoDocuments,

    #[tl_id = "9eddf188"]
    InputDocument,

    #[tl_id = "7ef0dd87"]
    InputUrl,

    #[tl_id = "ffc86587"]
    InputGif,

    #[tl_id = "50f5c392"]
    InputVoice,

    #[tl_id = "3751b49e"]
    InputMusic,

    #[tl_id = "3a20ecb8"]
    InputChatPhotos,

    #[tl_id = "80c99768"]
    InputPhoneCalls {
        flags: u32,
        missed: bool,
    },
}

#[derive(Debug, TlType)]
enum Update {
    #[tl_id = "1f2b0afd"]
    NewMessage {
        message: Box<Message>,
        pts: i32,
        pts_count: i32,
    },

    #[tl_id = "4e90bfd6"]
    MessageID {
        id: i32,
        random_id: i64,
    },

    #[tl_id = "a20db0e5"]
    DeleteMessages {
        messages: Vec<i32>,
        pts: i32,
        pts_count: i32,
    },

    #[tl_id = "5c486927"]
    UserTyping {
        user_id: i32,
        action: Box<SendMessageAction>,
    },

    #[tl_id = "9a65ea1f"]
    ChatUserTyping {
        chat_id: i32,
        user_id: i32,
        action: Box<SendMessageAction>,
    },

    #[tl_id = "7761198"]
    ChatParticipants {
        participants: Box<ChatParticipants>,
    },

    #[tl_id = "1bfbd823"]
    UserStatus {
        user_id: i32,
        status: Box<UserStatus>,
    },

    #[tl_id = "a7332b73"]
    UserName {
        user_id: i32,
        first_name: String,
        last_name: String,
        username: String,
    },

    #[tl_id = "95313b0c"]
    UserPhoto {
        user_id: i32,
        date: i32,
        photo: Box<UserProfilePhoto>,
        previous: bool,
    },

    #[tl_id = "2575bbb9"]
    ContactRegistered {
        user_id: i32,
        date: i32,
    },

    #[tl_id = "9d2e67c5"]
    ContactLink {
        user_id: i32,
        my_link: Box<ContactLink>,
        foreign_link: Box<ContactLink>,
    },

    #[tl_id = "12bcbd9a"]
    NewEncryptedMessage {
        message: Box<EncryptedMessage>,
        qts: i32,
    },

    #[tl_id = "1710f156"]
    EncryptedChatTyping {
        chat_id: i32,
    },

    #[tl_id = "b4a2e88d"]
    Encryption {
        chat: Box<EncryptedChat>,
        date: i32,
    },

    #[tl_id = "38fe25b7"]
    EncryptedMessagesRead {
        chat_id: i32,
        max_date: i32,
        date: i32,
    },

    #[tl_id = "ea4b0e5c"]
    ChatParticipantAdd {
        chat_id: i32,
        user_id: i32,
        inviter_id: i32,
        date: i32,
        version: i32,
    },

    #[tl_id = "6e5f8c22"]
    ChatParticipantDelete {
        chat_id: i32,
        user_id: i32,
        version: i32,
    },

    #[tl_id = "8e5e9873"]
    DcOptions {
        dc_options: Vec<DcOption>,
    },

    #[tl_id = "80ece81a"]
    UserBlocked {
        user_id: i32,
        blocked: bool,
    },

    #[tl_id = "bec268ef"]
    NotifySettings {
        peer: Box<NotifyPeer>,
        notify_settings: Box<PeerNotifySettings>,
    },

    #[tl_id = "ebe46819"]
    ServiceNotification {
        flags: u32,
        popup: bool,
        #[flag_bit = "1"]
        inbox_date: Option<i32>,
        ttype: String,
        message: String,
        media: Box<MessageMedia>,
        entities: Vec<MessageEntity>,
    },

    #[tl_id = "ee3b272a"]
    Privacy {
        key: Box<PrivacyKey>,
        rules: Vec<PrivacyRule>,
    },

    #[tl_id = "12b9417b"]
    UserPhone {
        user_id: i32,
        phone: String,
    },

    #[tl_id = "9961fd5c"]
    ReadHistoryInbox {
        peer: Box<Peer>,
        max_id: i32,
        pts: i32,
        pts_count: i32,
    },

    #[tl_id = "2f2f21bf"]
    ReadHistoryOutbox {
        peer: Box<Peer>,
        max_id: i32,
        pts: i32,
        pts_count: i32,
    },

    #[tl_id = "7f891213"]
    WebPage {
        webpage: Box<WebPage>,
        pts: i32,
        pts_count: i32,
    },

    #[tl_id = "68c13933"]
    ReadMessagesContents {
        messages: Vec<i32>,
        pts: i32,
        pts_count: i32,
    },

    #[tl_id = "eb0467fb"]
    ChannelTooLong {
        flags: u32,
        channel_id: i32,
        pts: i32,
    },

    #[tl_id = "b6d45656"]
    Channel {
        channel_id: i32,
    },

    #[tl_id = "62ba04d9"]
    NewChannelMessage {
        message: Box<Message>,
        pts: i32,
        pts_count: i32,
    },

    #[tl_id = "4214f37f"]
    ReadChannelInbox {
        channel_id: i32,
        max_id: i32,
    },

    #[tl_id = "c37521c9"]
    DeleteChannelMessages {
        channel_id: i32,
        messages: Vec<i32>,
        pts: i32,
        pts_count: i32,
    },

    #[tl_id = "98a12b4b"]
    ChannelMessageViews {
        channel_id: i32,
        id: i32,
        views: i32,
    },

    #[tl_id = "6e947941"]
    ChatAdmins {
        chat_id: i32,
        enabled: bool,
        version: i32,
    },

    #[tl_id = "b6901959"]
    ChatParticipantAdmin {
        chat_id: i32,
        user_id: i32,
        is_admin: bool,
        version: i32,
    },

    #[tl_id = "688a30aa"]
    NewStickerSet {
        stickerset: Box<MessagesStickerSet>,
    },

    #[tl_id = "bb2d201"]
    StickerSetsOrder {
        flags: u32,
        masks: bool,
        order: Vec<i64>,
    },

    #[tl_id = "43ae3dec"]
    StickerSets,

    #[tl_id = "9375341e"]
    SavedGifs,

    #[tl_id = "54826690"]
    BotInlineQuery {
        flags: u32,
        query_id: i64,
        user_id: i32,
        query: String,
        geo: Box<GeoPoint>,
        offset: String,
    },

    #[tl_id = "e48f964"]
    BotInlineSend {
        flags: u32,
        user_id: i32,
        query: String,
        geo: Box<GeoPoint>,
        id: String,
        #[flag_bit = "1"]
        msg_id: Option<Box<InputBotInlineMessageID>>,
    },

    #[tl_id = "1b3f4df7"]
    EditChannelMessage {
        message: Box<Message>,
        pts: i32,
        pts_count: i32,
    },

    #[tl_id = "98592475"]
    ChannelPinnedMessage {
        channel_id: i32,
        id: i32,
    },

    #[tl_id = "e73547e1"]
    BotCallbackQuery {
        flags: u32,
        query_id: i64,
        user_id: i32,
        peer: Box<Peer>,
        msg_id: i32,
        chat_instance: i64,
        data: Vec<u8>,
        #[flag_bit = "1"]
        game_short_name: Option<String>,
    },

    #[tl_id = "e40370a3"]
    EditMessage {
        message: Box<Message>,
        pts: i32,
        pts_count: i32,
    },

    #[tl_id = "f9d27a5a"]
    InlineBotCallbackQuery {
        flags: u32,
        query_id: i64,
        user_id: i32,
        msg_id: Box<InputBotInlineMessageID>,
        chat_instance: i64,
        data: Vec<u8>,
        #[flag_bit = "1"]
        game_short_name: Option<String>,
    },

    #[tl_id = "25d6c9c7"]
    ReadChannelOutbox {
        channel_id: i32,
        max_id: i32,
    },

    #[tl_id = "ee2bb969"]
    DraftMessage {
        peer: Box<Peer>,
        draft: Box<DraftMessage>,
    },

    #[tl_id = "571d2742"]
    ReadFeaturedStickers,

    #[tl_id = "9a422c20"]
    RecentStickers,

    #[tl_id = "a229dd06"]
    Config,

    #[tl_id = "3354678f"]
    PtsChanged,

    #[tl_id = "40771900"]
    ChannelWebPage {
        channel_id: i32,
        webpage: Box<WebPage>,
        pts: i32,
        pts_count: i32,
    },

    #[tl_id = "ab0f6b1e"]
    PhoneCall {
        phone_call: Box<PhoneCall>,
    },

    #[tl_id = "d711a2cc"]
    DialogPinned {
        flags: u32,
        pinned: bool,
        peer: Box<Peer>,
    },

    #[tl_id = "d8caf68d"]
    PinnedDialogs {
        flags: u32,
        order: Vec<Peer>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "a56c2a3e"]
struct UpdatesState {
    pts: i32,
    qts: i32,
    date: i32,
    seq: i32,
    unread_count: i32,
}

#[derive(Debug, TlType)]
enum UpdatesDifference {
    #[tl_id = "5d75a138"]
    Empty {
        date: i32,
        seq: i32,
    },

    #[tl_id = "f49ca0"]
    Difference {
        new_messages: Vec<Message>,
        new_encrypted_messages: Vec<EncryptedMessage>,
        other_updates: Vec<Update>,
        chats: Vec<Chat>,
        users: Vec<User>,
        state: Box<UpdatesState>,
    },

    #[tl_id = "a8fb1981"]
    Slice {
        new_messages: Vec<Message>,
        new_encrypted_messages: Vec<EncryptedMessage>,
        other_updates: Vec<Update>,
        chats: Vec<Chat>,
        users: Vec<User>,
        intermediate_state: Box<UpdatesState>,
    },

    #[tl_id = "4afe8f6d"]
    TooLong {
        pts: i32,
    },
}

#[derive(Debug, TlType)]
enum Updates {
    #[tl_id = "e317af7e"]
    TooLong,

    #[tl_id = "914fbf11"]
    UpdateShortMessage {
        flags: u32,
        #[flag_bit = "1"]
        out: Option<bool>,
        #[flag_bit = "4"]
        mentioned: Option<bool>,
        #[flag_bit = "5"]
        media_unread: Option<bool>,
        #[flag_bit = "13"]
        silent: Option<bool>,
        id: i32,
        user_id: i32,
        message: String,
        pts: i32,
        pts_count: i32,
        date: i32,
        #[flag_bit = "2"]
        fwd_from: Option<Box<MessageFwdHeader>>,
        #[flag_bit = "11"]
        via_bot_id: Option<i32>,
        #[flag_bit = "3"]
        reply_to_msg_id: Option<i32>,
        #[flag_bit = "7"]
        entities: Option<Vec<MessageEntity>>,
    },

    #[tl_id = "16812688"]
    UpdateShortChatMessage {
        flags: u32,
        #[flag_bit = "1"]
        out: Option<bool>,
        #[flag_bit = "4"]
        mentioned: Option<bool>,
        #[flag_bit = "5"]
        media_unread: Option<bool>,
        #[flag_bit = "13"]
        silent: Option<bool>,
        id: i32,
        from_id: i32,
        chat_id: i32,
        message: String,
        pts: i32,
        pts_count: i32,
        date: i32,
        #[flag_bit = "2"]
        fwd_from: Option<Box<MessageFwdHeader>>,
        #[flag_bit = "11"]
        via_bot_id: Option<i32>,
        #[flag_bit = "3"]
        reply_to_msg_id: Option<i32>,
        #[flag_bit = "7"]
        entities: Option<Vec<MessageEntity>>,
    },

    #[tl_id = "78d4dec1"]
    UpdateShort {
        update: Box<Update>,
        date: i32,
    },

    #[tl_id = "725b04c3"]
    Combined {
        updates: Vec<Update>,
        users: Vec<User>,
        chats: Vec<Chat>,
        date: i32,
        seq_start: i32,
        seq: i32,
    },

    #[tl_id = "74ae4240"]
    Updates {
        updates: Vec<Update>,
        users: Vec<User>,
        chats: Vec<Chat>,
        date: i32,
        seq: i32,
    },

    #[tl_id = "11f1331c"]
    UpdateShortSentMessage {
        flags: u32,
        #[flag_bit = "1"]
        out: Option<bool>,
        id: i32,
        pts: i32,
        pts_count: i32,
        date: i32,
        #[flag_bit = "9"]
        media: Option<Box<MessageMedia>>,
        #[flag_bit = "7"]
        entities: Option<Vec<MessageEntity>>,
    },
}

#[derive(Debug, TlType)]
enum PhotosPhotos {
    #[tl_id = "8dca6aa5"]
    Photos {
        photos: Vec<Photo>,
        users: Vec<User>,
    },

    #[tl_id = "15051f54"]
    PhotosSlice {
        count: i32,
        photos: Vec<Photo>,
        users: Vec<User>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "20212ca8"]
struct PhotosPhoto {
    photo: Box<Photo>,
    users: Vec<User>,
}

#[derive(Debug, TlType)]
#[tl_id = "96a18d5"]
struct UploadFile {
    ttype: Box<StorageFileType>,
    mtime: i32,
    bytes: Vec<u8>,
}

#[derive(Debug, TlType)]
#[tl_id = "5d8c6cc"]
struct DcOption {
    flags: u32,
    ipv6: bool,
    #[flag_bit = "1"]
    media_only: Option<bool>,
    #[flag_bit = "2"]
    tcpo_only: Option<bool>,
    id: i32,
    ip_address: String,
    port: i32,
}

#[derive(Debug, TlType)]
#[tl_id = "3af6fb5f"]
struct Config {
    flags: u32,
    #[flag_bit = "1"]
    phonecalls_enabled: Option<bool>,
    date: i32,
    expires: i32,
    test_mode: bool,
    this_dc: i32,
    dc_options: Vec<DcOption>,
    chat_size_max: i32,
    megagroup_size_max: i32,
    forwarded_count_max: i32,
    online_update_period_ms: i32,
    offline_blur_timeout_ms: i32,
    offline_idle_timeout_ms: i32,
    online_cloud_timeout_ms: i32,
    notify_cloud_delay_ms: i32,
    notify_default_delay_ms: i32,
    chat_big_size: i32,
    push_chat_period_ms: i32,
    push_chat_limit: i32,
    saved_gifs_limit: i32,
    edit_time_limit: i32,
    rating_e_decay: i32,
    stickers_recent_limit: i32,
    tmp_sessions: i32,
    pinned_dialogs_count_max: i32,
    call_receive_timeout_ms: i32,
    call_ring_timeout_ms: i32,
    call_connect_timeout_ms: i32,
    call_packet_timeout_ms: i32,
    disabled_features: Vec<DisabledFeature>,
}

#[derive(Debug, TlType)]
#[tl_id = "8e1a1775"]
struct NearestDc {
    country: String,
    this_dc: i32,
    nearest_dc: i32,
}

#[derive(Debug, TlType)]
enum HelpAppUpdate {
    #[tl_id = "8987f311"]
    Update {
        id: i32,
        critical: bool,
        url: String,
        text: String,
    },

    #[tl_id = "c45a6536"]
    No,
}

#[derive(Debug, TlType)]
#[tl_id = "18cb9f78"]
struct HelpInviteText {
    message: String,
}

#[derive(Debug, TlType)]
enum EncryptedChat {
    #[tl_id = "ab7ec0a0"]
    Empty {
        id: i32,
    },

    #[tl_id = "3bf703dc"]
    Waiting {
        id: i32,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
    },

    #[tl_id = "c878527e"]
    Requested {
        id: i32,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
        g_a: Vec<u8>,
    },

    #[tl_id = "fa56ce36"]
    Chat {
        id: i32,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
        g_a_or_b: Vec<u8>,
        key_fingerprint: i64,
    },

    #[tl_id = "13d6dd27"]
    Discarded {
        id: i32,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "f141b5e1"]
struct InputEncryptedChat {
    chat_id: i32,
    access_hash: i64,
}

#[derive(Debug, TlType)]
enum EncryptedFile {
    #[tl_id = "c21f497e"]
    Empty,

    #[tl_id = "4a70994c"]
    File {
        id: i64,
        access_hash: i64,
        size: i32,
        dc_id: i32,
        key_fingerprint: i32,
    },
}

#[derive(Debug, TlType)]
enum InputEncryptedFile {
    #[tl_id = "1837c364"]
    Empty,

    #[tl_id = "64bd0306"]
    Uploaded {
        id: i64,
        parts: i32,
        md5_checksum: String,
        key_fingerprint: i32,
    },

    #[tl_id = "5a17b5e5"]
    File {
        id: i64,
        access_hash: i64,
    },

    #[tl_id = "2dc173c8"]
    BigUploaded {
        id: i64,
        parts: i32,
        key_fingerprint: i32,
    },
}

#[derive(Debug, TlType)]
enum EncryptedMessage {
    #[tl_id = "ed18c118"]
    Message {
        random_id: i64,
        chat_id: i32,
        date: i32,
        bytes: Vec<u8>,
        file: Box<EncryptedFile>,
    },

    #[tl_id = "23734b06"]
    Service {
        random_id: i64,
        chat_id: i32,
        date: i32,
        bytes: Vec<u8>,
    },
}

#[derive(Debug, TlType)]
enum MessagesDhConfig {
    #[tl_id = "c0e24635"]
    NotModified {
        random: Vec<u8>,
    },

    #[tl_id = "2c221edd"]
    Config {
        g: i32,
        p: Vec<u8>,
        version: i32,
        random: Vec<u8>,
    },
}

#[derive(Debug, TlType)]
enum MessagesSentEncryptedMessage {
    #[tl_id = "560f8935"]
    Message {
        date: i32,
    },

    #[tl_id = "9493ff32"]
    File {
        date: i32,
        file: Box<EncryptedFile>,
    },
}

#[derive(Debug, TlType)]
enum InputDocument {
    #[tl_id = "72f0eaae"]
    Empty,

    #[tl_id = "18798952"]
    Document {
        id: i64,
        access_hash: i64,
    },
}

#[derive(Debug, TlType)]
enum Document {
    #[tl_id = "36f8c871"]
    Empty {
        id: i64,
    },

    #[tl_id = "87232bc7"]
    Document {
        id: i64,
        access_hash: i64,
        date: i32,
        mime_type: String,
        size: i32,
        thumb: Box<PhotoSize>,
        dc_id: i32,
        version: i32,
        attributes: Vec<DocumentAttribute>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "17c6b5f6"]
struct HelpSupport {
    phone_number: String,
    user: Box<User>,
}

#[derive(Debug, TlType)]
enum NotifyPeer {
    #[tl_id = "9fd40bd8"]
    Peer {
        peer: Box<Peer>,
    },

    #[tl_id = "b4c83b4c"]
    Users,

    #[tl_id = "c007cec3"]
    Chats,

    #[tl_id = "74d07c60"]
    All,
}

#[derive(Debug, TlType)]
enum SendMessageAction {
    #[tl_id = "16bf744e"]
    Typing,

    #[tl_id = "fd5ec8f5"]
    Cancel,

    #[tl_id = "a187d66f"]
    RecordVideo,

    #[tl_id = "e9763aec"]
    UploadVideo {
        progress: i32,
    },

    #[tl_id = "d52f73f7"]
    RecordAudio,

    #[tl_id = "f351d7ab"]
    UploadAudio {
        progress: i32,
    },

    #[tl_id = "d1d34a26"]
    UploadPhoto {
        progress: i32,
    },

    #[tl_id = "aa0cd9e4"]
    UploadDocument {
        progress: i32,
    },

    #[tl_id = "176f8ba1"]
    GeoLocation,

    #[tl_id = "628cbc6f"]
    ChooseContact,

    #[tl_id = "dd6a8f48"]
    GamePlay,
}

#[derive(Debug, TlType)]
#[tl_id = "1aa1f784"]
struct ContactsFound {
    results: Vec<Peer>,
    chats: Vec<Chat>,
    users: Vec<User>,
}

#[derive(Debug, TlType)]
enum InputPrivacyKey {
    #[tl_id = "4f96cb18"]
    StatusTimestamp,

    #[tl_id = "bdfb0426"]
    ChatInvite,

    #[tl_id = "fabadc5f"]
    PhoneCall,
}

#[derive(Debug, TlType)]
enum PrivacyKey {
    #[tl_id = "bc2eab30"]
    StatusTimestamp,

    #[tl_id = "500e6dfa"]
    ChatInvite,

    #[tl_id = "3d662b7b"]
    PhoneCall,
}

#[derive(Debug, TlType)]
enum InputPrivacyRule {
    #[tl_id = "d09e07b"]
    ValueAllowContacts,

    #[tl_id = "184b35ce"]
    ValueAllowAll,

    #[tl_id = "131cc67f"]
    ValueAllowUsers {
        users: Vec<InputUser>,
    },

    #[tl_id = "ba52007"]
    ValueDisallowContacts,

    #[tl_id = "d66b66c9"]
    ValueDisallowAll,

    #[tl_id = "90110467"]
    ValueDisallowUsers {
        users: Vec<InputUser>,
    },
}

#[derive(Debug, TlType)]
enum PrivacyRule {
    #[tl_id = "fffe1bac"]
    ValueAllowContacts,

    #[tl_id = "65427b82"]
    ValueAllowAll,

    #[tl_id = "4d5bbe0c"]
    ValueAllowUsers {
        users: Vec<i32>,
    },

    #[tl_id = "f888fa1a"]
    ValueDisallowContacts,

    #[tl_id = "8b73e763"]
    ValueDisallowAll,

    #[tl_id = "c7f49b7"]
    ValueDisallowUsers {
        users: Vec<i32>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "554abb6f"]
struct AccountPrivacyRules {
    rules: Vec<PrivacyRule>,
    users: Vec<User>,
}

#[derive(Debug, TlType)]
#[tl_id = "b8d0afdf"]
struct AccountDaysTTL {
    days: i32,
}

#[derive(Debug, TlType)]
enum DocumentAttribute {
    #[tl_id = "6c37c15c"]
    ImageSize {
        w: i32,
        h: i32,
    },

    #[tl_id = "11b58939"]
    Animated,

    #[tl_id = "6319d612"]
    Sticker {
        flags: u32,
        #[flag_bit = "1"]
        mask: Option<bool>,
        alt: String,
        stickerset: Box<InputStickerSet>,
        mask_coords: Box<MaskCoords>,
    },

    #[tl_id = "5910cccb"]
    Video {
        duration: i32,
        w: i32,
        h: i32,
    },

    #[tl_id = "9852f9c6"]
    Audio {
        flags: u32,
        #[flag_bit = "10"]
        voice: Option<bool>,
        duration: i32,
        title: String,
        #[flag_bit = "1"]
        performer: Option<String>,
        #[flag_bit = "2"]
        waveform: Option<Vec<u8>>,
    },

    #[tl_id = "15590068"]
    Filename {
        file_name: String,
    },

    #[tl_id = "9801d2f7"]
    HasStickers,
}

#[derive(Debug, TlType)]
enum MessagesStickers {
    #[tl_id = "f1749a22"]
    NotModified,

    #[tl_id = "8a8ecd32"]
    Stickers {
        hash: String,
        stickers: Vec<Document>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "12b299d4"]
struct StickerPack {
    emoticon: String,
    documents: Vec<i64>,
}

#[derive(Debug, TlType)]
enum MessagesAllStickers {
    #[tl_id = "e86602c3"]
    NotModified,

    #[tl_id = "edfd405f"]
    Stickers {
        hash: i32,
        sets: Vec<StickerSet>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "ae636f24"]
struct DisabledFeature {
    feature: String,
    description: String,
}

#[derive(Debug, TlType)]
#[tl_id = "84d19185"]
struct MessagesAffectedMessages {
    pts: i32,
    pts_count: i32,
}

#[derive(Debug, TlType)]
enum ContactLink {
    #[tl_id = "5f4f9247"]
    Unknown,

    #[tl_id = "feedd3ad"]
    None,

    #[tl_id = "268f3f59"]
    HasPhone,

    #[tl_id = "d502c2d0"]
    Contact,
}

#[derive(Debug, TlType)]
enum WebPage {
    #[tl_id = "eb1477e8"]
    Empty {
        id: i64,
    },

    #[tl_id = "c586da1c"]
    Pending {
        id: i64,
        date: i32,
    },

    #[tl_id = "5f07b4bc"]
    Page {
        flags: u32,
        id: i64,
        url: String,
        display_url: String,
        hash: i32,
        ttype: String,
        #[flag_bit = "1"]
        site_name: Option<String>,
        #[flag_bit = "2"]
        title: Option<String>,
        #[flag_bit = "3"]
        description: Option<String>,
        #[flag_bit = "4"]
        photo: Option<Box<Photo>>,
        #[flag_bit = "5"]
        embed_url: Option<String>,
        #[flag_bit = "5"]
        embed_type: Option<String>,
        #[flag_bit = "6"]
        embed_width: Option<i32>,
        #[flag_bit = "6"]
        embed_height: Option<i32>,
        #[flag_bit = "7"]
        duration: Option<i32>,
        #[flag_bit = "8"]
        author: Option<String>,
        #[flag_bit = "9"]
        document: Option<Box<Document>>,
        #[flag_bit = "10"]
        cached_page: Option<Box<Page>>,
    },

    #[tl_id = "85849473"]
    NotModified,
}

#[derive(Debug, TlType)]
#[tl_id = "7bf2e6f6"]
struct Authorization {
    hash: i64,
    flags: i32,
    device_model: String,
    platform: String,
    system_version: String,
    api_id: i32,
    app_name: String,
    app_version: String,
    date_created: i32,
    date_active: i32,
    ip: String,
    country: String,
    region: String,
}

#[derive(Debug, TlType)]
#[tl_id = "1250abde"]
struct AccountAuthorizations {
    authorizations: Vec<Authorization>,
}

#[derive(Debug, TlType)]
enum AccountPassword {
    #[tl_id = "96dabc18"]
    No {
        new_salt: Vec<u8>,
        email_unconfirmed_pattern: String,
    },

    #[tl_id = "7c18141c"]
    Password {
        current_salt: Vec<u8>,
        new_salt: Vec<u8>,
        hint: String,
        has_recovery: bool,
        email_unconfirmed_pattern: String,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "b7b72ab3"]
struct AccountPasswordSettings {
    email: String,
}

#[derive(Debug, TlType)]
#[tl_id = "86916deb"]
struct AccountPasswordInputSettings {
    flags: u32,
    new_salt: Vec<u8>,
    new_password_hash: Vec<u8>,
    hint: String,
    #[flag_bit = "1"]
    email: Option<String>,
}

#[derive(Debug, TlType)]
#[tl_id = "137948a5"]
struct AuthPasswordRecovery {
    email_pattern: String,
}

#[derive(Debug, TlType)]
#[tl_id = "a384b779"]
struct ReceivedNotifyMessage {
    id: i32,
    flags: i32,
}

#[derive(Debug, TlType)]
enum ExportedChatInvite {
    #[tl_id = "69df3769"]
    Empty,

    #[tl_id = "fc2e05bc"]
    Exported {
        link: String,
    },
}

#[derive(Debug, TlType)]
enum ChatInvite {
    #[tl_id = "5a686d7c"]
    Already {
        chat: Box<Chat>,
    },

    #[tl_id = "db74f558"]
    Invite {
        flags: u32,
        channel: bool,
        #[flag_bit = "1"]
        broadcast: Option<bool>,
        #[flag_bit = "2"]
        public: Option<bool>,
        #[flag_bit = "3"]
        megagroup: Option<bool>,
        title: String,
        photo: Box<ChatPhoto>,
        participants_count: i32,
        #[flag_bit = "4"]
        participants: Option<Vec<User>>,
    },
}

#[derive(Debug, TlType)]
enum InputStickerSet {
    #[tl_id = "ffb62b95"]
    Empty,

    #[tl_id = "9de7a269"]
    ID {
        id: i64,
        access_hash: i64,
    },

    #[tl_id = "861cc8a0"]
    ShortName {
        short_name: String,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "cd303b41"]
struct StickerSet {
    flags: u32,
    installed: bool,
    #[flag_bit = "1"]
    archived: Option<bool>,
    #[flag_bit = "2"]
    official: Option<bool>,
    #[flag_bit = "3"]
    masks: Option<bool>,
    id: i64,
    access_hash: i64,
    title: String,
    short_name: String,
    count: i32,
    hash: i32,
}

#[derive(Debug, TlType)]
#[tl_id = "b60a24a6"]
struct MessagesStickerSet {
    set: Box<StickerSet>,
    packs: Vec<StickerPack>,
    documents: Vec<Document>,
}

#[derive(Debug, TlType)]
#[tl_id = "c27ac8c7"]
struct BotCommand {
    command: String,
    description: String,
}

#[derive(Debug, TlType)]
#[tl_id = "98e81d3a"]
struct BotInfo {
    user_id: i32,
    description: String,
    commands: Vec<BotCommand>,
}

#[derive(Debug, TlType)]
enum KeyboardButton {
    #[tl_id = "a2fa4880"]
    Button {
        text: String,
    },

    #[tl_id = "258aff05"]
    Url {
        text: String,
        url: String,
    },

    #[tl_id = "683a5e46"]
    Callback {
        text: String,
        data: Vec<u8>,
    },

    #[tl_id = "b16a6c29"]
    RequestPhone {
        text: String,
    },

    #[tl_id = "fc796b3f"]
    RequestGeoLocation {
        text: String,
    },

    #[tl_id = "568a748"]
    SwitchInline {
        flags: u32,
        same_peer: bool,
        text: String,
        query: String,
    },

    #[tl_id = "50f41ccf"]
    Game {
        text: String,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "77608b83"]
struct KeyboardButtonRow {
    buttons: Vec<KeyboardButton>,
}

#[derive(Debug, TlType)]
enum ReplyMarkup {
    #[tl_id = "a03e5b85"]
    KeyboardHide {
        flags: u32,
        #[flag_bit = "2"]
        selective: Option<bool>,
    },

    #[tl_id = "f4108aa0"]
    KeyboardForceReply {
        flags: u32,
        #[flag_bit = "1"]
        single_use: Option<bool>,
        #[flag_bit = "2"]
        selective: Option<bool>,
    },

    #[tl_id = "3502758c"]
    Keyboard {
        flags: u32,
        resize: bool,
        #[flag_bit = "1"]
        single_use: Option<bool>,
        #[flag_bit = "2"]
        selective: Option<bool>,
        rows: Vec<KeyboardButtonRow>,
    },

    #[tl_id = "48a30254"]
    Inline {
        rows: Vec<KeyboardButtonRow>,
    },
}

#[derive(Debug, TlType)]
enum HelpAppChangelog {
    #[tl_id = "af7e0394"]
    Empty,

    #[tl_id = "2a137e7c"]
    Changelog {
        message: String,
        media: Box<MessageMedia>,
        entities: Vec<MessageEntity>,
    },
}

#[derive(Debug, TlType)]
enum MessageEntity {
    #[tl_id = "bb92ba95"]
    Unknown {
        offset: i32,
        length: i32,
    },

    #[tl_id = "fa04579d"]
    Mention {
        offset: i32,
        length: i32,
    },

    #[tl_id = "6f635b0d"]
    Hashtag {
        offset: i32,
        length: i32,
    },

    #[tl_id = "6cef8ac7"]
    BotCommand {
        offset: i32,
        length: i32,
    },

    #[tl_id = "6ed02538"]
    Url {
        offset: i32,
        length: i32,
    },

    #[tl_id = "64e475c2"]
    Email {
        offset: i32,
        length: i32,
    },

    #[tl_id = "bd610bc9"]
    Bold {
        offset: i32,
        length: i32,
    },

    #[tl_id = "826f8b60"]
    Italic {
        offset: i32,
        length: i32,
    },

    #[tl_id = "28a20571"]
    Code {
        offset: i32,
        length: i32,
    },

    #[tl_id = "73924be0"]
    Pre {
        offset: i32,
        length: i32,
        language: String,
    },

    #[tl_id = "76a6d327"]
    TextUrl {
        offset: i32,
        length: i32,
        url: String,
    },

    #[tl_id = "352dca58"]
    MentionName {
        offset: i32,
        length: i32,
        user_id: i32,
    },

    #[tl_id = "208e68c9"]
    InputMentionName {
        offset: i32,
        length: i32,
        user_id: Box<InputUser>,
    },
}

#[derive(Debug, TlType)]
enum InputChannel {
    #[tl_id = "ee8c1e86"]
    Empty,

    #[tl_id = "afeb712e"]
    Channel {
        channel_id: i32,
        access_hash: i64,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "7f077ad9"]
struct ContactsResolvedPeer {
    peer: Box<Peer>,
    chats: Vec<Chat>,
    users: Vec<User>,
}

#[derive(Debug, TlType)]
#[tl_id = "ae30253"]
struct MessageRange {
    min_id: i32,
    max_id: i32,
}

#[derive(Debug, TlType)]
enum UpdatesChannelDifference {
    #[tl_id = "3e11affb"]
    Empty {
        flags: u32,
        ffinal: bool,
        pts: i32,
        #[flag_bit = "1"]
        timeout: Option<i32>,
    },

    #[tl_id = "410dee07"]
    TooLong {
        flags: u32,
        ffinal: bool,
        pts: i32,
        #[flag_bit = "1"]
        timeout: Option<i32>,
        top_message: i32,
        read_inbox_max_id: i32,
        read_outbox_max_id: i32,
        unread_count: i32,
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },

    #[tl_id = "2064674e"]
    Difference {
        flags: u32,
        ffinal: bool,
        pts: i32,
        #[flag_bit = "1"]
        timeout: Option<i32>,
        new_messages: Vec<Message>,
        other_updates: Vec<Update>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },
}

#[derive(Debug, TlType)]
enum ChannelMessagesFilter {
    #[tl_id = "94d42ee7"]
    Empty,

    #[tl_id = "cd77d957"]
    Filter {
        flags: u32,
        #[flag_bit = "1"]
        exclude_new_messages: Option<bool>,
        ranges: Vec<MessageRange>,
    },
}

#[derive(Debug, TlType)]
enum ChannelParticipant {
    #[tl_id = "15ebac1d"]
    Participant {
        user_id: i32,
        date: i32,
    },

    #[tl_id = "a3289a6d"]
    SSelf {
        user_id: i32,
        inviter_id: i32,
        date: i32,
    },

    #[tl_id = "91057fef"]
    Moderator {
        user_id: i32,
        inviter_id: i32,
        date: i32,
    },

    #[tl_id = "98192d61"]
    Editor {
        user_id: i32,
        inviter_id: i32,
        date: i32,
    },

    #[tl_id = "8cc5e69a"]
    Kicked {
        user_id: i32,
        kicked_by: i32,
        date: i32,
    },

    #[tl_id = "e3e2e1f9"]
    Creator {
        user_id: i32,
    },
}

#[derive(Debug, TlType)]
enum ChannelParticipantsFilter {
    #[tl_id = "de3f3c79"]
    Recent,

    #[tl_id = "b4608969"]
    Admins,

    #[tl_id = "3c37bb7a"]
    Kicked,

    #[tl_id = "b0d1865b"]
    Bots,
}

#[derive(Debug, TlType)]
enum ChannelParticipantRole {
    #[tl_id = "b285a0c6"]
    RoleEmpty,

    #[tl_id = "9618d975"]
    RoleModerator,

    #[tl_id = "820bfe8c"]
    RoleEditor,
}

#[derive(Debug, TlType)]
#[tl_id = "f56ee2a8"]
struct ChannelsChannelParticipants {
    count: i32,
    participants: Vec<ChannelParticipant>,
    users: Vec<User>,
}

#[derive(Debug, TlType)]
#[tl_id = "d0d9b163"]
struct ChannelsChannelParticipant {
    participant: Box<ChannelParticipant>,
    users: Vec<User>,
}

#[derive(Debug, TlType)]
#[tl_id = "f1ee3e90"]
struct HelpTermsOfService {
    text: String,
}

#[derive(Debug, TlType)]
enum FoundGif {
    #[tl_id = "162ecc1f"]
    Gif {
        url: String,
        thumb_url: String,
        content_url: String,
        content_type: String,
        w: i32,
        h: i32,
    },

    #[tl_id = "9c750409"]
    Cached {
        url: String,
        photo: Box<Photo>,
        document: Box<Document>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "450a1c0a"]
struct MessagesFoundGifs {
    next_offset: i32,
    results: Vec<FoundGif>,
}

#[derive(Debug, TlType)]
enum MessagesSavedGifs {
    #[tl_id = "e8025ca2"]
    NotModified,

    #[tl_id = "2e0709a5"]
    Gifs {
        hash: i32,
        gifs: Vec<Document>,
    },
}

#[derive(Debug, TlType)]
enum InputBotInlineMessage {
    #[tl_id = "292fed13"]
    MediaAuto {
        flags: u32,
        caption: String,
        #[flag_bit = "2"]
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    #[tl_id = "3dcd7a87"]
    Text {
        flags: u32,
        no_webpage: bool,
        message: String,
        #[flag_bit = "1"]
        entities: Option<Vec<MessageEntity>>,
        #[flag_bit = "2"]
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    #[tl_id = "f4a59de1"]
    MediaGeo {
        flags: u32,
        geo_point: Box<InputGeoPoint>,
        #[flag_bit = "2"]
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    #[tl_id = "aaafadc8"]
    MediaVenue {
        flags: u32,
        geo_point: Box<InputGeoPoint>,
        title: String,
        address: String,
        provider: String,
        venue_id: String,
        #[flag_bit = "2"]
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    #[tl_id = "2daf01a7"]
    MediaContact {
        flags: u32,
        phone_number: String,
        first_name: String,
        last_name: String,
        #[flag_bit = "2"]
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    #[tl_id = "4b425864"]
    Game {
        flags: u32,
        #[flag_bit = "2"]
        reply_markup: Option<Box<ReplyMarkup>>,
    },
}

#[derive(Debug, TlType)]
enum InputBotInlineResult {
    #[tl_id = "2cbbe15a"]
    Result {
        flags: u32,
        id: String,
        ttype: String,
        #[flag_bit = "1"]
        title: Option<String>,
        #[flag_bit = "2"]
        description: Option<String>,
        #[flag_bit = "3"]
        url: Option<String>,
        #[flag_bit = "4"]
        thumb_url: Option<String>,
        #[flag_bit = "5"]
        content_url: Option<String>,
        #[flag_bit = "5"]
        content_type: Option<String>,
        #[flag_bit = "6"]
        w: Option<i32>,
        #[flag_bit = "6"]
        h: Option<i32>,
        #[flag_bit = "7"]
        duration: Option<i32>,
        send_message: Box<InputBotInlineMessage>,
    },

    #[tl_id = "a8d864a7"]
    Photo {
        id: String,
        ttype: String,
        photo: Box<InputPhoto>,
        send_message: Box<InputBotInlineMessage>,
    },

    #[tl_id = "fff8fdc4"]
    Document {
        flags: u32,
        id: String,
        ttype: String,
        #[flag_bit = "1"]
        title: Option<String>,
        #[flag_bit = "2"]
        description: Option<String>,
        document: Box<InputDocument>,
        send_message: Box<InputBotInlineMessage>,
    },

    #[tl_id = "4fa417f2"]
    Game {
        id: String,
        short_name: String,
        send_message: Box<InputBotInlineMessage>,
    },
}

#[derive(Debug, TlType)]
enum BotInlineMessage {
    #[tl_id = "a74b15b"]
    MediaAuto {
        flags: u32,
        caption: String,
        #[flag_bit = "2"]
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    #[tl_id = "8c7f65e2"]
    Text {
        flags: u32,
        no_webpage: bool,
        message: String,
        #[flag_bit = "1"]
        entities: Option<Vec<MessageEntity>>,
        #[flag_bit = "2"]
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    #[tl_id = "3a8fd8b8"]
    MediaGeo {
        flags: u32,
        geo: Box<GeoPoint>,
        #[flag_bit = "2"]
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    #[tl_id = "4366232e"]
    MediaVenue {
        flags: u32,
        geo: Box<GeoPoint>,
        title: String,
        address: String,
        provider: String,
        venue_id: String,
        #[flag_bit = "2"]
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    #[tl_id = "35edb4d4"]
    MediaContact {
        flags: u32,
        phone_number: String,
        first_name: String,
        last_name: String,
        #[flag_bit = "2"]
        reply_markup: Option<Box<ReplyMarkup>>,
    },
}

#[derive(Debug, TlType)]
enum BotInlineResult {
    #[tl_id = "9bebaeb9"]
    Result {
        flags: u32,
        id: String,
        ttype: String,
        #[flag_bit = "1"]
        title: Option<String>,
        #[flag_bit = "2"]
        description: Option<String>,
        #[flag_bit = "3"]
        url: Option<String>,
        #[flag_bit = "4"]
        thumb_url: Option<String>,
        #[flag_bit = "5"]
        content_url: Option<String>,
        #[flag_bit = "5"]
        content_type: Option<String>,
        #[flag_bit = "6"]
        w: Option<i32>,
        #[flag_bit = "6"]
        h: Option<i32>,
        #[flag_bit = "7"]
        duration: Option<i32>,
        send_message: Box<BotInlineMessage>,
    },

    #[tl_id = "17db940b"]
    Media {
        flags: u32,
        id: String,
        ttype: String,
        photo: Box<Photo>,
        #[flag_bit = "1"]
        document: Option<Box<Document>>,
        #[flag_bit = "2"]
        title: Option<String>,
        #[flag_bit = "3"]
        description: Option<String>,
        send_message: Box<BotInlineMessage>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "ccd3563d"]
struct MessagesBotResults {
    flags: u32,
    gallery: bool,
    query_id: i64,
    #[flag_bit = "1"]
    next_offset: Option<String>,
    #[flag_bit = "2"]
    switch_pm: Option<Box<InlineBotSwitchPM>>,
    results: Vec<BotInlineResult>,
    cache_time: i32,
}

#[derive(Debug, TlType)]
#[tl_id = "1f486803"]
struct ExportedMessageLink {
    link: String,
}

#[derive(Debug, TlType)]
#[tl_id = "c786ddcb"]
struct MessageFwdHeader {
    flags: u32,
    from_id: i32,
    date: i32,
    #[flag_bit = "1"]
    channel_id: Option<i32>,
    #[flag_bit = "2"]
    channel_post: Option<i32>,
}

#[derive(Debug, TlType)]
enum AuthCodeType {
    #[tl_id = "72a3158c"]
    Sms,

    #[tl_id = "741cd3e3"]
    Call,

    #[tl_id = "226ccefb"]
    FlashCall,
}

#[derive(Debug, TlType)]
enum AuthSentCodeType {
    #[tl_id = "3dbb5986"]
    App {
        length: i32,
    },

    #[tl_id = "c000bba2"]
    Sms {
        length: i32,
    },

    #[tl_id = "5353e5a7"]
    Call {
        length: i32,
    },

    #[tl_id = "ab03c6d9"]
    FlashCall {
        pattern: String,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "36585ea4"]
struct MessagesBotCallbackAnswer {
    flags: u32,
    #[flag_bit = "1"]
    alert: Option<bool>,
    #[flag_bit = "3"]
    has_url: Option<bool>,
    message: String,
    #[flag_bit = "2"]
    url: Option<String>,
    cache_time: i32,
}

#[derive(Debug, TlType)]
#[tl_id = "26b5dde6"]
struct MessagesMessageEditData {
    flags: u32,
    caption: bool,
}

#[derive(Debug, TlType)]
#[tl_id = "890c3d89"]
struct InputBotInlineMessageID {
    dc_id: i32,
    id: i64,
    access_hash: i64,
}

#[derive(Debug, TlType)]
#[tl_id = "3c20629f"]
struct InlineBotSwitchPM {
    text: String,
    start_param: String,
}

#[derive(Debug, TlType)]
#[tl_id = "3371c354"]
struct MessagesPeerDialogs {
    dialogs: Vec<Dialog>,
    messages: Vec<Message>,
    chats: Vec<Chat>,
    users: Vec<User>,
    state: Box<UpdatesState>,
}

#[derive(Debug, TlType)]
#[tl_id = "edcdc05b"]
struct TopPeer {
    peer: Box<Peer>,
    rating: f64,
}

#[derive(Debug, TlType)]
enum TopPeerCategory {
    #[tl_id = "ab661b5b"]
    BotsPM,

    #[tl_id = "148677e2"]
    BotsInline,

    #[tl_id = "637b7ed"]
    Correspondents,

    #[tl_id = "bd17a14a"]
    Groups,

    #[tl_id = "161d9628"]
    Channels,
}

#[derive(Debug, TlType)]
#[tl_id = "fb834291"]
struct TopPeerCategoryPeers {
    category: Box<TopPeerCategory>,
    count: i32,
    peers: Vec<TopPeer>,
}

#[derive(Debug, TlType)]
enum ContactsTopPeers {
    #[tl_id = "de266ef5"]
    NotModified,

    #[tl_id = "70b772a8"]
    Peers {
        categories: Vec<TopPeerCategoryPeers>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },
}

#[derive(Debug, TlType)]
enum DraftMessage {
    #[tl_id = "ba4baec5"]
    Empty,

    #[tl_id = "fd8e711f"]
    Message {
        flags: u32,
        #[flag_bit = "1"]
        no_webpage: Option<bool>,
        reply_to_msg_id: i32,
        message: String,
        #[flag_bit = "3"]
        entities: Option<Vec<MessageEntity>>,
        date: i32,
    },
}

#[derive(Debug, TlType)]
enum MessagesFeaturedStickers {
    #[tl_id = "4ede3cf"]
    NotModified,

    #[tl_id = "f89d88e5"]
    Stickers {
        hash: i32,
        sets: Vec<StickerSetCovered>,
        unread: Vec<i64>,
    },
}

#[derive(Debug, TlType)]
enum MessagesRecentStickers {
    #[tl_id = "b17f890"]
    NotModified,

    #[tl_id = "5ce20970"]
    Stickers {
        hash: i32,
        stickers: Vec<Document>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "4fcba9c8"]
struct MessagesArchivedStickers {
    count: i32,
    sets: Vec<StickerSetCovered>,
}

#[derive(Debug, TlType)]
enum MessagesStickerSetInstallResult {
    #[tl_id = "38641628"]
    Success,

    #[tl_id = "35e410a8"]
    Archive {
        sets: Vec<StickerSetCovered>,
    },
}

#[derive(Debug, TlType)]
enum StickerSetCovered {
    #[tl_id = "6410a5d2"]
    Covered {
        set: Box<StickerSet>,
        cover: Box<Document>,
    },

    #[tl_id = "3407e51b"]
    Multi {
        set: Box<StickerSet>,
        covers: Vec<Document>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "aed6dbb2"]
struct MaskCoords {
    n: i32,
    x: f64,
    y: f64,
    zoom: f64,
}

#[derive(Debug, TlType)]
enum InputStickeredMedia {
    #[tl_id = "4a992157"]
    Photo {
        id: Box<InputPhoto>,
    },

    #[tl_id = "438865b"]
    Document {
        id: Box<InputDocument>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "bdf9653b"]
struct Game {
    flags: u32,
    id: i64,
    access_hash: i64,
    short_name: String,
    title: String,
    description: String,
    photo: Box<Photo>,
    document: Box<Document>,
}

#[derive(Debug, TlType)]
enum InputGame {
    #[tl_id = "32c3e77"]
    ID {
        id: i64,
        access_hash: i64,
    },

    #[tl_id = "c331e80a"]
    ShortName {
        bot_id: Box<InputUser>,
        short_name: String,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "58fffcd0"]
struct HighScore {
    pos: i32,
    user_id: i32,
    score: i32,
}

#[derive(Debug, TlType)]
#[tl_id = "9a3bfd99"]
struct MessagesHighScores {
    scores: Vec<HighScore>,
    users: Vec<User>,
}

#[derive(Debug, TlType)]
enum RichText {
    #[tl_id = "dc3d824f"]
    Empty,

    #[tl_id = "744694e0"]
    Plain {
        text: String,
    },

    #[tl_id = "6724abc4"]
    Bold {
        text: Box<RichText>,
    },

    #[tl_id = "d912a59c"]
    Italic {
        text: Box<RichText>,
    },

    #[tl_id = "c12622c4"]
    Underline {
        text: Box<RichText>,
    },

    #[tl_id = "9bf8bb95"]
    Strike {
        text: Box<RichText>,
    },

    #[tl_id = "6c3f19b9"]
    Fixed {
        text: Box<RichText>,
    },

    #[tl_id = "3c2884c1"]
    Url {
        text: Box<RichText>,
        url: String,
        webpage_id: i64,
    },

    #[tl_id = "de5a0dd6"]
    Email {
        text: Box<RichText>,
        email: String,
    },

    #[tl_id = "7e6260d7"]
    Concat {
        texts: Vec<RichText>,
    },
}

#[derive(Debug, TlType)]
enum PageBlock {
    #[tl_id = "13567e8a"]
    Unsupported,

    #[tl_id = "70abc3fd"]
    Title {
        text: Box<RichText>,
    },

    #[tl_id = "8ffa9a1f"]
    Subtitle {
        text: Box<RichText>,
    },

    #[tl_id = "baafe5e0"]
    AuthorDate {
        author: Box<RichText>,
        published_date: i32,
    },

    #[tl_id = "bfd064ec"]
    Header {
        text: Box<RichText>,
    },

    #[tl_id = "f12bb6e1"]
    Subheader {
        text: Box<RichText>,
    },

    #[tl_id = "467a0766"]
    Paragraph {
        text: Box<RichText>,
    },

    #[tl_id = "c070d93e"]
    Preformatted {
        text: Box<RichText>,
        language: String,
    },

    #[tl_id = "48870999"]
    Footer {
        text: Box<RichText>,
    },

    #[tl_id = "db20b188"]
    Divider,

    #[tl_id = "ce0d37b0"]
    Anchor {
        name: String,
    },

    #[tl_id = "3a58c7f4"]
    List {
        ordered: bool,
        items: Vec<RichText>,
    },

    #[tl_id = "263d7c26"]
    Blockquote {
        text: Box<RichText>,
        caption: Box<RichText>,
    },

    #[tl_id = "4f4456d3"]
    Pullquote {
        text: Box<RichText>,
        caption: Box<RichText>,
    },

    #[tl_id = "e9c69982"]
    Photo {
        photo_id: i64,
        caption: Box<RichText>,
    },

    #[tl_id = "d9d71866"]
    Video {
        flags: u32,
        autoplay: bool,
        #[flag_bit = "1"]
        lloop: Option<bool>,
        video_id: i64,
        caption: Box<RichText>,
    },

    #[tl_id = "39f23300"]
    Cover {
        cover: Box<PageBlock>,
    },

    #[tl_id = "cde200d1"]
    Embed {
        flags: u32,
        full_width: bool,
        #[flag_bit = "3"]
        allow_scrolling: Option<bool>,
        #[flag_bit = "1"]
        url: Option<String>,
        #[flag_bit = "2"]
        html: Option<String>,
        #[flag_bit = "4"]
        poster_photo_id: Option<i64>,
        w: i32,
        h: i32,
        caption: Box<RichText>,
    },

    #[tl_id = "292c7be9"]
    EmbedPost {
        url: String,
        webpage_id: i64,
        author_photo_id: i64,
        author: String,
        date: i32,
        blocks: Vec<PageBlock>,
        caption: Box<RichText>,
    },

    #[tl_id = "8b31c4f"]
    Collage {
        items: Vec<PageBlock>,
        caption: Box<RichText>,
    },

    #[tl_id = "130c8963"]
    Slideshow {
        items: Vec<PageBlock>,
        caption: Box<RichText>,
    },
}

#[derive(Debug, TlType)]
enum Page {
    #[tl_id = "8dee6c44"]
    Part {
        blocks: Vec<PageBlock>,
        photos: Vec<Photo>,
        videos: Vec<Document>,
    },

    #[tl_id = "d7a19d69"]
    Full {
        blocks: Vec<PageBlock>,
        photos: Vec<Photo>,
        videos: Vec<Document>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "1e36fded"]
struct InputPhoneCall {
    id: i64,
    access_hash: i64,
}

#[derive(Debug, TlType)]
enum PhoneCall {
    #[tl_id = "5366c915"]
    Empty {
        id: i64,
    },

    #[tl_id = "1b8f4ad1"]
    Waiting {
        flags: u32,
        id: i64,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
        protocol: Box<PhoneCallProtocol>,
        receive_date: i32,
    },

    #[tl_id = "6c448ae8"]
    Requested {
        id: i64,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
        g_a: Vec<u8>,
        protocol: Box<PhoneCallProtocol>,
    },

    #[tl_id = "ffe6ab67"]
    Call {
        id: i64,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
        g_a_or_b: Vec<u8>,
        key_fingerprint: i64,
        protocol: Box<PhoneCallProtocol>,
        connection: Box<PhoneConnection>,
        alternative_connections: Vec<PhoneConnection>,
        start_date: i32,
    },

    #[tl_id = "50ca4de1"]
    Discarded {
        flags: u32,
        id: i64,
        reason: Box<PhoneCallDiscardReason>,
        #[flag_bit = "1"]
        duration: Option<i32>,
    },
}

#[derive(Debug, TlType)]
#[tl_id = "9d4c17c0"]
struct PhoneConnection {
    id: i64,
    ip: String,
    ipv6: String,
    port: i32,
    peer_tag: Vec<u8>,
}

#[derive(Debug, TlType)]
#[tl_id = "a2bb35cb"]
struct PhoneCallProtocol {
    flags: u32,
    udp_p2p: bool,
    #[flag_bit = "1"]
    udp_reflector: Option<bool>,
    min_layer: i32,
    max_layer: i32,
}

#[derive(Debug, TlType)]
#[tl_id = "ec82e140"]
struct PhonePhoneCall {
    phone_call: Box<PhoneCall>,
    users: Vec<User>,
}

#[derive(Debug, TlType)]
enum PhoneCallDiscardReason {
    #[tl_id = "85e42301"]
    Missed,

    #[tl_id = "e095c1a0"]
    Disconnect,

    #[tl_id = "57adc690"]
    Hangup,

    #[tl_id = "faf7e8c9"]
    Busy,
}

#[derive(Debug)]
enum TlType {
    Bool(Box<Bool>),
    True(Box<True>),
    Error(Box<Error>),
    Null(Box<Null>),
    InputPeer(Box<InputPeer>),
    InputUser(Box<InputUser>),
    InputContact(Box<InputContact>),
    InputFile(Box<InputFile>),
    InputMedia(Box<InputMedia>),
    InputChatPhoto(Box<InputChatPhoto>),
    InputGeoPoint(Box<InputGeoPoint>),
    InputPhoto(Box<InputPhoto>),
    InputFileLocation(Box<InputFileLocation>),
    InputAppEvent(Box<InputAppEvent>),
    Peer(Box<Peer>),
    StorageFileType(Box<StorageFileType>),
    FileLocation(Box<FileLocation>),
    User(Box<User>),
    UserProfilePhoto(Box<UserProfilePhoto>),
    UserStatus(Box<UserStatus>),
    Chat(Box<Chat>),
    ChatFull(Box<ChatFull>),
    ChatParticipant(Box<ChatParticipant>),
    ChatParticipants(Box<ChatParticipants>),
    ChatPhoto(Box<ChatPhoto>),
    Message(Box<Message>),
    MessageMedia(Box<MessageMedia>),
    MessageAction(Box<MessageAction>),
    Dialog(Box<Dialog>),
    Photo(Box<Photo>),
    PhotoSize(Box<PhotoSize>),
    GeoPoint(Box<GeoPoint>),
    AuthCheckedPhone(Box<AuthCheckedPhone>),
    AuthSentCode(Box<AuthSentCode>),
    AuthAuthorization(Box<AuthAuthorization>),
    AuthExportedAuthorization(Box<AuthExportedAuthorization>),
    InputNotifyPeer(Box<InputNotifyPeer>),
    InputPeerNotifyEvents(Box<InputPeerNotifyEvents>),
    InputPeerNotifySettings(Box<InputPeerNotifySettings>),
    PeerNotifyEvents(Box<PeerNotifyEvents>),
    PeerNotifySettings(Box<PeerNotifySettings>),
    PeerSettings(Box<PeerSettings>),
    WallPaper(Box<WallPaper>),
    ReportReason(Box<ReportReason>),
    UserFull(Box<UserFull>),
    Contact(Box<Contact>),
    ImportedContact(Box<ImportedContact>),
    ContactBlocked(Box<ContactBlocked>),
    ContactStatus(Box<ContactStatus>),
    ContactsLink(Box<ContactsLink>),
    ContactsContacts(Box<ContactsContacts>),
    ContactsImportedContacts(Box<ContactsImportedContacts>),
    ContactsBlocked(Box<ContactsBlocked>),
    MessagesDialogs(Box<MessagesDialogs>),
    MessagesMessages(Box<MessagesMessages>),
    MessagesChats(Box<MessagesChats>),
    MessagesChatFull(Box<MessagesChatFull>),
    MessagesAffectedHistory(Box<MessagesAffectedHistory>),
    MessagesFilter(Box<MessagesFilter>),
    Update(Box<Update>),
    UpdatesState(Box<UpdatesState>),
    UpdatesDifference(Box<UpdatesDifference>),
    Updates(Box<Updates>),
    PhotosPhotos(Box<PhotosPhotos>),
    PhotosPhoto(Box<PhotosPhoto>),
    UploadFile(Box<UploadFile>),
    DcOption(Box<DcOption>),
    Config(Box<Config>),
    NearestDc(Box<NearestDc>),
    HelpAppUpdate(Box<HelpAppUpdate>),
    HelpInviteText(Box<HelpInviteText>),
    EncryptedChat(Box<EncryptedChat>),
    InputEncryptedChat(Box<InputEncryptedChat>),
    EncryptedFile(Box<EncryptedFile>),
    InputEncryptedFile(Box<InputEncryptedFile>),
    EncryptedMessage(Box<EncryptedMessage>),
    MessagesDhConfig(Box<MessagesDhConfig>),
    MessagesSentEncryptedMessage(Box<MessagesSentEncryptedMessage>),
    InputDocument(Box<InputDocument>),
    Document(Box<Document>),
    HelpSupport(Box<HelpSupport>),
    NotifyPeer(Box<NotifyPeer>),
    SendMessageAction(Box<SendMessageAction>),
    ContactsFound(Box<ContactsFound>),
    InputPrivacyKey(Box<InputPrivacyKey>),
    PrivacyKey(Box<PrivacyKey>),
    InputPrivacyRule(Box<InputPrivacyRule>),
    PrivacyRule(Box<PrivacyRule>),
    AccountPrivacyRules(Box<AccountPrivacyRules>),
    AccountDaysTTL(Box<AccountDaysTTL>),
    DocumentAttribute(Box<DocumentAttribute>),
    MessagesStickers(Box<MessagesStickers>),
    StickerPack(Box<StickerPack>),
    MessagesAllStickers(Box<MessagesAllStickers>),
    DisabledFeature(Box<DisabledFeature>),
    MessagesAffectedMessages(Box<MessagesAffectedMessages>),
    ContactLink(Box<ContactLink>),
    WebPage(Box<WebPage>),
    Authorization(Box<Authorization>),
    AccountAuthorizations(Box<AccountAuthorizations>),
    AccountPassword(Box<AccountPassword>),
    AccountPasswordSettings(Box<AccountPasswordSettings>),
    AccountPasswordInputSettings(Box<AccountPasswordInputSettings>),
    AuthPasswordRecovery(Box<AuthPasswordRecovery>),
    ReceivedNotifyMessage(Box<ReceivedNotifyMessage>),
    ExportedChatInvite(Box<ExportedChatInvite>),
    ChatInvite(Box<ChatInvite>),
    InputStickerSet(Box<InputStickerSet>),
    StickerSet(Box<StickerSet>),
    MessagesStickerSet(Box<MessagesStickerSet>),
    BotCommand(Box<BotCommand>),
    BotInfo(Box<BotInfo>),
    KeyboardButton(Box<KeyboardButton>),
    KeyboardButtonRow(Box<KeyboardButtonRow>),
    ReplyMarkup(Box<ReplyMarkup>),
    HelpAppChangelog(Box<HelpAppChangelog>),
    MessageEntity(Box<MessageEntity>),
    InputChannel(Box<InputChannel>),
    ContactsResolvedPeer(Box<ContactsResolvedPeer>),
    MessageRange(Box<MessageRange>),
    UpdatesChannelDifference(Box<UpdatesChannelDifference>),
    ChannelMessagesFilter(Box<ChannelMessagesFilter>),
    ChannelParticipant(Box<ChannelParticipant>),
    ChannelParticipantsFilter(Box<ChannelParticipantsFilter>),
    ChannelParticipantRole(Box<ChannelParticipantRole>),
    ChannelsChannelParticipants(Box<ChannelsChannelParticipants>),
    ChannelsChannelParticipant(Box<ChannelsChannelParticipant>),
    HelpTermsOfService(Box<HelpTermsOfService>),
    FoundGif(Box<FoundGif>),
    MessagesFoundGifs(Box<MessagesFoundGifs>),
    MessagesSavedGifs(Box<MessagesSavedGifs>),
    InputBotInlineMessage(Box<InputBotInlineMessage>),
    InputBotInlineResult(Box<InputBotInlineResult>),
    BotInlineMessage(Box<BotInlineMessage>),
    BotInlineResult(Box<BotInlineResult>),
    MessagesBotResults(Box<MessagesBotResults>),
    ExportedMessageLink(Box<ExportedMessageLink>),
    MessageFwdHeader(Box<MessageFwdHeader>),
    AuthCodeType(Box<AuthCodeType>),
    AuthSentCodeType(Box<AuthSentCodeType>),
    MessagesBotCallbackAnswer(Box<MessagesBotCallbackAnswer>),
    MessagesMessageEditData(Box<MessagesMessageEditData>),
    InputBotInlineMessageID(Box<InputBotInlineMessageID>),
    InlineBotSwitchPM(Box<InlineBotSwitchPM>),
    MessagesPeerDialogs(Box<MessagesPeerDialogs>),
    TopPeer(Box<TopPeer>),
    TopPeerCategory(Box<TopPeerCategory>),
    TopPeerCategoryPeers(Box<TopPeerCategoryPeers>),
    ContactsTopPeers(Box<ContactsTopPeers>),
    DraftMessage(Box<DraftMessage>),
    MessagesFeaturedStickers(Box<MessagesFeaturedStickers>),
    MessagesRecentStickers(Box<MessagesRecentStickers>),
    MessagesArchivedStickers(Box<MessagesArchivedStickers>),
    MessagesStickerSetInstallResult(Box<MessagesStickerSetInstallResult>),
    StickerSetCovered(Box<StickerSetCovered>),
    MaskCoords(Box<MaskCoords>),
    InputStickeredMedia(Box<InputStickeredMedia>),
    Game(Box<Game>),
    InputGame(Box<InputGame>),
    HighScore(Box<HighScore>),
    MessagesHighScores(Box<MessagesHighScores>),
    RichText(Box<RichText>),
    PageBlock(Box<PageBlock>),
    Page(Box<Page>),
    InputPhoneCall(Box<InputPhoneCall>),
    PhoneCall(Box<PhoneCall>),
    PhoneConnection(Box<PhoneConnection>),
    PhoneCallProtocol(Box<PhoneCallProtocol>),
    PhonePhoneCall(Box<PhonePhoneCall>),
    PhoneCallDiscardReason(Box<PhoneCallDiscardReason>),
}