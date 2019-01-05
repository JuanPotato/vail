#[derive(Debug, Clone, PartialEq)]
pub enum MessageAction {
    ChatCreate {
        title: String,
        users: Vec<i32>,
    },

    ChatDeleteUser {
        user_id: i32,
    },

    CustomAction {
        message: String,
    },

    ChatMigrateTo {
        channel_id: i32,
    },

    PaymentSentMe {
        flags: u32,
        currency: String,
        total_amount: i64,
        payload: Vec<u8>,
        info: Option<Box<PaymentRequestedInfo>>,
        shipping_option_id: Option<String>,
        charge: Box<PaymentCharge>,
    },

    PhoneCall {
        flags: u32,
        call_id: i64,
        reason: Option<Box<PhoneCallDiscardReason>>,
        duration: Option<i32>,
    },

    Empty,

    ScreenshotTaken,

    ChatJoinedByLink {
        inviter_id: i32,
    },

    ChatAddUser {
        users: Vec<i32>,
    },

    GameScore {
        game_id: i64,
        score: i32,
    },

    PaymentSent {
        currency: String,
        total_amount: i64,
    },

    HistoryClear,

    ChatEditTitle {
        title: String,
    },

    ChannelMigrateFrom {
        title: String,
        chat_id: i32,
    },

    ChatDeletePhoto,

    ChatEditPhoto {
        photo: Box<Photo>,
    },

    PinMessage,

    ChannelCreate {
        title: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum TopPeerCategory {
    Correspondents,

    Channels,

    BotsPm,

    BotsInline,

    Groups,

    PhoneCalls,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContactsResolvedPeer {
    pub peer: Box<Peer>,
    pub chats: Vec<Chat>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StickerPack {
    pub emoticon: String,
    pub documents: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Photo {
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

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelAdminLogEvent {
    pub id: i64,
    pub date: i32,
    pub user_id: i32,
    pub action: Box<ChannelAdminLogEventAction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessagesAffectedMessages {
    pub pts: i32,
    pub pts_count: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhotosPhoto {
    pub photo: Box<Photo>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HelpSupport {
    pub phone_number: String,
    pub user: Box<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MsgsStateReq {
    pub msg_ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PeerNotifySettings {
    Empty,

    Settings {
        flags: u32,
        show_previews: bool,
        silent: bool,
        mute_until: i32,
        sound: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputBotInlineMessage {
    MediaGeo {
        flags: u32,
        geo_point: Box<InputGeoPoint>,
        period: i32,
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    MediaVenue {
        flags: u32,
        geo_point: Box<InputGeoPoint>,
        title: String,
        address: String,
        provider: String,
        venue_id: String,
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    Text {
        flags: u32,
        no_webpage: bool,
        message: String,
        entities: Option<Vec<MessageEntity>>,
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    MediaAuto {
        flags: u32,
        caption: String,
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    MediaContact {
        flags: u32,
        phone_number: String,
        first_name: String,
        last_name: String,
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    Game {
        flags: u32,
        reply_markup: Option<Box<ReplyMarkup>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputPhoneCall {
    pub id: i64,
    pub access_hash: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagesFavedStickers {
    NotModified,

    Stickers {
        hash: i32,
        packs: Vec<StickerPack>,
        stickers: Vec<Document>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputAppEvent {
    pub time: f64,
    pub type_: String,
    pub peer: i64,
    pub data: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChannelAdminLogEventAction {
    ChangeStickerSet {
        prev_stickerset: Box<InputStickerSet>,
        new_stickerset: Box<InputStickerSet>,
    },

    TogglePreHistoryHidden {
        new_value: bool,
    },

    ParticipantLeave,

    ChangePhoto {
        prev_photo: Box<ChatPhoto>,
        new_photo: Box<ChatPhoto>,
    },

    ToggleSignatures {
        new_value: bool,
    },

    ParticipantInvite {
        participant: Box<ChannelParticipant>,
    },

    ChangeAbout {
        prev_value: String,
        new_value: String,
    },

    EditMessage {
        prev_message: Box<Message>,
        new_message: Box<Message>,
    },

    ChangeTitle {
        prev_value: String,
        new_value: String,
    },

    ChangeUsername {
        prev_value: String,
        new_value: String,
    },

    ParticipantToggleAdmin {
        prev_participant: Box<ChannelParticipant>,
        new_participant: Box<ChannelParticipant>,
    },

    ParticipantToggleBan {
        prev_participant: Box<ChannelParticipant>,
        new_participant: Box<ChannelParticipant>,
    },

    ParticipantJoin,

    DeleteMessage {
        message: Box<Message>,
    },

    UpdatePinned {
        message: Box<Message>,
    },

    ToggleInvites {
        new_value: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChannelParticipant {
    Creator {
        user_id: i32,
    },

    Participant {
        user_id: i32,
        date: i32,
    },

    Self_ {
        user_id: i32,
        inviter_id: i32,
        date: i32,
    },

    Admin {
        flags: u32,
        can_edit: bool,
        user_id: i32,
        inviter_id: i32,
        promoted_by: i32,
        date: i32,
        admin_rights: Box<ChannelAdminRights>,
    },

    Banned {
        flags: u32,
        left: bool,
        user_id: i32,
        kicked_by: i32,
        date: i32,
        banned_rights: Box<ChannelBannedRights>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContactStatus {
    pub user_id: i32,
    pub status: Box<UserStatus>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ServerDhParams {
    Ok {
        nonce: i128,
        server_nonce: i128,
        encrypted_answer: Vec<u8>,
    },

    Fail {
        nonce: i128,
        server_nonce: i128,
        new_nonce_hash: i128,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PopularContact {
    pub client_id: i64,
    pub importers: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputStickeredMedia {
    Photo {
        id: Box<InputPhoto>,
    },

    Document {
        id: Box<InputDocument>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputBotInlineResult {
    Photo {
        id: String,
        type_: String,
        photo: Box<InputPhoto>,
        send_message: Box<InputBotInlineMessage>,
    },

    Result {
        flags: u32,
        id: String,
        type_: String,
        title: Option<String>,
        description: Option<String>,
        url: Option<String>,
        thumb_url: Option<String>,
        content_url: Option<String>,
        content_type: Option<String>,
        w: Option<i32>,
        h: Option<i32>,
        duration: Option<i32>,
        send_message: Box<InputBotInlineMessage>,
    },

    Game {
        id: String,
        short_name: String,
        send_message: Box<InputBotInlineMessage>,
    },

    Document {
        flags: u32,
        id: String,
        type_: String,
        title: Option<String>,
        description: Option<String>,
        document: Box<InputDocument>,
        send_message: Box<InputBotInlineMessage>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum RichText {
    Bold {
        text: Box<RichText>,
    },

    Empty,

    Url {
        text: Box<RichText>,
        url: String,
        webpage_id: i64,
    },

    Italic {
        text: Box<RichText>,
    },

    Plain {
        text: String,
    },

    Email {
        text: Box<RichText>,
        email: String,
    },

    Concat {
        texts: Vec<RichText>,
    },

    Strike {
        text: Box<RichText>,
    },

    Underline {
        text: Box<RichText>,
    },

    Fixed {
        text: Box<RichText>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatParticipant {
    Admin {
        user_id: i32,
        inviter_id: i32,
        date: i32,
    },

    Creator {
        user_id: i32,
    },

    Participant {
        user_id: i32,
        inviter_id: i32,
        date: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct DataJson {
    pub data: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NewSession {
    Created {
        first_msg_id: i64,
        unique_id: i64,
        server_salt: i64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MsgResendReq {
    pub msg_ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelAdminRights {
    pub flags: u32,
    pub change_info: bool,
    pub post_messages: bool,
    pub edit_messages: bool,
    pub delete_messages: bool,
    pub ban_users: bool,
    pub invite_users: bool,
    pub invite_link: bool,
    pub pin_messages: bool,
    pub add_admins: bool,
    pub manage_call: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountAuthorizations {
    pub authorizations: Vec<Authorization>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputWebFileLocation {
    pub url: String,
    pub access_hash: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserProfilePhoto {
    Photo {
        photo_id: i64,
        photo_small: Box<FileLocation>,
        photo_big: Box<FileLocation>,
    },

    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopPeerCategoryPeers {
    pub category: Box<TopPeerCategory>,
    pub count: i32,
    pub peers: Vec<TopPeer>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PhoneCallDiscardReason {
    Busy,

    Hangup,

    Disconnect,

    Missed,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessagesHighScores {
    pub scores: Vec<HighScore>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputBotInlineMessageId {
    pub dc_id: i32,
    pub id: i64,
    pub access_hash: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Invoice {
    pub flags: u32,
    pub test: bool,
    pub name_requested: bool,
    pub phone_requested: bool,
    pub email_requested: bool,
    pub shipping_address_requested: bool,
    pub flexible: bool,
    pub phone_to_provider: bool,
    pub email_to_provider: bool,
    pub currency: String,
    pub prices: Vec<LabeledPrice>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagesStickerSetInstallResult {
    Archive {
        sets: Vec<StickerSetCovered>,
    },

    Success,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HelpTermsOfService {
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagesAllStickers {
    Stickers {
        hash: i32,
        sets: Vec<StickerSet>,
    },

    NotModified,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputPeerNotifySettings {
    pub flags: u32,
    pub show_previews: bool,
    pub silent: bool,
    pub mute_until: i32,
    pub sound: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SendMessageAction {
    RecordVideo,

    UploadDocument {
        progress: i32,
    },

    GamePlay,

    ChooseContact,

    UploadPhoto {
        progress: i32,
    },

    Typing,

    RecordAudio,

    RecordRound,

    UploadVideo {
        progress: i32,
    },

    UploadRound {
        progress: i32,
    },

    UploadAudio {
        progress: i32,
    },

    GeoLocation,

    Cancel,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContactsImportedContacts {
    pub imported: Vec<ImportedContact>,
    pub popular_invites: Vec<PopularContact>,
    pub retry_contacts: Vec<i64>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatFull {
    ChannelFull {
        flags: u32,
        can_view_participants: bool,
        can_set_username: bool,
        can_set_stickers: bool,
        hidden_prehistory: bool,
        id: i32,
        about: String,
        participants_count: Option<i32>,
        admins_count: Option<i32>,
        kicked_count: Option<i32>,
        banned_count: Option<i32>,
        read_inbox_max_id: i32,
        read_outbox_max_id: i32,
        unread_count: i32,
        chat_photo: Box<Photo>,
        notify_settings: Box<PeerNotifySettings>,
        exported_invite: Box<ExportedChatInvite>,
        bot_info: Vec<BotInfo>,
        migrated_from_chat_id: Option<i32>,
        migrated_from_max_id: Option<i32>,
        pinned_msg_id: Option<i32>,
        stickerset: Option<Box<StickerSet>>,
        available_min_id: Option<i32>,
    },

    Full {
        id: i32,
        participants: Box<ChatParticipants>,
        chat_photo: Box<Photo>,
        notify_settings: Box<PeerNotifySettings>,
        exported_invite: Box<ExportedChatInvite>,
        bot_info: Vec<BotInfo>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct LabeledPrice {
    pub label: String,
    pub amount: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagesFeaturedStickers {
    Stickers {
        hash: i32,
        sets: Vec<StickerSetCovered>,
        unread: Vec<i64>,
    },

    NotModified,
}

#[derive(Debug, Clone, PartialEq)]
pub struct LangPackDifference {
    pub lang_code: String,
    pub from_version: i32,
    pub version: i32,
    pub strings: Vec<LangPackString>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum EncryptedChat {
    Empty {
        id: i32,
    },

    Discarded {
        id: i32,
    },

    Requested {
        id: i32,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
        g_a: Vec<u8>,
    },

    Waiting {
        id: i32,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum FoundGif {
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
        photo: Box<Photo>,
        document: Box<Document>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FeedBroadcasts {
    Ungrouped {
        channels: Vec<i32>,
    },

    Broadcasts {
        feed_id: i32,
        channels: Vec<i32>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Updates {
    UpdateShortChatMessage {
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
        fwd_from: Option<Box<MessageFwdHeader>>,
        via_bot_id: Option<i32>,
        reply_to_msg_id: Option<i32>,
        entities: Option<Vec<MessageEntity>>,
    },

    UpdateShortMessage {
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
        fwd_from: Option<Box<MessageFwdHeader>>,
        via_bot_id: Option<i32>,
        reply_to_msg_id: Option<i32>,
        entities: Option<Vec<MessageEntity>>,
    },

    TooLong,

    UpdateShortSentMessage {
        flags: u32,
        out: bool,
        id: i32,
        pts: i32,
        pts_count: i32,
        date: i32,
        media: Option<Box<MessageMedia>>,
        entities: Option<Vec<MessageEntity>>,
    },

    Updates {
        updates: Vec<Update>,
        users: Vec<User>,
        chats: Vec<Chat>,
        date: i32,
        seq: i32,
    },

    UpdateShort {
        update: Box<Update>,
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
}

#[derive(Debug, Clone, PartialEq)]
pub enum PhotosPhotos {
    PhotosSlice {
        count: i32,
        photos: Vec<Photo>,
        users: Vec<User>,
    },

    Photos {
        photos: Vec<Photo>,
        users: Vec<User>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum RpcDropAnswer {
    AnswerUnknown,

    AnswerDroppedRunning,

    AnswerDropped {
        msg_id: i64,
        seq_no: i32,
        bytes: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputChatPhoto {
    Photo {
        id: Box<InputPhoto>,
    },

    Uploaded {
        file: Box<InputFile>,
    },

    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputDocument {
    Document {
        id: i64,
        access_hash: i64,
    },

    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ImportedContact {
    pub user_id: i32,
    pub client_id: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct UploadWebFile {
    pub size: i32,
    pub mime_type: String,
    pub file_type: Box<StorageFileType>,
    pub mtime: i32,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MsgsAck {
    pub msg_ids: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputFile {
    Big {
        id: i64,
        parts: i32,
        name: String,
    },

    File {
        id: i64,
        parts: i32,
        name: String,
        md5_checksum: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaymentsPaymentReceipt {
    pub flags: u32,
    pub date: i32,
    pub bot_id: i32,
    pub invoice: Box<Invoice>,
    pub provider_id: i32,
    pub info: Option<Box<PaymentRequestedInfo>>,
    pub shipping: Option<Box<ShippingOption>>,
    pub currency: String,
    pub total_amount: i64,
    pub credentials_title: String,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagesFeedMessages {
    NotModified,

    Messages {
        flags: u32,
        max_position: Option<Box<FeedPosition>>,
        min_position: Option<Box<FeedPosition>>,
        read_max_position: Option<Box<FeedPosition>>,
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct HttpWait {
    pub max_delay: i32,
    pub wait_after: i32,
    pub max_wait: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DisabledFeature {
    pub feature: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrivacyKey {
    PhoneCall,

    StatusTimestamp,

    ChatInvite,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BotInlineMessage {
    MediaVenue {
        flags: u32,
        geo: Box<GeoPoint>,
        title: String,
        address: String,
        provider: String,
        venue_id: String,
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    Text {
        flags: u32,
        no_webpage: bool,
        message: String,
        entities: Option<Vec<MessageEntity>>,
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    MediaGeo {
        flags: u32,
        geo: Box<GeoPoint>,
        period: i32,
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    MediaContact {
        flags: u32,
        phone_number: String,
        first_name: String,
        last_name: String,
        reply_markup: Option<Box<ReplyMarkup>>,
    },

    MediaAuto {
        flags: u32,
        caption: String,
        reply_markup: Option<Box<ReplyMarkup>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum BotInlineResult {
    Result {
        flags: u32,
        id: String,
        type_: String,
        title: Option<String>,
        description: Option<String>,
        url: Option<String>,
        thumb_url: Option<String>,
        content_url: Option<String>,
        content_type: Option<String>,
        w: Option<i32>,
        h: Option<i32>,
        duration: Option<i32>,
        send_message: Box<BotInlineMessage>,
    },

    Media {
        flags: u32,
        id: String,
        type_: String,
        photo: Option<Box<Photo>>,
        document: Option<Box<Document>>,
        title: Option<String>,
        description: Option<String>,
        send_message: Box<BotInlineMessage>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyboardButton {
    SwitchInline {
        flags: u32,
        same_peer: bool,
        text: String,
        query: String,
    },

    RequestGeoLocation {
        text: String,
    },

    Url {
        text: String,
        url: String,
    },

    Button {
        text: String,
    },

    RequestPhone {
        text: String,
    },

    Callback {
        text: String,
        data: Vec<u8>,
    },

    Buy {
        text: String,
    },

    Game {
        text: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum EncryptedMessage {
    Service {
        random_id: i64,
        chat_id: i32,
        date: i32,
        bytes: Vec<u8>,
    },

    Message {
        random_id: i64,
        chat_id: i32,
        date: i32,
        bytes: Vec<u8>,
        file: Box<EncryptedFile>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum StickerSetCovered {
    Multi {
        set: Box<StickerSet>,
        covers: Vec<Document>,
    },

    Covered {
        set: Box<StickerSet>,
        cover: Box<Document>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagesRecentStickers {
    Stickers {
        hash: i32,
        stickers: Vec<Document>,
    },

    NotModified,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputUser {
    Empty,

    User {
        user_id: i32,
        access_hash: i64,
    },

    Self_,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelAdminLogEventsFilter {
    pub flags: u32,
    pub join: bool,
    pub leave: bool,
    pub invite: bool,
    pub ban: bool,
    pub unban: bool,
    pub kick: bool,
    pub unkick: bool,
    pub promote: bool,
    pub demote: bool,
    pub info: bool,
    pub settings: bool,
    pub pinned: bool,
    pub edit: bool,
    pub delete: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputMedia {
    DocumentExternal {
        flags: u32,
        url: String,
        caption: String,
        ttl_seconds: Option<i32>,
    },

    Venue {
        geo_point: Box<InputGeoPoint>,
        title: String,
        address: String,
        provider: String,
        venue_id: String,
        venue_type: String,
    },

    Contact {
        phone_number: String,
        first_name: String,
        last_name: String,
    },

    GifExternal {
        url: String,
        q: String,
    },

    Document {
        flags: u32,
        id: Box<InputDocument>,
        caption: String,
        ttl_seconds: Option<i32>,
    },

    PhotoExternal {
        flags: u32,
        url: String,
        caption: String,
        ttl_seconds: Option<i32>,
    },

    Photo {
        flags: u32,
        id: Box<InputPhoto>,
        caption: String,
        ttl_seconds: Option<i32>,
    },

    UploadedDocument {
        flags: u32,
        nosound_video: bool,
        file: Box<InputFile>,
        thumb: Option<Box<InputFile>>,
        mime_type: String,
        attributes: Vec<DocumentAttribute>,
        caption: String,
        stickers: Option<Vec<InputDocument>>,
        ttl_seconds: Option<i32>,
    },

    GeoPoint {
        geo_point: Box<InputGeoPoint>,
    },

    Invoice {
        flags: u32,
        title: String,
        description: String,
        photo: Option<Box<InputWebDocument>>,
        invoice: Box<Invoice>,
        payload: Vec<u8>,
        provider: String,
        provider_data: Box<DataJson>,
        start_param: String,
    },

    Empty,

    GeoLive {
        geo_point: Box<InputGeoPoint>,
        period: i32,
    },

    Game {
        id: Box<InputGame>,
    },

    UploadedPhoto {
        flags: u32,
        file: Box<InputFile>,
        caption: String,
        stickers: Option<Vec<InputDocument>>,
        ttl_seconds: Option<i32>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum EncryptedFile {
    Empty,

    File {
        id: i64,
        access_hash: i64,
        size: i32,
        dc_id: i32,
        key_fingerprint: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthSentCode {
    pub flags: u32,
    pub phone_registered: bool,
    pub type_: Box<AuthSentCodeType>,
    pub phone_code_hash: String,
    pub next_type: Option<Box<AuthCodeType>>,
    pub timeout: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhoneCallProtocol {
    pub flags: u32,
    pub udp_p2p: bool,
    pub udp_reflector: bool,
    pub min_layer: i32,
    pub max_layer: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelBannedRights {
    pub flags: u32,
    pub view_messages: bool,
    pub send_messages: bool,
    pub send_media: bool,
    pub send_stickers: bool,
    pub send_gifs: bool,
    pub send_games: bool,
    pub send_inline: bool,
    pub embed_links: bool,
    pub until_date: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TopPeer {
    pub peer: Box<Peer>,
    pub rating: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputSingleMedia {
    pub media: Box<InputMedia>,
    pub random_id: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ExportedChatInvite {
    Exported {
        link: String,
    },

    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PhoneCall {
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

    Accepted {
        id: i64,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
        g_b: Vec<u8>,
        protocol: Box<PhoneCallProtocol>,
    },

    Waiting {
        flags: u32,
        id: i64,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
        protocol: Box<PhoneCallProtocol>,
        receive_date: Option<i32>,
    },

    Empty {
        id: i64,
    },

    Discarded {
        flags: u32,
        need_rating: bool,
        need_debug: bool,
        id: i64,
        reason: Option<Box<PhoneCallDiscardReason>>,
        duration: Option<i32>,
    },

    Requested {
        id: i64,
        access_hash: i64,
        date: i32,
        admin_id: i32,
        participant_id: i32,
        g_a_hash: Vec<u8>,
        protocol: Box<PhoneCallProtocol>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Chat {
    Channel {
        flags: u32,
        creator: bool,
        left: bool,
        editor: bool,
        broadcast: bool,
        verified: bool,
        megagroup: bool,
        restricted: bool,
        democracy: bool,
        signatures: bool,
        min: bool,
        id: i32,
        access_hash: Option<i64>,
        title: String,
        username: Option<String>,
        photo: Box<ChatPhoto>,
        date: i32,
        version: i32,
        restriction_reason: Option<String>,
        admin_rights: Option<Box<ChannelAdminRights>>,
        banned_rights: Option<Box<ChannelBannedRights>>,
        participants_count: Option<i32>,
        feed_id: Option<i32>,
    },

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
        photo: Box<ChatPhoto>,
        participants_count: i32,
        date: i32,
        version: i32,
        migrated_to: Option<Box<InputChannel>>,
    },

    Forbidden {
        id: i32,
        title: String,
    },

    ChannelForbidden {
        flags: u32,
        broadcast: bool,
        megagroup: bool,
        id: i32,
        access_hash: i64,
        title: String,
        until_date: Option<i32>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessagesChatFull {
    pub full_chat: Box<ChatFull>,
    pub chats: Vec<Chat>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PrivacyRule {
    ValueAllowContacts,

    ValueDisallowUsers {
        users: Vec<i32>,
    },

    ValueDisallowContacts,

    ValueAllowUsers {
        users: Vec<i32>,
    },

    ValueAllowAll,

    ValueDisallowAll,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthAuthorization {
    pub flags: u32,
    pub tmp_sessions: Option<i32>,
    pub user: Box<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputContact {
    Phone {
        client_id: i64,
        phone: String,
        first_name: String,
        last_name: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputNotifyPeer {
    Users,

    Chats,

    Peer {
        peer: Box<InputPeer>,
    },

    All,
}

#[derive(Debug, Clone, PartialEq)]
pub struct BotCommand {
    pub command: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageMedia {
    Invoice {
        flags: u32,
        shipping_address_requested: bool,
        test: bool,
        title: String,
        description: String,
        photo: Option<Box<WebDocument>>,
        receipt_msg_id: Option<i32>,
        currency: String,
        total_amount: i64,
        start_param: String,
    },

    Contact {
        phone_number: String,
        first_name: String,
        last_name: String,
        user_id: i32,
    },

    Empty,

    Document {
        flags: u32,
        document: Option<Box<Document>>,
        caption: Option<String>,
        ttl_seconds: Option<i32>,
    },

    Venue {
        geo: Box<GeoPoint>,
        title: String,
        address: String,
        provider: String,
        venue_id: String,
        venue_type: String,
    },

    Photo {
        flags: u32,
        photo: Option<Box<Photo>>,
        caption: Option<String>,
        ttl_seconds: Option<i32>,
    },

    WebPage {
        webpage: Box<WebPage>,
    },

    GeoLive {
        geo: Box<GeoPoint>,
        period: i32,
    },

    Unsupported,

    Geo {
        geo: Box<GeoPoint>,
    },

    Game {
        game: Box<Game>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputDialogPeer {
    Peer {
        peer: Box<InputPeer>,
    },

    Feed {
        feed_id: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ClientDhInnerData {
    pub nonce: i128,
    pub server_nonce: i128,
    pub retry_id: i64,
    pub g_b: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Peer {
    Channel {
        channel_id: i32,
    },

    User {
        user_id: i32,
    },

    Chat {
        chat_id: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaymentsPaymentForm {
    pub flags: u32,
    pub can_save_credentials: bool,
    pub password_missing: bool,
    pub bot_id: i32,
    pub invoice: Box<Invoice>,
    pub provider_id: i32,
    pub url: String,
    pub native_provider: Option<String>,
    pub native_params: Option<Box<DataJson>>,
    pub saved_info: Option<Box<PaymentRequestedInfo>>,
    pub saved_credentials: Option<Box<PaymentSavedCredentials>>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountDaysTtl {
    pub days: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountTmpPassword {
    pub tmp_password: Vec<u8>,
    pub valid_until: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Message {
        flags: u32,
        out: bool,
        mentioned: bool,
        media_unread: bool,
        silent: bool,
        post: bool,
        id: i32,
        from_id: Option<i32>,
        to_id: Box<Peer>,
        fwd_from: Option<Box<MessageFwdHeader>>,
        via_bot_id: Option<i32>,
        reply_to_msg_id: Option<i32>,
        date: i32,
        message: String,
        media: Option<Box<MessageMedia>>,
        reply_markup: Option<Box<ReplyMarkup>>,
        entities: Option<Vec<MessageEntity>>,
        views: Option<i32>,
        edit_date: Option<i32>,
        post_author: Option<String>,
        grouped_id: Option<i64>,
    },

    Empty {
        id: i32,
    },

    Service {
        flags: u32,
        out: bool,
        mentioned: bool,
        media_unread: bool,
        silent: bool,
        post: bool,
        id: i32,
        from_id: Option<i32>,
        to_id: Box<Peer>,
        reply_to_msg_id: Option<i32>,
        date: i32,
        action: Box<MessageAction>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct UpdatesState {
    pub pts: i32,
    pub qts: i32,
    pub date: i32,
    pub seq: i32,
    pub unread_count: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthPasswordRecovery {
    pub email_pattern: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputStickerSet {
    Id {
        id: i64,
        access_hash: i64,
    },

    ShortName {
        short_name: String,
    },

    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageEntity {
    Code {
        offset: i32,
        length: i32,
    },

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

    MentionName {
        offset: i32,
        length: i32,
        user_id: i32,
    },

    Email {
        offset: i32,
        length: i32,
    },

    Bold {
        offset: i32,
        length: i32,
    },

    BotCommand {
        offset: i32,
        length: i32,
    },

    TextUrl {
        offset: i32,
        length: i32,
        url: String,
    },

    Italic {
        offset: i32,
        length: i32,
    },

    Pre {
        offset: i32,
        length: i32,
        language: String,
    },

    InputMentionName {
        offset: i32,
        length: i32,
        user_id: Box<InputUser>,
    },

    Url {
        offset: i32,
        length: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MsgsStateInfo {
    pub req_msg_id: i64,
    pub info: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DocumentAttribute {
    Sticker {
        flags: u32,
        mask: bool,
        alt: String,
        stickerset: Box<InputStickerSet>,
        mask_coords: Option<Box<MaskCoords>>,
    },

    Video {
        flags: u32,
        round_message: bool,
        duration: i32,
        w: i32,
        h: i32,
    },

    ImageSize {
        w: i32,
        h: i32,
    },

    Filename {
        file_name: String,
    },

    HasStickers,

    Animated,

    Audio {
        flags: u32,
        voice: bool,
        duration: i32,
        title: Option<String>,
        performer: Option<String>,
        waveform: Option<Vec<u8>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct HelpRecentMeUrls {
    pub urls: Vec<RecentMeUrl>,
    pub chats: Vec<Chat>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsChannelParticipant {
    pub participant: Box<ChannelParticipant>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthCheckedPhone {
    pub phone_registered: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Game {
    pub flags: u32,
    pub id: i64,
    pub access_hash: i64,
    pub short_name: String,
    pub title: String,
    pub description: String,
    pub photo: Box<Photo>,
    pub document: Option<Box<Document>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DcOption {
    pub flags: u32,
    pub ipv6: bool,
    pub media_only: bool,
    pub tcpo_only: bool,
    pub cdn: bool,
    pub static_: bool,
    pub id: i32,
    pub ip_address: String,
    pub port: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentsPaymentResult {
    VerficationNeeded {
        url: String,
    },

    Result {
        updates: Box<Updates>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessagesArchivedStickers {
    pub count: i32,
    pub sets: Vec<StickerSetCovered>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdatesChannelDifference {
    Difference {
        flags: u32,
        final_: bool,
        pts: i32,
        timeout: Option<i32>,
        new_messages: Vec<Message>,
        other_updates: Vec<Update>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },

    Empty {
        flags: u32,
        final_: bool,
        pts: i32,
        timeout: Option<i32>,
    },

    TooLong {
        flags: u32,
        final_: bool,
        pts: i32,
        timeout: Option<i32>,
        top_message: i32,
        read_inbox_max_id: i32,
        read_outbox_max_id: i32,
        unread_count: i32,
        unread_mentions_count: i32,
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Authorization {
    pub hash: i64,
    pub flags: i32,
    pub device_model: String,
    pub platform: String,
    pub system_version: String,
    pub api_id: i32,
    pub app_name: String,
    pub app_version: String,
    pub date_created: i32,
    pub date_active: i32,
    pub ip: String,
    pub country: String,
    pub region: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct NearestDc {
    pub country: String,
    pub this_dc: i32,
    pub nearest_dc: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountPasswordInputSettings {
    pub flags: u32,
    pub new_salt: Option<Vec<u8>>,
    pub new_password_hash: Option<Vec<u8>>,
    pub hint: Option<String>,
    pub email: Option<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct WebDocument {
    pub url: String,
    pub access_hash: i64,
    pub size: i32,
    pub mime_type: String,
    pub attributes: Vec<DocumentAttribute>,
    pub dc_id: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct FutureSalts {
    pub req_msg_id: i64,
    pub now: i32,
    pub salts: Bare<Vec<Bare<FutureSalt>>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Page {
    Full {
        blocks: Vec<PageBlock>,
        photos: Vec<Photo>,
        documents: Vec<Document>,
    },

    Part {
        blocks: Vec<PageBlock>,
        photos: Vec<Photo>,
        documents: Vec<Document>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhoneConnection {
    pub id: i64,
    pub ip: String,
    pub ipv6: String,
    pub port: i32,
    pub peer_tag: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HelpInviteText {
    pub message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UpdatesDifference {
    Slice {
        new_messages: Vec<Message>,
        new_encrypted_messages: Vec<EncryptedMessage>,
        other_updates: Vec<Update>,
        chats: Vec<Chat>,
        users: Vec<User>,
        intermediate_state: Box<UpdatesState>,
    },

    TooLong {
        pts: i32,
    },

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
        state: Box<UpdatesState>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct CdnConfig {
    pub public_keys: Vec<CdnPublicKey>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessagesMessageEditData {
    pub flags: u32,
    pub caption: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessagesBotCallbackAnswer {
    pub flags: u32,
    pub alert: bool,
    pub has_url: bool,
    pub native_ui: bool,
    pub message: Option<String>,
    pub url: Option<String>,
    pub cache_time: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct CdnPublicKey {
    pub dc_id: i32,
    pub public_key: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct StickerSet {
    pub flags: u32,
    pub installed: bool,
    pub archived: bool,
    pub official: bool,
    pub masks: bool,
    pub id: i64,
    pub access_hash: i64,
    pub title: String,
    pub short_name: String,
    pub count: i32,
    pub hash: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WallPaper {
    Solid {
        id: i32,
        title: String,
        bg_color: i32,
        color: i32,
    },

    Paper {
        id: i32,
        title: String,
        sizes: Vec<PhotoSize>,
        color: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum User {
    User {
        flags: u32,
        self_: bool,
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
        access_hash: Option<i64>,
        first_name: Option<String>,
        last_name: Option<String>,
        username: Option<String>,
        phone: Option<String>,
        photo: Option<Box<UserProfilePhoto>>,
        status: Option<Box<UserStatus>>,
        bot_info_version: Option<i32>,
        restriction_reason: Option<String>,
        bot_inline_placeholder: Option<String>,
        lang_code: Option<String>,
    },

    Empty {
        id: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputEncryptedChat {
    pub chat_id: i32,
    pub access_hash: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessageRange {
    pub min_id: i32,
    pub max_id: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessagesAffectedHistory {
    pub pts: i32,
    pub pts_count: i32,
    pub offset: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContactsTopPeers {
    Peers {
        categories: Vec<TopPeerCategoryPeers>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },

    NotModified,
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountPrivacyRules {
    pub rules: Vec<PrivacyRule>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputFileLocation {
    Encrypted {
        id: i64,
        access_hash: i64,
    },

    Document {
        id: i64,
        access_hash: i64,
        version: i32,
    },

    Location {
        volume_id: i64,
        local_id: i32,
        secret: i64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaymentsValidatedRequestedInfo {
    pub flags: u32,
    pub id: Option<String>,
    pub shipping_options: Option<Vec<ShippingOption>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ResPq {
    pub nonce: i128,
    pub server_nonce: i128,
    pub pq: Vec<u8>,
    pub server_public_key_fingerprints: Vec<i64>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HighScore {
    pub pos: i32,
    pub user_id: i32,
    pub score: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UploadFile {
    CdnRedirect {
        dc_id: i32,
        file_token: Vec<u8>,
        encryption_key: Vec<u8>,
        encryption_iv: Vec<u8>,
        cdn_file_hashes: Vec<CdnFileHash>,
    },

    File {
        type_: Box<StorageFileType>,
        mtime: i32,
        bytes: Vec<u8>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PaymentSavedCredentials {
    Card {
        id: String,
        title: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessagesBotResults {
    pub flags: u32,
    pub gallery: bool,
    pub query_id: i64,
    pub next_offset: Option<String>,
    pub switch_pm: Option<Box<InlineBotSwitchPm>>,
    pub results: Vec<BotInlineResult>,
    pub cache_time: i32,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum HelpAppUpdate {
    No,

    Update {
        id: i32,
        critical: bool,
        url: String,
        text: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputEncryptedFile {
    BigUploaded {
        id: i64,
        parts: i32,
        key_fingerprint: i32,
    },

    File {
        id: i64,
        access_hash: i64,
    },

    Empty,

    Uploaded {
        id: i64,
        parts: i32,
        md5_checksum: String,
        key_fingerprint: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputPhoto {
    Photo {
        id: i64,
        access_hash: i64,
    },

    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Null;

#[derive(Debug, Clone, PartialEq)]
pub enum LangPackString {
    Pluralized {
        flags: u32,
        key: String,
        zero_value: Option<String>,
        one_value: Option<String>,
        two_value: Option<String>,
        few_value: Option<String>,
        many_value: Option<String>,
        other_value: String,
    },

    String {
        key: String,
        value: String,
    },

    Deleted {
        key: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessagesFoundGifs {
    pub next_offset: i32,
    pub results: Vec<FoundGif>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputPeerNotifyEvents {
    Empty,

    All,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MsgsAllInfo {
    pub msg_ids: Vec<i64>,
    pub info: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputPrivacyKey {
    ChatInvite,

    StatusTimestamp,

    PhoneCall,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagesSentEncryptedMessage {
    Message {
        date: i32,
    },

    File {
        date: i32,
        file: Box<EncryptedFile>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Config {
    pub flags: u32,
    pub phonecalls_enabled: bool,
    pub default_p2p_contacts: bool,
    pub date: i32,
    pub expires: i32,
    pub test_mode: bool,
    pub this_dc: i32,
    pub dc_options: Vec<DcOption>,
    pub chat_size_max: i32,
    pub megagroup_size_max: i32,
    pub forwarded_count_max: i32,
    pub online_update_period_ms: i32,
    pub offline_blur_timeout_ms: i32,
    pub offline_idle_timeout_ms: i32,
    pub online_cloud_timeout_ms: i32,
    pub notify_cloud_delay_ms: i32,
    pub notify_default_delay_ms: i32,
    pub chat_big_size: i32,
    pub push_chat_period_ms: i32,
    pub push_chat_limit: i32,
    pub saved_gifs_limit: i32,
    pub edit_time_limit: i32,
    pub rating_e_decay: i32,
    pub stickers_recent_limit: i32,
    pub stickers_faved_limit: i32,
    pub channels_read_media_period: i32,
    pub tmp_sessions: Option<i32>,
    pub pinned_dialogs_count_max: i32,
    pub call_receive_timeout_ms: i32,
    pub call_ring_timeout_ms: i32,
    pub call_connect_timeout_ms: i32,
    pub call_packet_timeout_ms: i32,
    pub me_url_prefix: String,
    pub suggested_lang_code: Option<String>,
    pub lang_pack_version: Option<i32>,
    pub disabled_features: Vec<DisabledFeature>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AuthSentCodeType {
    FlashCall {
        pattern: String,
    },

    App {
        length: i32,
    },

    Call {
        length: i32,
    },

    Sms {
        length: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChannelMessagesFilter {
    Empty,

    Filter {
        flags: u32,
        exclude_new_messages: bool,
        ranges: Vec<MessageRange>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct UserFull {
    pub flags: u32,
    pub blocked: bool,
    pub phone_calls_available: bool,
    pub phone_calls_private: bool,
    pub user: Box<User>,
    pub about: Option<String>,
    pub link: Box<ContactsLink>,
    pub profile_photo: Option<Box<Photo>>,
    pub notify_settings: Box<PeerNotifySettings>,
    pub bot_info: Option<Box<BotInfo>>,
    pub common_chats_count: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RecentMeUrl {
    Unknown {
        url: String,
    },

    User {
        url: String,
        user_id: i32,
    },

    Chat {
        url: String,
        chat_id: i32,
    },

    ChatInvite {
        url: String,
        chat_invite: Box<ChatInvite>,
    },

    StickerSet {
        url: String,
        set: Box<StickerSetCovered>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum FileLocation {
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

#[derive(Debug, Clone, PartialEq)]
pub struct FeedPosition {
    pub date: i32,
    pub peer: Box<Peer>,
    pub id: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AccountPassword {
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

#[derive(Debug, Clone, PartialEq)]
pub struct LangPackLanguage {
    pub name: String,
    pub native_name: String,
    pub lang_code: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PostAddress {
    pub street_line1: String,
    pub street_line2: String,
    pub city: String,
    pub state: String,
    pub country_iso2: String,
    pub post_code: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UploadCdnFile {
    ReuploadNeeded {
        request_token: Vec<u8>,
    },

    File {
        bytes: Vec<u8>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MaskCoords {
    pub n: i32,
    pub x: f64,
    pub y: f64,
    pub zoom: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ReceivedNotifyMessage {
    pub id: i32,
    pub flags: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PhonePhoneCall {
    pub phone_call: Box<PhoneCall>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaymentRequestedInfo {
    pub flags: u32,
    pub name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub shipping_address: Option<Box<PostAddress>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagesChats {
    Slice {
        count: i32,
        chats: Vec<Chat>,
    },

    Chats {
        chats: Vec<Chat>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ShippingOption {
    pub id: String,
    pub title: String,
    pub prices: Vec<LabeledPrice>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagesStickers {
    NotModified,

    Stickers {
        hash: String,
        stickers: Vec<Document>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SetClientDhParamsAnswer {
    DhGenOk {
        nonce: i128,
        server_nonce: i128,
        new_nonce_hash1: i128,
    },

    DhGenRetry {
        nonce: i128,
        server_nonce: i128,
        new_nonce_hash2: i128,
    },

    DhGenFail {
        nonce: i128,
        server_nonce: i128,
        new_nonce_hash3: i128,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputGeoPoint {
    Empty,

    Point {
        lat: f64,
        long: f64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct IpPort {
    pub ipv4: i32,
    pub port: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessagesPeerDialogs {
    pub dialogs: Vec<Dialog>,
    pub messages: Vec<Message>,
    pub chats: Vec<Chat>,
    pub users: Vec<User>,
    pub state: Box<UpdatesState>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum NotifyPeer {
    All,

    Users,

    Peer {
        peer: Box<Peer>,
    },

    Chats,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DialogPeer {
    Feed {
        feed_id: i32,
    },

    Peer {
        peer: Box<Peer>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct RpcError {
    pub error_code: i32,
    pub error_message: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatParticipants {
    Forbidden {
        flags: u32,
        chat_id: i32,
        self_participant: Option<Box<ChatParticipant>>,
    },

    Participants {
        chat_id: i32,
        participants: Vec<ChatParticipant>,
        version: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum DestroySessionRes {
    Ok {
        session_id: i64,
    },

    None {
        session_id: i64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct FutureSalt {
    pub valid_since: i32,
    pub valid_until: i32,
    pub salt: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChannelsChannelParticipants {
    NotModified,

    Participants {
        count: i32,
        participants: Vec<ChannelParticipant>,
        users: Vec<User>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pong {
    pub msg_id: i64,
    pub ping_id: i64,
}

#[derive(Debug, Clone, PartialEq)]
pub struct HelpConfigSimple {
    pub date: i32,
    pub expires: i32,
    pub dc_id: i32,
    pub ip_port_list: Vec<Bare<IpPort>>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct KeyboardButtonRow {
    pub buttons: Vec<KeyboardButton>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UserStatus {
    Recently,

    Offline {
        was_online: i32,
    },

    Online {
        expires: i32,
    },

    LastMonth,

    Empty,

    LastWeek,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReportReason {
    InputViolence,

    InputPornography,

    InputOther {
        text: String,
    },

    InputSpam,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputChannel {
    Empty,

    Channel {
        channel_id: i32,
        access_hash: i64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PhotoSize {
    Size {
        type_: String,
        location: Box<FileLocation>,
        w: i32,
        h: i32,
        size: i32,
    },

    Cached {
        type_: String,
        location: Box<FileLocation>,
        w: i32,
        h: i32,
        bytes: Vec<u8>,
    },

    Empty {
        type_: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum PageBlock {
    Subheader {
        text: Box<RichText>,
    },

    Divider,

    Blockquote {
        text: Box<RichText>,
        caption: Box<RichText>,
    },

    Anchor {
        name: String,
    },

    AuthorDate {
        author: Box<RichText>,
        published_date: i32,
    },

    EmbedPost {
        url: String,
        webpage_id: i64,
        author_photo_id: i64,
        author: String,
        date: i32,
        blocks: Vec<PageBlock>,
        caption: Box<RichText>,
    },

    Collage {
        items: Vec<PageBlock>,
        caption: Box<RichText>,
    },

    Unsupported,

    List {
        ordered: bool,
        items: Vec<RichText>,
    },

    Preformatted {
        text: Box<RichText>,
        language: String,
    },

    Channel {
        channel: Box<Chat>,
    },

    Embed {
        flags: u32,
        full_width: bool,
        allow_scrolling: bool,
        url: Option<String>,
        html: Option<String>,
        poster_photo_id: Option<i64>,
        w: i32,
        h: i32,
        caption: Box<RichText>,
    },

    Title {
        text: Box<RichText>,
    },

    Pullquote {
        text: Box<RichText>,
        caption: Box<RichText>,
    },

    Header {
        text: Box<RichText>,
    },

    Paragraph {
        text: Box<RichText>,
    },

    Cover {
        cover: Box<PageBlock>,
    },

    Slideshow {
        items: Vec<PageBlock>,
        caption: Box<RichText>,
    },

    Video {
        flags: u32,
        autoplay: bool,
        loop_: bool,
        video_id: i64,
        caption: Box<RichText>,
    },

    Footer {
        text: Box<RichText>,
    },

    Photo {
        photo_id: i64,
        caption: Box<RichText>,
    },

    Audio {
        audio_id: i64,
        caption: Box<RichText>,
    },

    Subtitle {
        text: Box<RichText>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct AuthExportedAuthorization {
    pub id: i32,
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContactsBlocked {
    Slice {
        count: i32,
        blocked: Vec<ContactBlocked>,
        users: Vec<User>,
    },

    Blocked {
        blocked: Vec<ContactBlocked>,
        users: Vec<User>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputGame {
    Id {
        id: i64,
        access_hash: i64,
    },

    ShortName {
        bot_id: Box<InputUser>,
        short_name: String,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagesFilter {
    InputGeo,

    InputPhotoVideo,

    InputPhoneCalls {
        flags: u32,
        missed: bool,
    },

    InputVideo,

    InputRoundVideo,

    InputVoice,

    InputPhotos,

    InputGif,

    InputRoundVoice,

    InputDocument,

    InputMusic,

    InputEmpty,

    InputContacts,

    InputMyMentions,

    InputChatPhotos,

    InputUrl,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputStickerSetItem {
    pub flags: u32,
    pub document: Box<InputDocument>,
    pub emoji: String,
    pub mask_coords: Option<Box<MaskCoords>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagesDialogs {
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

#[derive(Debug, Clone, PartialEq)]
pub enum ChannelParticipantsFilter {
    Admins,

    Search {
        q: String,
    },

    Bots,

    Banned {
        q: String,
    },

    Kicked {
        q: String,
    },

    Recent,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsAdminLogResults {
    pub events: Vec<ChannelAdminLogEvent>,
    pub chats: Vec<Chat>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputPrivacyRule {
    ValueDisallowContacts,

    ValueDisallowAll,

    ValueAllowContacts,

    ValueDisallowUsers {
        users: Vec<InputUser>,
    },

    ValueAllowAll,

    ValueAllowUsers {
        users: Vec<InputUser>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum Document {
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

    Empty {
        id: i64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagesMessages {
    ChannelMessages {
        flags: u32,
        pts: i32,
        count: i32,
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },

    MessagesSlice {
        count: i32,
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },

    MessagesNotModified {
        count: i32,
    },

    Messages {
        messages: Vec<Message>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PeerSettings {
    pub flags: u32,
    pub report_spam: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DraftMessage {
    Empty,

    Message {
        flags: u32,
        no_webpage: bool,
        reply_to_msg_id: Option<i32>,
        message: String,
        entities: Option<Vec<MessageEntity>>,
        date: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct MessagesStickerSet {
    pub set: Box<StickerSet>,
    pub packs: Vec<StickerPack>,
    pub documents: Vec<Document>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContactsLink {
    pub my_link: Box<ContactLink>,
    pub foreign_link: Box<ContactLink>,
    pub user: Box<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContactsFound {
    pub my_results: Vec<Peer>,
    pub results: Vec<Peer>,
    pub chats: Vec<Chat>,
    pub users: Vec<User>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BadMsgNotification {
    Notification {
        bad_msg_id: i64,
        bad_msg_seqno: i32,
        error_code: i32,
    },

    ServerSalt {
        bad_msg_id: i64,
        bad_msg_seqno: i32,
        error_code: i32,
        new_server_salt: i64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputPaymentCredentials {
    AndroidPay {
        payment_token: Box<DataJson>,
        google_transaction_id: String,
    },

    Credentials {
        flags: u32,
        save: bool,
        data: Box<DataJson>,
    },

    ApplePay {
        payment_data: Box<DataJson>,
    },

    Saved {
        id: String,
        tmp_password: Vec<u8>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContactBlocked {
    pub user_id: i32,
    pub date: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum StorageFileType {
    Unknown,

    Png,

    Pdf,

    Mp3,

    Gif,

    Webp,

    Mov,

    Jpeg,

    Mp4,

    Partial,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChannelsFeedSources {
    NotModified,

    Sources {
        flags: u32,
        newly_joined_feed: Option<i32>,
        feeds: Vec<FeedBroadcasts>,
        chats: Vec<Chat>,
        users: Vec<User>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct AccountPasswordSettings {
    pub email: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChatInvite {
    Invite {
        flags: u32,
        channel: bool,
        broadcast: bool,
        public: bool,
        megagroup: bool,
        title: String,
        photo: Box<ChatPhoto>,
        participants_count: i32,
        participants: Option<Vec<User>>,
    },

    Already {
        chat: Box<Chat>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum ReplyMarkup {
    Keyboard {
        flags: u32,
        resize: bool,
        single_use: bool,
        selective: bool,
        rows: Vec<KeyboardButtonRow>,
    },

    KeyboardHide {
        flags: u32,
        selective: bool,
    },

    Inline {
        rows: Vec<KeyboardButtonRow>,
    },

    KeyboardForceReply {
        flags: u32,
        single_use: bool,
        selective: bool,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct BotInfo {
    pub user_id: i32,
    pub description: String,
    pub commands: Vec<BotCommand>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InputPeer {
    Self_,

    Empty,

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

#[derive(Debug, Clone, PartialEq)]
pub enum ChatPhoto {
    Empty,

    Photo {
        photo_small: Box<FileLocation>,
        photo_big: Box<FileLocation>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum GeoPoint {
    Point {
        long: f64,
        lat: f64,
    },

    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum WebPage {
    NotModified,

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
        type_: Option<String>,
        site_name: Option<String>,
        title: Option<String>,
        description: Option<String>,
        photo: Option<Box<Photo>>,
        embed_url: Option<String>,
        embed_type: Option<String>,
        embed_width: Option<i32>,
        embed_height: Option<i32>,
        duration: Option<i32>,
        author: Option<String>,
        document: Option<Box<Document>>,
        cached_page: Option<Box<Page>>,
    },

    Empty {
        id: i64,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub enum AuthCodeType {
    Call,

    FlashCall,

    Sms,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InputWebDocument {
    pub url: String,
    pub size: i32,
    pub mime_type: String,
    pub attributes: Vec<DocumentAttribute>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MsgDetailedInfo {
    New {
        answer_msg_id: i64,
        bytes: i32,
        status: i32,
    },

    Info {
        msg_id: i64,
        answer_msg_id: i64,
        bytes: i32,
        status: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct CdnFileHash {
    pub offset: i32,
    pub limit: i32,
    pub hash: Vec<u8>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub code: i32,
    pub text: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ServerDhInnerData {
    pub nonce: i128,
    pub server_nonce: i128,
    pub g: i32,
    pub dh_prime: Vec<u8>,
    pub g_a: Vec<u8>,
    pub server_time: i32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagesSavedGifs {
    Gifs {
        hash: i32,
        gifs: Vec<Document>,
    },

    NotModified,
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaymentCharge {
    pub id: String,
    pub provider_charge_id: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Update {
    PinnedDialogs {
        flags: u32,
        order: Option<Vec<DialogPeer>>,
    },

    ContactRegistered {
        user_id: i32,
        date: i32,
    },

    UserTyping {
        user_id: i32,
        action: Box<SendMessageAction>,
    },

    ContactLink {
        user_id: i32,
        my_link: Box<ContactLink>,
        foreign_link: Box<ContactLink>,
    },

    DeleteChannelMessages {
        channel_id: i32,
        messages: Vec<i32>,
        pts: i32,
        pts_count: i32,
    },

    SavedGifs,

    EditMessage {
        message: Box<Message>,
        pts: i32,
        pts_count: i32,
    },

    DeleteMessages {
        messages: Vec<i32>,
        pts: i32,
        pts_count: i32,
    },

    ChannelMessageViews {
        channel_id: i32,
        id: i32,
        views: i32,
    },

    BotInlineQuery {
        flags: u32,
        query_id: i64,
        user_id: i32,
        query: String,
        geo: Option<Box<GeoPoint>>,
        offset: String,
    },

    Privacy {
        key: Box<PrivacyKey>,
        rules: Vec<PrivacyRule>,
    },

    BotWebhookJson {
        data: Box<DataJson>,
    },

    ReadFeed {
        feed_id: i32,
        max_position: Box<FeedPosition>,
    },

    ContactsReset,

    BotInlineSend {
        flags: u32,
        user_id: i32,
        query: String,
        geo: Option<Box<GeoPoint>>,
        id: String,
        msg_id: Option<Box<InputBotInlineMessageId>>,
    },

    ChannelWebPage {
        channel_id: i32,
        webpage: Box<WebPage>,
        pts: i32,
        pts_count: i32,
    },

    ChannelPinnedMessage {
        channel_id: i32,
        id: i32,
    },

    DialogPinned {
        flags: u32,
        pinned: bool,
        peer: Box<DialogPeer>,
    },

    BotShippingQuery {
        query_id: i64,
        user_id: i32,
        payload: Vec<u8>,
        shipping_address: Box<PostAddress>,
    },

    ChatUserTyping {
        chat_id: i32,
        user_id: i32,
        action: Box<SendMessageAction>,
    },

    BotWebhookJsonquery {
        query_id: i64,
        data: Box<DataJson>,
        timeout: i32,
    },

    FavedStickers,

    UserStatus {
        user_id: i32,
        status: Box<UserStatus>,
    },

    DraftMessage {
        peer: Box<Peer>,
        draft: Box<DraftMessage>,
    },

    NewChannelMessage {
        message: Box<Message>,
        pts: i32,
        pts_count: i32,
    },

    UserBlocked {
        user_id: i32,
        blocked: bool,
    },

    ReadChannelOutbox {
        channel_id: i32,
        max_id: i32,
    },

    PhoneCall {
        phone_call: Box<PhoneCall>,
    },

    ReadFeaturedStickers,

    ReadMessagesContents {
        messages: Vec<i32>,
        pts: i32,
        pts_count: i32,
    },

    StickerSetsOrder {
        flags: u32,
        masks: bool,
        order: Vec<i64>,
    },

    LangPackTooLong,

    ChannelTooLong {
        flags: u32,
        channel_id: i32,
        pts: Option<i32>,
    },

    RecentStickers,

    NotifySettings {
        peer: Box<NotifyPeer>,
        notify_settings: Box<PeerNotifySettings>,
    },

    ReadHistoryOutbox {
        peer: Box<Peer>,
        max_id: i32,
        pts: i32,
        pts_count: i32,
    },

    Config,

    WebPage {
        webpage: Box<WebPage>,
        pts: i32,
        pts_count: i32,
    },

    NewMessage {
        message: Box<Message>,
        pts: i32,
        pts_count: i32,
    },

    EncryptedMessagesRead {
        chat_id: i32,
        max_date: i32,
        date: i32,
    },

    DcOptions {
        dc_options: Vec<DcOption>,
    },

    ServiceNotification {
        flags: u32,
        popup: bool,
        inbox_date: Option<i32>,
        type_: String,
        message: String,
        media: Box<MessageMedia>,
        entities: Vec<MessageEntity>,
    },

    EncryptedChatTyping {
        chat_id: i32,
    },

    ReadChannelInbox {
        channel_id: i32,
        max_id: i32,
    },

    MessageId {
        id: i32,
        random_id: i64,
    },

    LangPack {
        difference: Box<LangPackDifference>,
    },

    ChatParticipantAdmin {
        chat_id: i32,
        user_id: i32,
        is_admin: bool,
        version: i32,
    },

    UserName {
        user_id: i32,
        first_name: String,
        last_name: String,
        username: String,
    },

    ChatParticipantAdd {
        chat_id: i32,
        user_id: i32,
        inviter_id: i32,
        date: i32,
        version: i32,
    },

    Encryption {
        chat: Box<EncryptedChat>,
        date: i32,
    },

    BotPrecheckoutQuery {
        flags: u32,
        query_id: i64,
        user_id: i32,
        payload: Vec<u8>,
        info: Option<Box<PaymentRequestedInfo>>,
        shipping_option_id: Option<String>,
        currency: String,
        total_amount: i64,
    },

    NewEncryptedMessage {
        message: Box<EncryptedMessage>,
        qts: i32,
    },

    Channel {
        channel_id: i32,
    },

    NewStickerSet {
        stickerset: Box<MessagesStickerSet>,
    },

    ReadHistoryInbox {
        peer: Box<Peer>,
        max_id: i32,
        pts: i32,
        pts_count: i32,
    },

    ChannelAvailableMessages {
        channel_id: i32,
        available_min_id: i32,
    },

    ChatAdmins {
        chat_id: i32,
        enabled: bool,
        version: i32,
    },

    UserPhone {
        user_id: i32,
        phone: String,
    },

    BotCallbackQuery {
        flags: u32,
        query_id: i64,
        user_id: i32,
        peer: Box<Peer>,
        msg_id: i32,
        chat_instance: i64,
        data: Option<Vec<u8>>,
        game_short_name: Option<String>,
    },

    ChatParticipantDelete {
        chat_id: i32,
        user_id: i32,
        version: i32,
    },

    InlineBotCallbackQuery {
        flags: u32,
        query_id: i64,
        user_id: i32,
        msg_id: Box<InputBotInlineMessageId>,
        chat_instance: i64,
        data: Option<Vec<u8>>,
        game_short_name: Option<String>,
    },

    ChatParticipants {
        participants: Box<ChatParticipants>,
    },

    StickerSets,

    PtsChanged,

    ChannelReadMessagesContents {
        channel_id: i32,
        messages: Vec<i32>,
    },

    UserPhoto {
        user_id: i32,
        date: i32,
        photo: Box<UserProfilePhoto>,
        previous: bool,
    },

    EditChannelMessage {
        message: Box<Message>,
        pts: i32,
        pts_count: i32,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PaymentsSavedInfo {
    pub flags: u32,
    pub has_saved_credentials: bool,
    pub saved_info: Option<Box<PaymentRequestedInfo>>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessagesDhConfig {
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

#[derive(Debug, Clone, PartialEq)]
pub struct MessageFwdHeader {
    pub flags: u32,
    pub from_id: Option<i32>,
    pub date: i32,
    pub channel_id: Option<i32>,
    pub channel_post: Option<i32>,
    pub post_author: Option<String>,
    pub saved_from_peer: Option<Box<Peer>>,
    pub saved_from_msg_id: Option<i32>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContactsContacts {
    Contacts {
        contacts: Vec<Contact>,
        saved_count: i32,
        users: Vec<User>,
    },

    ContactsNotModified,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExportedMessageLink {
    pub link: String,
    pub html: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Dialog {
    Feed {
        flags: u32,
        pinned: bool,
        peer: Box<Peer>,
        top_message: i32,
        feed_id: i32,
        feed_other_channels: Vec<i32>,
        read_max_position: Option<Box<FeedPosition>>,
        unread_count: i32,
        unread_muted_count: i32,
    },

    Dialog {
        flags: u32,
        pinned: bool,
        peer: Box<Peer>,
        top_message: i32,
        read_inbox_max_id: i32,
        read_outbox_max_id: i32,
        unread_count: i32,
        unread_mentions_count: i32,
        notify_settings: Box<PeerNotifySettings>,
        pts: Option<i32>,
        draft: Option<Box<DraftMessage>>,
    },
}

#[derive(Debug, Clone, PartialEq)]
pub struct PQInnerData {
    pub pq: Vec<u8>,
    pub p: Vec<u8>,
    pub q: Vec<u8>,
    pub nonce: i128,
    pub server_nonce: i128,
    pub new_nonce: [u8; 32],
}

#[derive(Debug, Clone, PartialEq)]
pub enum PeerNotifyEvents {
    All,

    Empty,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ContactLink {
    HasPhone,

    Contact,

    Unknown,

    None,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DestroyAuthKeyRes {
    Ok,

    None,

    Fail,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InlineBotSwitchPm {
    pub text: String,
    pub start_param: String,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Contact {
    pub user_id: i32,
    pub mutual: bool,
}

