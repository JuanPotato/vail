#[derive(Debug)]
enum Bool {
    False,

    True,
}

#[derive(Debug)]
struct True;

#[derive(Debug)]
struct Error {
    code: i32,
    text: String,
}

#[derive(Debug)]
struct Null;

#[derive(Debug)]
enum InputPeer {
    Empty,

    SSelf,

    Chat {
        chat_id: i32,
    },

    User {
        user_id: i32,
        access_hash: i64,
    },

    Channel {
        channel_id: i32,
        access_hash: i64,
    },
}

#[derive(Debug)]
enum InputUser {
    Empty,

    SSelf,

    User {
        user_id: i32,
        access_hash: i64,
    },
}

#[derive(Debug)]
enum InputContact {
    Phone {
        client_id: i64,
        phone: String,
        first_name: String,
        last_name: String,
    },
}

#[derive(Debug)]
enum InputFile {
    File {
        id: i64,
        parts: i32,
        name: String,
        md5_checksum: String,
    },

    Big {
        id: i64,
        parts: i32,
        name: String,
    },
}

#[derive(Debug)]
enum InputMedia {
    Empty,

    UploadedPhoto {
        flags: u32,
        file: InputFile,
        caption: String,
        stickers: Vec<InputDocument>,
    },

    Photo {
        id: InputPhoto,
        caption: String,
    },

    GeoPoint {
        geo_point: InputGeoPoint,
    },

    Contact {
        phone_number: String,
        first_name: String,
        last_name: String,
    },

    UploadedDocument {
        flags: u32,
        file: InputFile,
        mime_type: String,
        attributes: Vec<DocumentAttribute>,
        caption: String,
        stickers: Vec<InputDocument>,
    },

    UploadedThumbDocument {
        flags: u32,
        file: InputFile,
        thumb: InputFile,
        mime_type: String,
        attributes: Vec<DocumentAttribute>,
        caption: String,
        stickers: Vec<InputDocument>,
    },

    Document {
        id: InputDocument,
        caption: String,
    },

    Venue {
        geo_point: InputGeoPoint,
        title: String,
        address: String,
        provider: String,
        venue_id: String,
    },

    GifExternal {
        url: String,
        q: String,
    },

    PhotoExternal {
        url: String,
        caption: String,
    },

    DocumentExternal {
        url: String,
        caption: String,
    },

    Game {
        id: InputGame,
    },
}

#[derive(Debug)]
enum InputChatPhoto {
    Empty,

    Uploaded {
        file: InputFile,
    },

    Photo {
        id: InputPhoto,
    },
}

#[derive(Debug)]
enum InputGeoPoint {
    Empty,

    Point {
        lat: f64,
        long: f64,
    },
}

#[derive(Debug)]
enum InputPhoto {
    Empty,

    Photo {
        id: i64,
        access_hash: i64,
    },
}

#[derive(Debug)]
enum InputFileLocation {
    Location {
        volume_id: i64,
        local_id: i32,
        secret: i64,
    },

    Encrypted {
        id: i64,
        access_hash: i64,
    },

    Document {
        id: i64,
        access_hash: i64,
        version: i32,
    },
}

#[derive(Debug)]
struct InputAppEvent {
    time: f64,
    ttype: String,
    peer: i64,
    data: String,
}

#[derive(Debug)]
enum Peer {
    User {
        user_id: i32,
    },

    Chat {
        chat_id: i32,
    },

    Channel {
        channel_id: i32,
    },
}

#[derive(Debug)]
enum StorageFileType {
    Unknown,

    Jpeg,

    Gif,

    Png,

    Pdf,

    Mp3,

    Mov,

    Partial,

    Mp4,

    Webp,
}

#[derive(Debug)]
enum FileLocation {
    Unavailable {
        volume_id: i64,
        local_id: i32,
        secret: i64,
    },

    Location {
        dc_id: i32,
        volume_id: i64,
        local_id: i32,
        secret: i64,
    },
}

#[derive(Debug)]
enum User {
    Empty {
        id: i32,
    },

    User {
        flags: u32,
        sself: bool,
        contact: bool,
        mutual_contact: bool,
        deleted: bool,
        bot: bool,
        bot_chat_history: bool,
        bot_nochats: bool,
        verified: bool,
        restricted: bool,
        min: bool,
        bot_inline_geo: bool,
        id: i32,
        access_hash: i64,
        first_name: String,
        last_name: String,
        username: String,
        phone: String,
        photo: UserProfilePhoto,
        status: UserStatus,
        bot_info_version: i32,
        restriction_reason: String,
        bot_inline_placeholder: String,
    },
}

#[derive(Debug)]
enum UserProfilePhoto {
    Empty,

    Photo {
        photo_id: i64,
        photo_small: FileLocation,
        photo_big: FileLocation,
    },
}

#[derive(Debug)]
enum UserStatus {
    Empty,

    Online {
        expires: i32,
    },

    Offline {
        was_online: i32,
    },

    Recently,

    LastWeek,

    LastMonth,
}

#[derive(Debug)]
enum Chat {
    Empty {
        id: i32,
    },

    Chat {
        flags: u32,
        creator: bool,
        kicked: bool,
        left: bool,
        admins_enabled: bool,
        admin: bool,
        deactivated: bool,
        id: i32,
        title: String,
        photo: ChatPhoto,
        participants_count: i32,
        date: i32,
        version: i32,
        migrated_to: InputChannel,
    },

    Forbidden {
        id: i32,
        title: String,
    },

    nnel {
        flags: u32,
        creator: bool,
        kicked: bool,
        left: bool,
        editor: bool,
        moderator: bool,
        broadcast: bool,
        verified: bool,
        megagroup: bool,
        restricted: bool,
        democracy: bool,
        signatures: bool,
        min: bool,
        id: i32,
        access_hash: i64,
        title: String,
        username: String,
        photo: ChatPhoto,
        date: i32,
        version: i32,
        restriction_reason: String,
    },

    nnelForbidden {
        flags: u32,
        broadcast: bool,
        megagroup: bool,
        id: i32,
        access_hash: i64,
        title: String,
    },
}

#[derive(Debug)]
enum ChatFull {
    Full {
        id: i32,
        participants: ChatParticipants,
        chat_photo: Photo,
        notify_settings: PeerNotifySettings,
        exported_invite: ExportedChatInvite,
        bot_info: Vec<BotInfo>,
    },

    nnelFull {
        flags: u32,
        can_view_participants: bool,
        can_set_username: bool,
        id: i32,
        about: String,
        participants_count: i32,
        admins_count: i32,
        kicked_count: i32,
        read_inbox_max_id: i32,
        read_outbox_max_id: i32,
        unread_count: i32,
        chat_photo: Photo,
        notify_settings: PeerNotifySettings,
        exported_invite: ExportedChatInvite,
        bot_info: Vec<BotInfo>,
        migrated_from_chat_id: i32,
        migrated_from_max_id: i32,
        pinned_msg_id: i32,
    },
}

#[derive(Debug)]
enum ChatParticipant {
    Participant {
        user_id: i32,
        inviter_id: i32,
        date: i32,
    },

    Creator {
        user_id: i32,
    },

    Admin {
        user_id: i32,
        inviter_id: i32,
        date: i32,
    },
}

#[derive(Debug)]
enum ChatParticipants {
    Forbidden {
        flags: u32,
        chat_id: i32,
        self_participant: ChatParticipant,
    },

    Participants {
        chat_id: i32,
        participants: Vec<ChatParticipant>,
        version: i32,
    },
}

#[derive(Debug)]
enum ChatPhoto {
    Empty,

    Photo {
        photo_small: FileLocation,
        photo_big: FileLocation,
    },
}

#[derive(Debug)]
enum Message {
    Empty {
        id: i32,
    },

    Message {
        flags: u32,
        out: bool,
        mentioned: bool,
        media_unread: bool,
        silent: bool,
        post: bool,
        id: i32,
        from_id: i32,
        to_id: Peer,
        fwd_from: MessageFwdHeader,
        via_bot_id: i32,
        reply_to_msg_id: i32,
        date: i32,
        message: String,
        media: MessageMedia,
        reply_markup: ReplyMarkup,
        entities: Vec<MessageEntity>,
        views: i32,
        edit_date: i32,
    },

    Service {
        flags: u32,
        out: bool,
        mentioned: bool,
        media_unread: bool,
        silent: bool,
        post: bool,
        id: i32,
        from_id: i32,
        to_id: Peer,
        reply_to_msg_id: i32,
        date: i32,
        action: MessageAction,
    },
}

#[derive(Debug)]
enum MessageMedia {
    Empty,

    Photo {
        photo: Photo,
        caption: String,
    },

    Geo {
        geo: GeoPoint,
    },

    Contact {
        phone_number: String,
        first_name: String,
        last_name: String,
        user_id: i32,
    },

    Unsupported,

    Document {
        document: Document,
        caption: String,
    },

    WebPage {
        webpage: WebPage,
    },

    Venue {
        geo: GeoPoint,
        title: String,
        address: String,
        provider: String,
        venue_id: String,
    },

    Game {
        game: Game,
    },
}

#[derive(Debug)]
enum MessageAction {
    Empty,

    ChatCreate {
        title: String,
        users: Vec<i32>,
    },

    ChatEditTitle {
        title: String,
    },

    ChatEditPhoto {
        photo: Photo,
    },

    ChatDeletePhoto,

    ChatAddUser {
        users: Vec<i32>,
    },

    ChatDeleteUser {
        user_id: i32,
    },

    ChatJoinedByLink {
        inviter_id: i32,
    },

    ChannelCreate {
        title: String,
    },

    ChatMigrateTo {
        channel_id: i32,
    },

    ChannelMigrateFrom {
        title: String,
        chat_id: i32,
    },

    PinMessage,

    HistoryClear,

    GameScore {
        game_id: i64,
        score: i32,
    },

    PhoneCall {
        flags: u32,
        call_id: i64,
        reason: PhoneCallDiscardReason,
        duration: i32,
    },
}

#[derive(Debug)]
struct Dialog {
    flags: u32,
    pinned: bool,
    peer: Peer,
    top_message: i32,
    read_inbox_max_id: i32,
    read_outbox_max_id: i32,
    unread_count: i32,
    notify_settings: PeerNotifySettings,
    pts: i32,
    draft: DraftMessage,
}

#[derive(Debug)]
enum Photo {
    Empty {
        id: i64,
    },

    Photo {
        flags: u32,
        has_stickers: bool,
        id: i64,
        access_hash: i64,
        date: i32,
        sizes: Vec<PhotoSize>,
    },
}

#[derive(Debug)]
enum PhotoSize {
    Empty {
        ttype: String,
    },

    Size {
        ttype: String,
        location: FileLocation,
        w: i32,
        h: i32,
        size: i32,
    },

    Cached {
        ttype: String,
        location: FileLocation,
        w: i32,
        h: i32,
        bytes: Vec<u8>,
    },
}

#[derive(Debug)]
enum GeoPoint {
    Empty,

    Point {
        long: f64,
        lat: f64,
    },
}

#[derive(Debug)]
struct AuthCheckedPhone {
    phone_registered: bool,
}

#[derive(Debug)]
struct AuthSentCode {
    flags: u32,
    phone_registered: bool,
    ttype: AuthSentCodeType,
    phone_code_hash: String,
    next_type: AuthCodeType,
    timeout: i32,
}

#[derive(Debug)]
struct AuthAuthorization {
    flags: u32,
    tmp_sessions: i32,
    user: User,
}

#[derive(Debug)]
struct AuthExportedAuthorization {
    id: i32,
    bytes: Vec<u8>,
}

#[derive(Debug)]
enum InputNotifyPeer {
    Peer {
        peer: InputPeer,
    },

    Users,

    Chats,

    All,
}

#[derive(Debug)]
enum InputPeerNotifyEvents {
    Empty,

    All,
}

#[derive(Debug)]
struct InputPeerNotifySettings {
    flags: u32,
    show_previews: bool,
    silent: bool,
    mute_until: i32,
    sound: String,
}

#[derive(Debug)]
enum PeerNotifyEvents {
    Empty,

    All,
}

#[derive(Debug)]
enum PeerNotifySettings {
    Empty,

    Settings {
        flags: u32,
        show_previews: bool,
        silent: bool,
        mute_until: i32,
        sound: String,
    },
}

#[derive(Debug)]
struct PeerSettings {
    flags: u32,
    report_spam: bool,
}

#[derive(Debug)]
enum WallPaper {
    Paper {
        id: i32,
        title: String,
        sizes: Vec<PhotoSize>,
        color: i32,
    },

    Solid {
        id: i32,
        title: String,
        bg_color: i32,
        color: i32,
    },
}

#[derive(Debug)]
enum ReportReason {
    putReportReasonSpam,

    putReportReasonViolence,

    putReportReasonPornography,

    putReportReasonOther {
        text: String,
    },
}

#[derive(Debug)]
struct UserFull {
    flags: u32,
    blocked: bool,
    phone_calls_available: bool,
    user: User,
    about: String,
    link: ContactsLink,
    profile_photo: Photo,
    notify_settings: PeerNotifySettings,
    bot_info: BotInfo,
    common_chats_count: i32,
}

#[derive(Debug)]
struct Contact {
    user_id: i32,
    mutual: bool,
}

#[derive(Debug)]
struct ImportedContact {
    user_id: i32,
    client_id: i64,
}

#[derive(Debug)]
struct ContactBlocked {
    user_id: i32,
    date: i32,
}

#[derive(Debug)]
struct ContactStatus {
    user_id: i32,
    status: UserStatus,
}

#[derive(Debug)]
struct ContactsLink {
    my_link: ContactLink,
    foreign_link: ContactLink,
    user: User,
}

#[derive(Debug)]
enum ContactsContacts {
    NotModified,

    Contacts {
        contacts: Vec<Contact>,
        users: Vec<User>,
    },
}

#[derive(Debug)]
struct ContactsImportedContacts {
    imported: Vec<ImportedContact>,
    retry_contacts: Vec<i64>,
    users: Vec<User>,
}

#[derive(Debug)]
enum ContactsBlocked {
    Blocked {
        blocked: Vec<ContactBlocked>,
        users: Vec<User>,
    },

    Slice {
        count: i32,
        blocked: Vec<ContactBlocked>,
        users: Vec<User>,
    },
}

#[derive(Debug)]
enum MessagesDialogs {
    Dialogs {
        dialogs: Vec<Dialog>,
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },

    Slice {
        count: i32,
        dialogs: Vec<Dialog>,
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },
}

#[derive(Debug)]
enum MessagesMessages {
    Messages {
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },

    Slice {
        count: i32,
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },

    Channel {
        flags: u32,
        pts: i32,
        count: i32,
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },
}

#[derive(Debug)]
enum MessagesChats {
    Chats {
        chats: Vec<Chat>,
    },

    Slice {
        count: i32,
        chats: Vec<Chat>,
    },
}

#[derive(Debug)]
struct MessagesChatFull {
    full_chat: ChatFull,
    chats: Vec<Chat>,
    users: Vec<User>,
}

#[derive(Debug)]
struct MessagesAffectedHistory {
    pts: i32,
    pts_count: i32,
    offset: i32,
}

#[derive(Debug)]
enum MessagesFilter {
    MessagesFilterEmpty,

    MessagesFilterPhotos,

    MessagesFilterVideo,

    MessagesFilterPhotoVideo,

    MessagesFilterPhotoVideoDocuments,

    MessagesFilterDocument,

    MessagesFilterUrl,

    MessagesFilterGif,

    MessagesFilterVoic,

    MessagesFilterMusic,

    MessagesFilterChatPhotos,

    MessagesFilterPhoneCalls {
        flags: u32,
        missed: bool,
    },
}

#[derive(Debug)]
enum Update {
    NewMessage {
        message: Message,
        pts: i32,
        pts_count: i32,
    },

    MessageID {
        id: i32,
        random_id: i64,
    },

    DeleteMessages {
        messages: Vec<i32>,
        pts: i32,
        pts_count: i32,
    },

    UserTyping {
        user_id: i32,
        action: SendMessageAction,
    },

    ChatUserTyping {
        chat_id: i32,
        user_id: i32,
        action: SendMessageAction,
    },

    ChatParticipants {
        participants: ChatParticipants,
    },

    UserStatus {
        user_id: i32,
        status: UserStatus,
    },

    UserName {
        user_id: i32,
        first_name: String,
        last_name: String,
        username: String,
    },

    UserPhoto {
        user_id: i32,
        date: i32,
        photo: UserProfilePhoto,
        previous: bool,
    },

    ContactRegistered {
        user_id: i32,
        date: i32,
    },

    ContactLink {
        user_id: i32,
        my_link: ContactLink,
        foreign_link: ContactLink,
    },

    NewEncryptedMessage {
        message: EncryptedMessage,
        qts: i32,
    },

    EncryptedChatTyping {
        chat_id: i32,
    },

    Encryption {
        chat: EncryptedChat,
        date: i32,
    },

    EncryptedMessagesRead {
        chat_id: i32,
        max_date: i32,
        date: i32,
    },

    ChatParticipantAdd {
        chat_id: i32,
        user_id: i32,
        inviter_id: i32,
        date: i32,
        version: i32,
    },

    ChatParticipantDelete {
        chat_id: i32,
        user_id: i32,
        version: i32,
    },

    DcOptions {
        dc_options: Vec<DcOption>,
    },

    UserBlocked {
        user_id: i32,
        blocked: bool,
    },

    NotifySettings {
        peer: NotifyPeer,
        notify_settings: PeerNotifySettings,
    },

    ServiceNotification {
        flags: u32,
        popup: bool,
        inbox_date: i32,
        ttype: String,
        message: String,
        media: MessageMedia,
        entities: Vec<MessageEntity>,
    },

    Privacy {
        key: PrivacyKey,
        rules: Vec<PrivacyRule>,
    },

    UserPhone {
        user_id: i32,
        phone: String,
    },

    ReadHistoryInbox {
        peer: Peer,
        max_id: i32,
        pts: i32,
        pts_count: i32,
    },

    ReadHistoryOutbox {
        peer: Peer,
        max_id: i32,
        pts: i32,
        pts_count: i32,
    },

    WebPage {
        webpage: WebPage,
        pts: i32,
        pts_count: i32,
    },

    ReadMessagesContents {
        messages: Vec<i32>,
        pts: i32,
        pts_count: i32,
    },

    ChannelTooLong {
        flags: u32,
        channel_id: i32,
        pts: i32,
    },

    Channel {
        channel_id: i32,
    },

    NewChannelMessage {
        message: Message,
        pts: i32,
        pts_count: i32,
    },

    ReadChannelInbox {
        channel_id: i32,
        max_id: i32,
    },

    DeleteChannelMessages {
        channel_id: i32,
        messages: Vec<i32>,
        pts: i32,
        pts_count: i32,
    },

    ChannelMessageViews {
        channel_id: i32,
        id: i32,
        views: i32,
    },

    ChatAdmins {
        chat_id: i32,
        enabled: bool,
        version: i32,
    },

    ChatParticipantAdmin {
        chat_id: i32,
        user_id: i32,
        is_admin: bool,
        version: i32,
    },

    NewStickerSet {
        stickerset: MessagesStickerSet,
    },

    StickerSetsOrder {
        flags: u32,
        masks: bool,
        order: Vec<i64>,
    },

    StickerSets,

    SavedGifs,

    BotInlineQuery {
        flags: u32,
        query_id: i64,
        user_id: i32,
        query: String,
        geo: GeoPoint,
        offset: String,
    },

    BotInlineSend {
        flags: u32,
        user_id: i32,
        query: String,
        geo: GeoPoint,
        id: String,
        msg_id: InputBotInlineMessageID,
    },

    EditChannelMessage {
        message: Message,
        pts: i32,
        pts_count: i32,
    },

    ChannelPinnedMessage {
        channel_id: i32,
        id: i32,
    },

    BotCallbackQuery {
        flags: u32,
        query_id: i64,
        user_id: i32,
        peer: Peer,
        msg_id: i32,
        chat_instance: i64,
        data: Vec<u8>,
        game_short_name: String,
    },

    EditMessage {
        message: Message,
        pts: i32,
        pts_count: i32,
    },

    InlineBotCallbackQuery {
        flags: u32,
        query_id: i64,
        user_id: i32,
        msg_id: InputBotInlineMessageID,
        chat_instance: i64,
        data: Vec<u8>,
        game_short_name: String,
    },

    ReadChannelOutbox {
        channel_id: i32,
        max_id: i32,
    },

    DraftMessage {
        peer: Peer,
        draft: DraftMessage,
    },

    ReadFeaturedStickers,

    RecentStickers,

    Config,

    PtsChanged,

    ChannelWebPage {
        channel_id: i32,
        webpage: WebPage,
        pts: i32,
        pts_count: i32,
    },

    PhoneCall {
        phone_call: PhoneCall,
    },

    DialogPinned {
        flags: u32,
        pinned: bool,
        peer: Peer,
    },

    PinnedDialogs {
        flags: u32,
        order: Vec<Peer>,
    },
}

#[derive(Debug)]
struct UpdatesState {
    pts: i32,
    qts: i32,
    date: i32,
    seq: i32,
    unread_count: i32,
}

#[derive(Debug)]
enum UpdatesDifference {
    Empty {
        date: i32,
        seq: i32,
    },

    Difference {
        new_messages: Vec<Message>,
        new_encrypted_messages: Vec<EncryptedMessage>,
        other_updates: Vec<Update>,
        chats: Vec<Chat>,
        users: Vec<User>,
        state: UpdatesState,
    },

    Slice {
        new_messages: Vec<Message>,
        new_encrypted_messages: Vec<EncryptedMessage>,
        other_updates: Vec<Update>,
        chats: Vec<Chat>,
        users: Vec<User>,
        intermediate_state: UpdatesState,
    },

    TooLong {
        pts: i32,
    },
}

#[derive(Debug)]
enum Updates {
    TooLong,

    ShortMessage {
        flags: u32,
        out: bool,
        mentioned: bool,
        media_unread: bool,
        silent: bool,
        id: i32,
        user_id: i32,
        message: String,
        pts: i32,
        pts_count: i32,
        date: i32,
        fwd_from: MessageFwdHeader,
        via_bot_id: i32,
        reply_to_msg_id: i32,
        entities: Vec<MessageEntity>,
    },

    ShortChatMessage {
        flags: u32,
        out: bool,
        mentioned: bool,
        media_unread: bool,
        silent: bool,
        id: i32,
        from_id: i32,
        chat_id: i32,
        message: String,
        pts: i32,
        pts_count: i32,
        date: i32,
        fwd_from: MessageFwdHeader,
        via_bot_id: i32,
        reply_to_msg_id: i32,
        entities: Vec<MessageEntity>,
    },

    Short {
        update: Update,
        date: i32,
    },

    Combined {
        updates: Vec<Update>,
        users: Vec<User>,
        chats: Vec<Chat>,
        date: i32,
        seq_start: i32,
        seq: i32,
    },

    Updates {
        updates: Vec<Update>,
        users: Vec<User>,
        chats: Vec<Chat>,
        date: i32,
        seq: i32,
    },

    ShortSentMessage {
        flags: u32,
        out: bool,
        id: i32,
        pts: i32,
        pts_count: i32,
        date: i32,
        media: MessageMedia,
        entities: Vec<MessageEntity>,
    },
}

#[derive(Debug)]
enum PhotosPhotos {
    Photos {
        photos: Vec<Photo>,
        users: Vec<User>,
    },

    Slice {
        count: i32,
        photos: Vec<Photo>,
        users: Vec<User>,
    },
}

#[derive(Debug)]
struct PhotosPhoto {
    photo: Photo,
    users: Vec<User>,
}

#[derive(Debug)]
struct UploadFile {
    ttype: StorageFileType,
    mtime: i32,
    bytes: Vec<u8>,
}

#[derive(Debug)]
struct DcOption {
    flags: u32,
    ipv6: bool,
    media_only: bool,
    tcpo_only: bool,
    id: i32,
    ip_address: String,
    port: i32,
}

#[derive(Debug)]
struct Config {
    flags: u32,
    phonecalls_enabled: bool,
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

#[derive(Debug)]
struct NearestDc {
    country: String,
    this_dc: i32,
    nearest_dc: i32,
}

#[derive(Debug)]
enum HelpAppUpdate {
    Update {
        id: i32,
        critical: bool,
        url: String,
        text: String,
    },

    No,
}

#[derive(Debug)]
struct HelpInviteText {
    message: String,
}

#[derive(Debug)]
enum EncryptedChat {
    Empty {
        id: i32,
    },

    Waiting {
        id: i32,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
    },

    Requested {
        id: i32,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
        g_a: Vec<u8>,
    },

    Chat {
        id: i32,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
        g_a_or_b: Vec<u8>,
        key_fingerprint: i64,
    },

    Discarded {
        id: i32,
    },
}

#[derive(Debug)]
struct InputEncryptedChat {
    chat_id: i32,
    access_hash: i64,
}

#[derive(Debug)]
enum EncryptedFile {
    Empty,

    File {
        id: i64,
        access_hash: i64,
        size: i32,
        dc_id: i32,
        key_fingerprint: i32,
    },
}

#[derive(Debug)]
enum InputEncryptedFile {
    Empty,

    Uploaded {
        id: i64,
        parts: i32,
        md5_checksum: String,
        key_fingerprint: i32,
    },

    File {
        id: i64,
        access_hash: i64,
    },

    BigUploaded {
        id: i64,
        parts: i32,
        key_fingerprint: i32,
    },
}

#[derive(Debug)]
enum EncryptedMessage {
    Message {
        random_id: i64,
        chat_id: i32,
        date: i32,
        bytes: Vec<u8>,
        file: EncryptedFile,
    },

    Service {
        random_id: i64,
        chat_id: i32,
        date: i32,
        bytes: Vec<u8>,
    },
}

#[derive(Debug)]
enum MessagesDhConfig {
    NotModified {
        random: Vec<u8>,
    },

    Config {
        g: i32,
        p: Vec<u8>,
        version: i32,
        random: Vec<u8>,
    },
}

#[derive(Debug)]
enum MessagesSentEncryptedMessage {
    Message {
        date: i32,
    },

    File {
        date: i32,
        file: EncryptedFile,
    },
}

#[derive(Debug)]
enum InputDocument {
    Empty,

    Document {
        id: i64,
        access_hash: i64,
    },
}

#[derive(Debug)]
enum Document {
    Empty {
        id: i64,
    },

    Document {
        id: i64,
        access_hash: i64,
        date: i32,
        mime_type: String,
        size: i32,
        thumb: PhotoSize,
        dc_id: i32,
        version: i32,
        attributes: Vec<DocumentAttribute>,
    },
}

#[derive(Debug)]
struct HelpSupport {
    phone_number: String,
    user: User,
}

#[derive(Debug)]
enum NotifyPeer {
    Peer {
        peer: Peer,
    },

    Users,

    Chats,

    All,
}

#[derive(Debug)]
enum SendMessageAction {
    Typing,

    Cancel,

    RecordVideo,

    UploadVideo {
        progress: i32,
    },

    RecordAudio,

    UploadAudio {
        progress: i32,
    },

    UploadPhoto {
        progress: i32,
    },

    UploadDocument {
        progress: i32,
    },

    GeoLocation,

    ChooseContact,

    GamePlay,
}

#[derive(Debug)]
struct ContactsFound {
    results: Vec<Peer>,
    chats: Vec<Chat>,
    users: Vec<User>,
}

#[derive(Debug)]
enum InputPrivacyKey {
    StatusTimestamp,

    ChatInvite,

    PhoneCall,
}

#[derive(Debug)]
enum PrivacyKey {
    StatusTimestamp,

    ChatInvite,

    PhoneCall,
}

#[derive(Debug)]
enum InputPrivacyRule {
    ValueAllowContacts,

    ValueAllowAll,

    ValueAllowUsers {
        users: Vec<InputUser>,
    },

    ValueDisallowContacts,

    ValueDisallowAll,

    ValueDisallowUsers {
        users: Vec<InputUser>,
    },
}

#[derive(Debug)]
enum PrivacyRule {
    ValueAllowContacts,

    ValueAllowAll,

    ValueAllowUsers {
        users: Vec<i32>,
    },

    ValueDisallowContacts,

    ValueDisallowAll,

    ValueDisallowUsers {
        users: Vec<i32>,
    },
}

#[derive(Debug)]
struct AccountPrivacyRules {
    rules: Vec<PrivacyRule>,
    users: Vec<User>,
}

#[derive(Debug)]
struct AccountDaysTTL {
    days: i32,
}

#[derive(Debug)]
enum DocumentAttribute {
    ImageSize {
        w: i32,
        h: i32,
    },

    Animated,

    Sticker {
        flags: u32,
        mask: bool,
        alt: String,
        stickerset: InputStickerSet,
        mask_coords: MaskCoords,
    },

    Video {
        duration: i32,
        w: i32,
        h: i32,
    },

    Audio {
        flags: u32,
        voice: bool,
        duration: i32,
        title: String,
        performer: String,
        waveform: Vec<u8>,
    },

    Filename {
        file_name: String,
    },

    HasStickers,
}

#[derive(Debug)]
enum MessagesStickers {
    NotModified,

    Stickers {
        hash: String,
        stickers: Vec<Document>,
    },
}

#[derive(Debug)]
struct StickerPack {
    emoticon: String,
    documents: Vec<i64>,
}

#[derive(Debug)]
enum MessagesAllStickers {
    NotModified,

    Stickers {
        hash: i32,
        sets: Vec<StickerSet>,
    },
}

#[derive(Debug)]
struct DisabledFeature {
    feature: String,
    description: String,
}

#[derive(Debug)]
struct MessagesAffectedMessages {
    pts: i32,
    pts_count: i32,
}

#[derive(Debug)]
enum ContactLink {
    Unknown,

    None,

    HasPhone,

    Contact,
}

#[derive(Debug)]
enum WebPage {
    Empty {
        id: i64,
    },

    Pending {
        id: i64,
        date: i32,
    },

    Page {
        flags: u32,
        id: i64,
        url: String,
        display_url: String,
        hash: i32,
        ttype: String,
        site_name: String,
        title: String,
        description: String,
        photo: Photo,
        embed_url: String,
        embed_type: String,
        embed_width: i32,
        embed_height: i32,
        duration: i32,
        author: String,
        document: Document,
        cached_page: Page,
    },

    NotModified,
}

#[derive(Debug)]
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

#[derive(Debug)]
struct AccountAuthorizations {
    authorizations: Vec<Authorization>,
}

#[derive(Debug)]
enum AccountPassword {
    No {
        new_salt: Vec<u8>,
        email_unconfirmed_pattern: String,
    },

    Password {
        current_salt: Vec<u8>,
        new_salt: Vec<u8>,
        hint: String,
        has_recovery: bool,
        email_unconfirmed_pattern: String,
    },
}

#[derive(Debug)]
struct AccountPasswordSettings {
    email: String,
}

#[derive(Debug)]
struct AccountPasswordInputSettings {
    flags: u32,
    new_salt: Vec<u8>,
    new_password_hash: Vec<u8>,
    hint: String,
    email: String,
}

#[derive(Debug)]
struct AuthPasswordRecovery {
    email_pattern: String,
}

#[derive(Debug)]
struct ReceivedNotifyMessage {
    id: i32,
    flags: i32,
}

#[derive(Debug)]
enum ExportedChatInvite {
    Empty,

    Exported {
        link: String,
    },
}

#[derive(Debug)]
enum ChatInvite {
    Already {
        chat: Chat,
    },

    Invite {
        flags: u32,
        channel: bool,
        broadcast: bool,
        public: bool,
        megagroup: bool,
        title: String,
        photo: ChatPhoto,
        participants_count: i32,
        participants: Vec<User>,
    },
}

#[derive(Debug)]
enum InputStickerSet {
    Empty,

    ID {
        id: i64,
        access_hash: i64,
    },

    ShortName {
        short_name: String,
    },
}

#[derive(Debug)]
struct StickerSet {
    flags: u32,
    installed: bool,
    archived: bool,
    official: bool,
    masks: bool,
    id: i64,
    access_hash: i64,
    title: String,
    short_name: String,
    count: i32,
    hash: i32,
}

#[derive(Debug)]
struct MessagesStickerSet {
    set: StickerSet,
    packs: Vec<StickerPack>,
    documents: Vec<Document>,
}

#[derive(Debug)]
struct BotCommand {
    command: String,
    description: String,
}

#[derive(Debug)]
struct BotInfo {
    user_id: i32,
    description: String,
    commands: Vec<BotCommand>,
}

#[derive(Debug)]
enum KeyboardButton {
    Button {
        text: String,
    },

    Url {
        text: String,
        url: String,
    },

    Callback {
        text: String,
        data: Vec<u8>,
    },

    RequestPhone {
        text: String,
    },

    RequestGeoLocation {
        text: String,
    },

    SwitchInline {
        flags: u32,
        same_peer: bool,
        text: String,
        query: String,
    },

    Game {
        text: String,
    },
}

#[derive(Debug)]
struct KeyboardButtonRow {
    buttons: Vec<KeyboardButton>,
}

#[derive(Debug)]
enum ReplyMarkup {
    KeyboardHide {
        flags: u32,
        selective: bool,
    },

    KeyboardForceReply {
        flags: u32,
        single_use: bool,
        selective: bool,
    },

    Keyboard {
        flags: u32,
        resize: bool,
        single_use: bool,
        selective: bool,
        rows: Vec<KeyboardButtonRow>,
    },

    Inline {
        rows: Vec<KeyboardButtonRow>,
    },
}

#[derive(Debug)]
enum HelpAppChangelog {
    Empty,

    Changelog {
        message: String,
        media: MessageMedia,
        entities: Vec<MessageEntity>,
    },
}

#[derive(Debug)]
enum MessageEntity {
    Unknown {
        offset: i32,
        length: i32,
    },

    Mention {
        offset: i32,
        length: i32,
    },

    Hashtag {
        offset: i32,
        length: i32,
    },

    BotCommand {
        offset: i32,
        length: i32,
    },

    Url {
        offset: i32,
        length: i32,
    },

    Email {
        offset: i32,
        length: i32,
    },

    Bold {
        offset: i32,
        length: i32,
    },

    Italic {
        offset: i32,
        length: i32,
    },

    Code {
        offset: i32,
        length: i32,
    },

    Pre {
        offset: i32,
        length: i32,
        language: String,
    },

    TextUrl {
        offset: i32,
        length: i32,
        url: String,
    },

    MentionName {
        offset: i32,
        length: i32,
        user_id: i32,
    },

    putMessageEntityMentionName {
        offset: i32,
        length: i32,
        user_id: InputUser,
    },
}

#[derive(Debug)]
enum InputChannel {
    Empty,

    Channel {
        channel_id: i32,
        access_hash: i64,
    },
}

#[derive(Debug)]
struct ContactsResolvedPeer {
    peer: Peer,
    chats: Vec<Chat>,
    users: Vec<User>,
}

#[derive(Debug)]
struct MessageRange {
    min_id: i32,
    max_id: i32,
}

#[derive(Debug)]
enum UpdatesChannelDifference {
    Empty {
        flags: u32,
        ffinal: bool,
        pts: i32,
        timeout: i32,
    },

    TooLong {
        flags: u32,
        ffinal: bool,
        pts: i32,
        timeout: i32,
        top_message: i32,
        read_inbox_max_id: i32,
        read_outbox_max_id: i32,
        unread_count: i32,
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },

    Difference {
        flags: u32,
        ffinal: bool,
        pts: i32,
        timeout: i32,
        new_messages: Vec<Message>,
        other_updates: Vec<Update>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },
}

#[derive(Debug)]
enum ChannelMessagesFilter {
    Empty,

    Filter {
        flags: u32,
        exclude_new_messages: bool,
        ranges: Vec<MessageRange>,
    },
}

#[derive(Debug)]
enum ChannelParticipant {
    Participant {
        user_id: i32,
        date: i32,
    },

    SSelf {
        user_id: i32,
        inviter_id: i32,
        date: i32,
    },

    Moderator {
        user_id: i32,
        inviter_id: i32,
        date: i32,
    },

    Editor {
        user_id: i32,
        inviter_id: i32,
        date: i32,
    },

    Kicked {
        user_id: i32,
        kicked_by: i32,
        date: i32,
    },

    Creator {
        user_id: i32,
    },
}

#[derive(Debug)]
enum ChannelParticipantsFilter {
    Recent,

    Admins,

    Kicked,

    Bots,
}

#[derive(Debug)]
enum ChannelParticipantRole {
    RoleEmpty,

    RoleModerator,

    RoleEditor,
}

#[derive(Debug)]
struct ChannelsChannelParticipants {
    count: i32,
    participants: Vec<ChannelParticipant>,
    users: Vec<User>,
}

#[derive(Debug)]
struct ChannelsChannelParticipant {
    participant: ChannelParticipant,
    users: Vec<User>,
}

#[derive(Debug)]
struct HelpTermsOfService {
    text: String,
}

#[derive(Debug)]
enum FoundGif {
    Gif {
        url: String,
        thumb_url: String,
        content_url: String,
        content_type: String,
        w: i32,
        h: i32,
    },

    Cached {
        url: String,
        photo: Photo,
        document: Document,
    },
}

#[derive(Debug)]
struct MessagesFoundGifs {
    next_offset: i32,
    results: Vec<FoundGif>,
}

#[derive(Debug)]
enum MessagesSavedGifs {
    NotModified,

    Gifs {
        hash: i32,
        gifs: Vec<Document>,
    },
}

#[derive(Debug)]
enum InputBotInlineMessage {
    MediaAuto {
        flags: u32,
        caption: String,
        reply_markup: ReplyMarkup,
    },

    Text {
        flags: u32,
        no_webpage: bool,
        message: String,
        entities: Vec<MessageEntity>,
        reply_markup: ReplyMarkup,
    },

    MediaGeo {
        flags: u32,
        geo_point: InputGeoPoint,
        reply_markup: ReplyMarkup,
    },

    MediaVenue {
        flags: u32,
        geo_point: InputGeoPoint,
        title: String,
        address: String,
        provider: String,
        venue_id: String,
        reply_markup: ReplyMarkup,
    },

    MediaContact {
        flags: u32,
        phone_number: String,
        first_name: String,
        last_name: String,
        reply_markup: ReplyMarkup,
    },

    Game {
        flags: u32,
        reply_markup: ReplyMarkup,
    },
}

#[derive(Debug)]
enum InputBotInlineResult {
    Result {
        flags: u32,
        id: String,
        ttype: String,
        title: String,
        description: String,
        url: String,
        thumb_url: String,
        content_url: String,
        content_type: String,
        w: i32,
        h: i32,
        duration: i32,
        send_message: InputBotInlineMessage,
    },

    Photo {
        id: String,
        ttype: String,
        photo: InputPhoto,
        send_message: InputBotInlineMessage,
    },

    Document {
        flags: u32,
        id: String,
        ttype: String,
        title: String,
        description: String,
        document: InputDocument,
        send_message: InputBotInlineMessage,
    },

    Game {
        id: String,
        short_name: String,
        send_message: InputBotInlineMessage,
    },
}

#[derive(Debug)]
enum BotInlineMessage {
    MediaAuto {
        flags: u32,
        caption: String,
        reply_markup: ReplyMarkup,
    },

    Text {
        flags: u32,
        no_webpage: bool,
        message: String,
        entities: Vec<MessageEntity>,
        reply_markup: ReplyMarkup,
    },

    MediaGeo {
        flags: u32,
        geo: GeoPoint,
        reply_markup: ReplyMarkup,
    },

    MediaVenue {
        flags: u32,
        geo: GeoPoint,
        title: String,
        address: String,
        provider: String,
        venue_id: String,
        reply_markup: ReplyMarkup,
    },

    MediaContact {
        flags: u32,
        phone_number: String,
        first_name: String,
        last_name: String,
        reply_markup: ReplyMarkup,
    },
}

#[derive(Debug)]
enum BotInlineResult {
    Result {
        flags: u32,
        id: String,
        ttype: String,
        title: String,
        description: String,
        url: String,
        thumb_url: String,
        content_url: String,
        content_type: String,
        w: i32,
        h: i32,
        duration: i32,
        send_message: BotInlineMessage,
    },

    Media {
        flags: u32,
        id: String,
        ttype: String,
        photo: Photo,
        document: Document,
        title: String,
        description: String,
        send_message: BotInlineMessage,
    },
}

#[derive(Debug)]
struct MessagesBotResults {
    flags: u32,
    gallery: bool,
    query_id: i64,
    next_offset: String,
    switch_pm: InlineBotSwitchPM,
    results: Vec<BotInlineResult>,
    cache_time: i32,
}

#[derive(Debug)]
struct ExportedMessageLink {
    link: String,
}

#[derive(Debug)]
struct MessageFwdHeader {
    flags: u32,
    from_id: i32,
    date: i32,
    channel_id: i32,
    channel_post: i32,
}

#[derive(Debug)]
enum AuthCodeType {
    Sms,

    Call,

    FlashCall,
}

#[derive(Debug)]
enum AuthSentCodeType {
    App {
        length: i32,
    },

    Sms {
        length: i32,
    },

    Call {
        length: i32,
    },

    FlashCall {
        pattern: String,
    },
}

#[derive(Debug)]
struct MessagesBotCallbackAnswer {
    flags: u32,
    alert: bool,
    has_url: bool,
    message: String,
    url: String,
    cache_time: i32,
}

#[derive(Debug)]
struct MessagesMessageEditData {
    flags: u32,
    caption: bool,
}

#[derive(Debug)]
struct InputBotInlineMessageID {
    dc_id: i32,
    id: i64,
    access_hash: i64,
}

#[derive(Debug)]
struct InlineBotSwitchPM {
    text: String,
    start_param: String,
}

#[derive(Debug)]
struct MessagesPeerDialogs {
    dialogs: Vec<Dialog>,
    messages: Vec<Message>,
    chats: Vec<Chat>,
    users: Vec<User>,
    state: UpdatesState,
}

#[derive(Debug)]
struct TopPeer {
    peer: Peer,
    rating: f64,
}

#[derive(Debug)]
enum TopPeerCategory {
    BotsPM,

    BotsInline,

    Correspondents,

    Groups,

    Channels,
}

#[derive(Debug)]
struct TopPeerCategoryPeers {
    category: TopPeerCategory,
    count: i32,
    peers: Vec<TopPeer>,
}

#[derive(Debug)]
enum ContactsTopPeers {
    NotModified,

    Peers {
        categories: Vec<TopPeerCategoryPeers>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },
}

#[derive(Debug)]
enum DraftMessage {
    Empty,

    Message {
        flags: u32,
        no_webpage: bool,
        reply_to_msg_id: i32,
        message: String,
        entities: Vec<MessageEntity>,
        date: i32,
    },
}

#[derive(Debug)]
enum MessagesFeaturedStickers {
    NotModified,

    Stickers {
        hash: i32,
        sets: Vec<StickerSetCovered>,
        unread: Vec<i64>,
    },
}

#[derive(Debug)]
enum MessagesRecentStickers {
    NotModified,

    Stickers {
        hash: i32,
        stickers: Vec<Document>,
    },
}

#[derive(Debug)]
struct MessagesArchivedStickers {
    count: i32,
    sets: Vec<StickerSetCovered>,
}

#[derive(Debug)]
enum MessagesStickerSetInstallResult {
    Success,

    Archive {
        sets: Vec<StickerSetCovered>,
    },
}

#[derive(Debug)]
enum StickerSetCovered {
    Covered {
        set: StickerSet,
        cover: Document,
    },

    Multi {
        set: StickerSet,
        covers: Vec<Document>,
    },
}

#[derive(Debug)]
struct MaskCoords {
    n: i32,
    x: f64,
    y: f64,
    zoom: f64,
}

#[derive(Debug)]
enum InputStickeredMedia {
    Photo {
        id: InputPhoto,
    },

    Document {
        id: InputDocument,
    },
}

#[derive(Debug)]
struct Game {
    flags: u32,
    id: i64,
    access_hash: i64,
    short_name: String,
    title: String,
    description: String,
    photo: Photo,
    document: Document,
}

#[derive(Debug)]
enum InputGame {
    ID {
        id: i64,
        access_hash: i64,
    },

    ShortName {
        bot_id: InputUser,
        short_name: String,
    },
}

#[derive(Debug)]
struct HighScore {
    pos: i32,
    user_id: i32,
    score: i32,
}

#[derive(Debug)]
struct MessagesHighScores {
    scores: Vec<HighScore>,
    users: Vec<User>,
}

#[derive(Debug)]
enum RichText {
    Empty,

    Plain {
        text: String,
    },

    Bold {
        text: RichText,
    },

    Italic {
        text: RichText,
    },

    Underline {
        text: RichText,
    },

    Strike {
        text: RichText,
    },

    Fixed {
        text: RichText,
    },

    Url {
        text: RichText,
        url: String,
        webpage_id: i64,
    },

    Email {
        text: RichText,
        email: String,
    },

    Concat {
        texts: Vec<RichText>,
    },
}

#[derive(Debug)]
enum PageBlock {
    Unsupported,

    Title {
        text: RichText,
    },

    Subtitle {
        text: RichText,
    },

    AuthorDate {
        author: RichText,
        published_date: i32,
    },

    Header {
        text: RichText,
    },

    Subheader {
        text: RichText,
    },

    Paragraph {
        text: RichText,
    },

    Preformatted {
        text: RichText,
        language: String,
    },

    Footer {
        text: RichText,
    },

    Divider,

    Anchor {
        name: String,
    },

    List {
        ordered: bool,
        items: Vec<RichText>,
    },

    Blockquote {
        text: RichText,
        caption: RichText,
    },

    Pullquote {
        text: RichText,
        caption: RichText,
    },

    Photo {
        photo_id: i64,
        caption: RichText,
    },

    Video {
        flags: u32,
        autoplay: bool,
        lloop: bool,
        video_id: i64,
        caption: RichText,
    },

    Cover {
        cover: PageBlock,
    },

    Embed {
        flags: u32,
        full_width: bool,
        allow_scrolling: bool,
        url: String,
        html: String,
        poster_photo_id: i64,
        w: i32,
        h: i32,
        caption: RichText,
    },

    EmbedPost {
        url: String,
        webpage_id: i64,
        author_photo_id: i64,
        author: String,
        date: i32,
        blocks: Vec<PageBlock>,
        caption: RichText,
    },

    Collage {
        items: Vec<PageBlock>,
        caption: RichText,
    },

    Slideshow {
        items: Vec<PageBlock>,
        caption: RichText,
    },
}

#[derive(Debug)]
enum Page {
    Part {
        blocks: Vec<PageBlock>,
        photos: Vec<Photo>,
        videos: Vec<Document>,
    },

    Full {
        blocks: Vec<PageBlock>,
        photos: Vec<Photo>,
        videos: Vec<Document>,
    },
}

#[derive(Debug)]
struct InputPhoneCall {
    id: i64,
    access_hash: i64,
}

#[derive(Debug)]
enum PhoneCall {
    Empty {
        id: i64,
    },

    Waiting {
        flags: u32,
        id: i64,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
        protocol: PhoneCallProtocol,
        receive_date: i32,
    },

    Requested {
        id: i64,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
        g_a: Vec<u8>,
        protocol: PhoneCallProtocol,
    },

    Call {
        id: i64,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
        g_a_or_b: Vec<u8>,
        key_fingerprint: i64,
        protocol: PhoneCallProtocol,
        connection: PhoneConnection,
        alternative_connections: Vec<PhoneConnection>,
        start_date: i32,
    },

    Discarded {
        flags: u32,
        id: i64,
        reason: PhoneCallDiscardReason,
        duration: i32,
    },
}

#[derive(Debug)]
struct PhoneConnection {
    id: i64,
    ip: String,
    ipv6: String,
    port: i32,
    peer_tag: Vec<u8>,
}

#[derive(Debug)]
struct PhoneCallProtocol {
    flags: u32,
    udp_p2p: bool,
    udp_reflector: bool,
    min_layer: i32,
    max_layer: i32,
}

#[derive(Debug)]
struct PhonePhoneCall {
    phone_call: PhoneCall,
    users: Vec<User>,
}

#[derive(Debug)]
enum PhoneCallDiscardReason {
    Missed,

    Disconnect,

    Hangup,

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