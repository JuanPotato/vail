#[derive(Debug, Clone, PartialEq)]
pub struct ReqPq {
    pub nonce: i128,
}

impl Method for ReqPq {
    type ReturnType = types::ResPq;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ReqDhParams {
    pub nonce: i128,
    pub server_nonce: i128,
    pub p: Vec<u8>,
    pub q: Vec<u8>,
    pub public_key_fingerprint: i64,
    pub encrypted_data: Vec<u8>,
}

impl Method for ReqDhParams {
    type ReturnType = types::ServerDhParams;
}


#[derive(Debug, Clone, PartialEq)]
pub struct SetClientDhParams {
    pub nonce: i128,
    pub server_nonce: i128,
    pub encrypted_data: Vec<u8>,
}

impl Method for SetClientDhParams {
    type ReturnType = types::SetClientDhParamsAnswer;
}


#[derive(Debug, Clone, PartialEq)]
pub struct DestroyAuthKey;

impl Method for DestroyAuthKey {
    type ReturnType = types::DestroyAuthKeyRes;
}


#[derive(Debug, Clone, PartialEq)]
pub struct RpcDropAnswer {
    pub req_msg_id: i64,
}

impl Method for RpcDropAnswer {
    type ReturnType = types::RpcDropAnswer;
}


#[derive(Debug, Clone, PartialEq)]
pub struct GetFutureSalts {
    pub num: i32,
}

impl Method for GetFutureSalts {
    type ReturnType = types::FutureSalts;
}


#[derive(Debug, Clone, PartialEq)]
pub struct Ping {
    pub ping_id: i64,
}

impl Method for Ping {
    type ReturnType = types::Pong;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PingDelayDisconnect {
    pub ping_id: i64,
    pub disconnect_delay: i32,
}

impl Method for PingDelayDisconnect {
    type ReturnType = types::Pong;
}


#[derive(Debug, Clone, PartialEq)]
pub struct DestroySession {
    pub session_id: i64,
}

impl Method for DestroySession {
    type ReturnType = types::DestroySessionRes;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContestSaveDeveloperInfo {
    pub vk_id: i32,
    pub name: String,
    pub phone_number: String,
    pub age: i32,
    pub city: String,
}

impl Method for ContestSaveDeveloperInfo {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthCheckPhone {
    pub phone_number: String,
}

impl Method for AuthCheckPhone {
    type ReturnType = types::AuthCheckedPhone;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthSendCode {
    pub flags: u32,
    pub allow_flashcall: bool,
    pub phone_number: String,
    pub current_number: Option<bool>,
    pub api_id: i32,
    pub api_hash: String,
}

impl Method for AuthSendCode {
    type ReturnType = types::AuthSentCode;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthSignUp {
    pub phone_number: String,
    pub phone_code_hash: String,
    pub phone_code: String,
    pub first_name: String,
    pub last_name: String,
}

impl Method for AuthSignUp {
    type ReturnType = types::AuthAuthorization;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthSignIn {
    pub phone_number: String,
    pub phone_code_hash: String,
    pub phone_code: String,
}

impl Method for AuthSignIn {
    type ReturnType = types::AuthAuthorization;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthLogOut;

impl Method for AuthLogOut {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthResetAuthorizations;

impl Method for AuthResetAuthorizations {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthSendInvites {
    pub phone_numbers: Vec<String>,
    pub message: String,
}

impl Method for AuthSendInvites {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthExportAuthorization {
    pub dc_id: i32,
}

impl Method for AuthExportAuthorization {
    type ReturnType = types::AuthExportedAuthorization;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthImportAuthorization {
    pub id: i32,
    pub bytes: Vec<u8>,
}

impl Method for AuthImportAuthorization {
    type ReturnType = types::AuthAuthorization;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthBindTempAuthKey {
    pub perm_auth_key_id: i64,
    pub nonce: i64,
    pub expires_at: i32,
    pub encrypted_message: Vec<u8>,
}

impl Method for AuthBindTempAuthKey {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthImportBotAuthorization {
    pub flags: i32,
    pub api_id: i32,
    pub api_hash: String,
    pub bot_auth_token: String,
}

impl Method for AuthImportBotAuthorization {
    type ReturnType = types::AuthAuthorization;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthCheckPassword {
    pub password_hash: Vec<u8>,
}

impl Method for AuthCheckPassword {
    type ReturnType = types::AuthAuthorization;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthRequestPasswordRecovery;

impl Method for AuthRequestPasswordRecovery {
    type ReturnType = types::AuthPasswordRecovery;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthRecoverPassword {
    pub code: String,
}

impl Method for AuthRecoverPassword {
    type ReturnType = types::AuthAuthorization;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthResendCode {
    pub phone_number: String,
    pub phone_code_hash: String,
}

impl Method for AuthResendCode {
    type ReturnType = types::AuthSentCode;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthCancelCode {
    pub phone_number: String,
    pub phone_code_hash: String,
}

impl Method for AuthCancelCode {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AuthDropTempAuthKeys {
    pub except_auth_keys: Vec<i64>,
}

impl Method for AuthDropTempAuthKeys {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountRegisterDevice {
    pub token_type: i32,
    pub token: String,
    pub app_sandbox: bool,
    pub other_uids: Vec<i32>,
}

impl Method for AccountRegisterDevice {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountUnregisterDevice {
    pub token_type: i32,
    pub token: String,
    pub other_uids: Vec<i32>,
}

impl Method for AccountUnregisterDevice {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountUpdateNotifySettings {
    pub peer: Box<InputNotifyPeer>,
    pub settings: Box<InputPeerNotifySettings>,
}

impl Method for AccountUpdateNotifySettings {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountGetNotifySettings {
    pub peer: Box<InputNotifyPeer>,
}

impl Method for AccountGetNotifySettings {
    type ReturnType = types::PeerNotifySettings;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountResetNotifySettings;

impl Method for AccountResetNotifySettings {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountUpdateProfile {
    pub flags: u32,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub about: Option<String>,
}

impl Method for AccountUpdateProfile {
    type ReturnType = types::User;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountUpdateStatus {
    pub offline: bool,
}

impl Method for AccountUpdateStatus {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountGetWallPapers;

impl Method for AccountGetWallPapers {
    type ReturnType = WallPaper;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountReportPeer {
    pub peer: Box<InputPeer>,
    pub reason: Box<ReportReason>,
}

impl Method for AccountReportPeer {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountCheckUsername {
    pub username: String,
}

impl Method for AccountCheckUsername {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountUpdateUsername {
    pub username: String,
}

impl Method for AccountUpdateUsername {
    type ReturnType = types::User;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountGetPrivacy {
    pub key: Box<InputPrivacyKey>,
}

impl Method for AccountGetPrivacy {
    type ReturnType = types::AccountPrivacyRules;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountSetPrivacy {
    pub key: Box<InputPrivacyKey>,
    pub rules: Vec<InputPrivacyRule>,
}

impl Method for AccountSetPrivacy {
    type ReturnType = types::AccountPrivacyRules;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountDeleteAccount {
    pub reason: String,
}

impl Method for AccountDeleteAccount {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountGetAccountTtl;

impl Method for AccountGetAccountTtl {
    type ReturnType = types::AccountDaysTtl;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountSetAccountTtl {
    pub ttl: Box<AccountDaysTtl>,
}

impl Method for AccountSetAccountTtl {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountSendChangePhoneCode {
    pub flags: u32,
    pub allow_flashcall: bool,
    pub phone_number: String,
    pub current_number: Option<bool>,
}

impl Method for AccountSendChangePhoneCode {
    type ReturnType = types::AuthSentCode;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountChangePhone {
    pub phone_number: String,
    pub phone_code_hash: String,
    pub phone_code: String,
}

impl Method for AccountChangePhone {
    type ReturnType = types::User;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountUpdateDeviceLocked {
    pub period: i32,
}

impl Method for AccountUpdateDeviceLocked {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountGetAuthorizations;

impl Method for AccountGetAuthorizations {
    type ReturnType = types::AccountAuthorizations;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountResetAuthorization {
    pub hash: i64,
}

impl Method for AccountResetAuthorization {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountGetPassword;

impl Method for AccountGetPassword {
    type ReturnType = types::AccountPassword;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountGetPasswordSettings {
    pub current_password_hash: Vec<u8>,
}

impl Method for AccountGetPasswordSettings {
    type ReturnType = types::AccountPasswordSettings;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountUpdatePasswordSettings {
    pub current_password_hash: Vec<u8>,
    pub new_settings: Box<AccountPasswordInputSettings>,
}

impl Method for AccountUpdatePasswordSettings {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountSendConfirmPhoneCode {
    pub flags: u32,
    pub allow_flashcall: bool,
    pub hash: String,
    pub current_number: Option<bool>,
}

impl Method for AccountSendConfirmPhoneCode {
    type ReturnType = types::AuthSentCode;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountConfirmPhone {
    pub phone_code_hash: String,
    pub phone_code: String,
}

impl Method for AccountConfirmPhone {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct AccountGetTmpPassword {
    pub password_hash: Vec<u8>,
    pub period: i32,
}

impl Method for AccountGetTmpPassword {
    type ReturnType = types::AccountTmpPassword;
}


#[derive(Debug, Clone, PartialEq)]
pub struct UsersGetUsers {
    pub id: Vec<InputUser>,
}

impl Method for UsersGetUsers {
    type ReturnType = User;
}


#[derive(Debug, Clone, PartialEq)]
pub struct UsersGetFullUser {
    pub id: Box<InputUser>,
}

impl Method for UsersGetFullUser {
    type ReturnType = types::UserFull;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsGetStatuses;

impl Method for ContactsGetStatuses {
    type ReturnType = ContactStatus;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsGetContacts {
    pub hash: i32,
}

impl Method for ContactsGetContacts {
    type ReturnType = types::ContactsContacts;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsImportContacts {
    pub contacts: Vec<InputContact>,
}

impl Method for ContactsImportContacts {
    type ReturnType = types::ContactsImportedContacts;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsDeleteContact {
    pub id: Box<InputUser>,
}

impl Method for ContactsDeleteContact {
    type ReturnType = types::ContactsLink;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsDeleteContacts {
    pub id: Vec<InputUser>,
}

impl Method for ContactsDeleteContacts {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsBlock {
    pub id: Box<InputUser>,
}

impl Method for ContactsBlock {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsUnblock {
    pub id: Box<InputUser>,
}

impl Method for ContactsUnblock {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsGetBlocked {
    pub offset: i32,
    pub limit: i32,
}

impl Method for ContactsGetBlocked {
    type ReturnType = types::ContactsBlocked;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsExportCard;

impl Method for ContactsExportCard {
    type ReturnType = i32;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsImportCard {
    pub export_card: Vec<i32>,
}

impl Method for ContactsImportCard {
    type ReturnType = types::User;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsSearch {
    pub q: String,
    pub limit: i32,
}

impl Method for ContactsSearch {
    type ReturnType = types::ContactsFound;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsResolveUsername {
    pub username: String,
}

impl Method for ContactsResolveUsername {
    type ReturnType = types::ContactsResolvedPeer;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsGetTopPeers {
    pub flags: u32,
    pub correspondents: bool,
    pub bots_pm: bool,
    pub bots_inline: bool,
    pub phone_calls: bool,
    pub groups: bool,
    pub channels: bool,
    pub offset: i32,
    pub limit: i32,
    pub hash: i32,
}

impl Method for ContactsGetTopPeers {
    type ReturnType = types::ContactsTopPeers;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsResetTopPeerRating {
    pub category: Box<TopPeerCategory>,
    pub peer: Box<InputPeer>,
}

impl Method for ContactsResetTopPeerRating {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ContactsResetSaved;

impl Method for ContactsResetSaved {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetMessages {
    pub id: Vec<i32>,
}

impl Method for MessagesGetMessages {
    type ReturnType = types::MessagesMessages;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetDialogs {
    pub flags: u32,
    pub exclude_pinned: bool,
    pub feed_id: Option<i32>,
    pub offset_date: i32,
    pub offset_id: i32,
    pub offset_peer: Box<InputPeer>,
    pub limit: i32,
}

impl Method for MessagesGetDialogs {
    type ReturnType = types::MessagesDialogs;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetHistory {
    pub peer: Box<InputPeer>,
    pub offset_id: i32,
    pub offset_date: i32,
    pub add_offset: i32,
    pub limit: i32,
    pub max_id: i32,
    pub min_id: i32,
    pub hash: i32,
}

impl Method for MessagesGetHistory {
    type ReturnType = types::MessagesMessages;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSearch {
    pub flags: u32,
    pub peer: Box<InputPeer>,
    pub q: String,
    pub from_id: Option<Box<InputUser>>,
    pub filter: Box<MessagesFilter>,
    pub min_date: i32,
    pub max_date: i32,
    pub offset_id: i32,
    pub add_offset: i32,
    pub limit: i32,
    pub max_id: i32,
    pub min_id: i32,
}

impl Method for MessagesSearch {
    type ReturnType = types::MessagesMessages;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesReadHistory {
    pub peer: Box<InputPeer>,
    pub max_id: i32,
}

impl Method for MessagesReadHistory {
    type ReturnType = types::MessagesAffectedMessages;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesDeleteHistory {
    pub flags: u32,
    pub just_clear: bool,
    pub peer: Box<InputPeer>,
    pub max_id: i32,
}

impl Method for MessagesDeleteHistory {
    type ReturnType = types::MessagesAffectedHistory;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesDeleteMessages {
    pub flags: u32,
    pub revoke: bool,
    pub id: Vec<i32>,
}

impl Method for MessagesDeleteMessages {
    type ReturnType = types::MessagesAffectedMessages;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesReceivedMessages {
    pub max_id: i32,
}

impl Method for MessagesReceivedMessages {
    type ReturnType = ReceivedNotifyMessage;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSetTyping {
    pub peer: Box<InputPeer>,
    pub action: Box<SendMessageAction>,
}

impl Method for MessagesSetTyping {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSendMessage {
    pub flags: u32,
    pub no_webpage: bool,
    pub silent: bool,
    pub background: bool,
    pub clear_draft: bool,
    pub peer: Box<InputPeer>,
    pub reply_to_msg_id: Option<i32>,
    pub message: String,
    pub random_id: i64,
    pub reply_markup: Option<Box<ReplyMarkup>>,
    pub entities: Option<Vec<MessageEntity>>,
}

impl Method for MessagesSendMessage {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSendMedia {
    pub flags: u32,
    pub silent: bool,
    pub background: bool,
    pub clear_draft: bool,
    pub peer: Box<InputPeer>,
    pub reply_to_msg_id: Option<i32>,
    pub media: Box<InputMedia>,
    pub random_id: i64,
    pub reply_markup: Option<Box<ReplyMarkup>>,
}

impl Method for MessagesSendMedia {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesForwardMessages {
    pub flags: u32,
    pub silent: bool,
    pub background: bool,
    pub with_my_score: bool,
    pub grouped: bool,
    pub from_peer: Box<InputPeer>,
    pub id: Vec<i32>,
    pub random_id: Vec<i64>,
    pub to_peer: Box<InputPeer>,
}

impl Method for MessagesForwardMessages {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesReportSpam {
    pub peer: Box<InputPeer>,
}

impl Method for MessagesReportSpam {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesHideReportSpam {
    pub peer: Box<InputPeer>,
}

impl Method for MessagesHideReportSpam {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetPeerSettings {
    pub peer: Box<InputPeer>,
}

impl Method for MessagesGetPeerSettings {
    type ReturnType = types::PeerSettings;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetChats {
    pub id: Vec<i32>,
}

impl Method for MessagesGetChats {
    type ReturnType = types::MessagesChats;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetFullChat {
    pub chat_id: i32,
}

impl Method for MessagesGetFullChat {
    type ReturnType = types::MessagesChatFull;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesEditChatTitle {
    pub chat_id: i32,
    pub title: String,
}

impl Method for MessagesEditChatTitle {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesEditChatPhoto {
    pub chat_id: i32,
    pub photo: Box<InputChatPhoto>,
}

impl Method for MessagesEditChatPhoto {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesAddChatUser {
    pub chat_id: i32,
    pub user_id: Box<InputUser>,
    pub fwd_limit: i32,
}

impl Method for MessagesAddChatUser {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesDeleteChatUser {
    pub chat_id: i32,
    pub user_id: Box<InputUser>,
}

impl Method for MessagesDeleteChatUser {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesCreateChat {
    pub users: Vec<InputUser>,
    pub title: String,
}

impl Method for MessagesCreateChat {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesForwardMessage {
    pub peer: Box<InputPeer>,
    pub id: i32,
    pub random_id: i64,
}

impl Method for MessagesForwardMessage {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetDhConfig {
    pub version: i32,
    pub random_length: i32,
}

impl Method for MessagesGetDhConfig {
    type ReturnType = types::MessagesDhConfig;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesRequestEncryption {
    pub user_id: Box<InputUser>,
    pub random_id: i32,
    pub g_a: Vec<u8>,
}

impl Method for MessagesRequestEncryption {
    type ReturnType = types::EncryptedChat;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesAcceptEncryption {
    pub peer: Box<InputEncryptedChat>,
    pub g_b: Vec<u8>,
    pub key_fingerprint: i64,
}

impl Method for MessagesAcceptEncryption {
    type ReturnType = types::EncryptedChat;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesDiscardEncryption {
    pub chat_id: i32,
}

impl Method for MessagesDiscardEncryption {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSetEncryptedTyping {
    pub peer: Box<InputEncryptedChat>,
    pub typing: bool,
}

impl Method for MessagesSetEncryptedTyping {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesReadEncryptedHistory {
    pub peer: Box<InputEncryptedChat>,
    pub max_date: i32,
}

impl Method for MessagesReadEncryptedHistory {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSendEncrypted {
    pub peer: Box<InputEncryptedChat>,
    pub random_id: i64,
    pub data: Vec<u8>,
}

impl Method for MessagesSendEncrypted {
    type ReturnType = types::MessagesSentEncryptedMessage;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSendEncryptedFile {
    pub peer: Box<InputEncryptedChat>,
    pub random_id: i64,
    pub data: Vec<u8>,
    pub file: Box<InputEncryptedFile>,
}

impl Method for MessagesSendEncryptedFile {
    type ReturnType = types::MessagesSentEncryptedMessage;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSendEncryptedService {
    pub peer: Box<InputEncryptedChat>,
    pub random_id: i64,
    pub data: Vec<u8>,
}

impl Method for MessagesSendEncryptedService {
    type ReturnType = types::MessagesSentEncryptedMessage;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesReceivedQueue {
    pub max_qts: i32,
}

impl Method for MessagesReceivedQueue {
    type ReturnType = i64;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesReportEncryptedSpam {
    pub peer: Box<InputEncryptedChat>,
}

impl Method for MessagesReportEncryptedSpam {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesReadMessageContents {
    pub id: Vec<i32>,
}

impl Method for MessagesReadMessageContents {
    type ReturnType = types::MessagesAffectedMessages;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetAllStickers {
    pub hash: i32,
}

impl Method for MessagesGetAllStickers {
    type ReturnType = types::MessagesAllStickers;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetWebPagePreview {
    pub message: String,
}

impl Method for MessagesGetWebPagePreview {
    type ReturnType = types::MessageMedia;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesExportChatInvite {
    pub chat_id: i32,
}

impl Method for MessagesExportChatInvite {
    type ReturnType = types::ExportedChatInvite;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesCheckChatInvite {
    pub hash: String,
}

impl Method for MessagesCheckChatInvite {
    type ReturnType = types::ChatInvite;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesImportChatInvite {
    pub hash: String,
}

impl Method for MessagesImportChatInvite {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetStickerSet {
    pub stickerset: Box<InputStickerSet>,
}

impl Method for MessagesGetStickerSet {
    type ReturnType = types::MessagesStickerSet;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesInstallStickerSet {
    pub stickerset: Box<InputStickerSet>,
    pub archived: bool,
}

impl Method for MessagesInstallStickerSet {
    type ReturnType = types::MessagesStickerSetInstallResult;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesUninstallStickerSet {
    pub stickerset: Box<InputStickerSet>,
}

impl Method for MessagesUninstallStickerSet {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesStartBot {
    pub bot: Box<InputUser>,
    pub peer: Box<InputPeer>,
    pub random_id: i64,
    pub start_param: String,
}

impl Method for MessagesStartBot {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetMessagesViews {
    pub peer: Box<InputPeer>,
    pub id: Vec<i32>,
    pub increment: bool,
}

impl Method for MessagesGetMessagesViews {
    type ReturnType = i32;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesToggleChatAdmins {
    pub chat_id: i32,
    pub enabled: bool,
}

impl Method for MessagesToggleChatAdmins {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesEditChatAdmin {
    pub chat_id: i32,
    pub user_id: Box<InputUser>,
    pub is_admin: bool,
}

impl Method for MessagesEditChatAdmin {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesMigrateChat {
    pub chat_id: i32,
}

impl Method for MessagesMigrateChat {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSearchGlobal {
    pub q: String,
    pub offset_date: i32,
    pub offset_peer: Box<InputPeer>,
    pub offset_id: i32,
    pub limit: i32,
}

impl Method for MessagesSearchGlobal {
    type ReturnType = types::MessagesMessages;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesReorderStickerSets {
    pub flags: u32,
    pub masks: bool,
    pub order: Vec<i64>,
}

impl Method for MessagesReorderStickerSets {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetDocumentByHash {
    pub sha256: Vec<u8>,
    pub size: i32,
    pub mime_type: String,
}

impl Method for MessagesGetDocumentByHash {
    type ReturnType = types::Document;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSearchGifs {
    pub q: String,
    pub offset: i32,
}

impl Method for MessagesSearchGifs {
    type ReturnType = types::MessagesFoundGifs;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetSavedGifs {
    pub hash: i32,
}

impl Method for MessagesGetSavedGifs {
    type ReturnType = types::MessagesSavedGifs;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSaveGif {
    pub id: Box<InputDocument>,
    pub unsave: bool,
}

impl Method for MessagesSaveGif {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetInlineBotResults {
    pub flags: u32,
    pub bot: Box<InputUser>,
    pub peer: Box<InputPeer>,
    pub geo_point: Option<Box<InputGeoPoint>>,
    pub query: String,
    pub offset: String,
}

impl Method for MessagesGetInlineBotResults {
    type ReturnType = types::MessagesBotResults;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSetInlineBotResults {
    pub flags: u32,
    pub gallery: bool,
    pub private: bool,
    pub query_id: i64,
    pub results: Vec<InputBotInlineResult>,
    pub cache_time: i32,
    pub next_offset: Option<String>,
    pub switch_pm: Option<Box<InlineBotSwitchPm>>,
}

impl Method for MessagesSetInlineBotResults {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSendInlineBotResult {
    pub flags: u32,
    pub silent: bool,
    pub background: bool,
    pub clear_draft: bool,
    pub peer: Box<InputPeer>,
    pub reply_to_msg_id: Option<i32>,
    pub random_id: i64,
    pub query_id: i64,
    pub id: String,
}

impl Method for MessagesSendInlineBotResult {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetMessageEditData {
    pub peer: Box<InputPeer>,
    pub id: i32,
}

impl Method for MessagesGetMessageEditData {
    type ReturnType = types::MessagesMessageEditData;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesEditMessage {
    pub flags: u32,
    pub no_webpage: bool,
    pub stop_geo_live: bool,
    pub peer: Box<InputPeer>,
    pub id: i32,
    pub message: Option<String>,
    pub reply_markup: Option<Box<ReplyMarkup>>,
    pub entities: Option<Vec<MessageEntity>>,
    pub geo_point: Option<Box<InputGeoPoint>>,
}

impl Method for MessagesEditMessage {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesEditInlineBotMessage {
    pub flags: u32,
    pub no_webpage: bool,
    pub stop_geo_live: bool,
    pub id: Box<InputBotInlineMessageId>,
    pub message: Option<String>,
    pub reply_markup: Option<Box<ReplyMarkup>>,
    pub entities: Option<Vec<MessageEntity>>,
    pub geo_point: Option<Box<InputGeoPoint>>,
}

impl Method for MessagesEditInlineBotMessage {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetBotCallbackAnswer {
    pub flags: u32,
    pub game: bool,
    pub peer: Box<InputPeer>,
    pub msg_id: i32,
    pub data: Option<Vec<u8>>,
}

impl Method for MessagesGetBotCallbackAnswer {
    type ReturnType = types::MessagesBotCallbackAnswer;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSetBotCallbackAnswer {
    pub flags: u32,
    pub alert: bool,
    pub query_id: i64,
    pub message: Option<String>,
    pub url: Option<String>,
    pub cache_time: i32,
}

impl Method for MessagesSetBotCallbackAnswer {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetPeerDialogs {
    pub peers: Vec<InputDialogPeer>,
}

impl Method for MessagesGetPeerDialogs {
    type ReturnType = types::MessagesPeerDialogs;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSaveDraft {
    pub flags: u32,
    pub no_webpage: bool,
    pub reply_to_msg_id: Option<i32>,
    pub peer: Box<InputPeer>,
    pub message: String,
    pub entities: Option<Vec<MessageEntity>>,
}

impl Method for MessagesSaveDraft {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetAllDrafts;

impl Method for MessagesGetAllDrafts {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetFeaturedStickers {
    pub hash: i32,
}

impl Method for MessagesGetFeaturedStickers {
    type ReturnType = types::MessagesFeaturedStickers;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesReadFeaturedStickers {
    pub id: Vec<i64>,
}

impl Method for MessagesReadFeaturedStickers {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetRecentStickers {
    pub flags: u32,
    pub attached: bool,
    pub hash: i32,
}

impl Method for MessagesGetRecentStickers {
    type ReturnType = types::MessagesRecentStickers;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSaveRecentSticker {
    pub flags: u32,
    pub attached: bool,
    pub id: Box<InputDocument>,
    pub unsave: bool,
}

impl Method for MessagesSaveRecentSticker {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesClearRecentStickers {
    pub flags: u32,
    pub attached: bool,
}

impl Method for MessagesClearRecentStickers {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetArchivedStickers {
    pub flags: u32,
    pub masks: bool,
    pub offset_id: i64,
    pub limit: i32,
}

impl Method for MessagesGetArchivedStickers {
    type ReturnType = types::MessagesArchivedStickers;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetMaskStickers {
    pub hash: i32,
}

impl Method for MessagesGetMaskStickers {
    type ReturnType = types::MessagesAllStickers;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetAttachedStickers {
    pub media: Box<InputStickeredMedia>,
}

impl Method for MessagesGetAttachedStickers {
    type ReturnType = StickerSetCovered;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSetGameScore {
    pub flags: u32,
    pub edit_message: bool,
    pub force: bool,
    pub peer: Box<InputPeer>,
    pub id: i32,
    pub user_id: Box<InputUser>,
    pub score: i32,
}

impl Method for MessagesSetGameScore {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSetInlineGameScore {
    pub flags: u32,
    pub edit_message: bool,
    pub force: bool,
    pub id: Box<InputBotInlineMessageId>,
    pub user_id: Box<InputUser>,
    pub score: i32,
}

impl Method for MessagesSetInlineGameScore {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetGameHighScores {
    pub peer: Box<InputPeer>,
    pub id: i32,
    pub user_id: Box<InputUser>,
}

impl Method for MessagesGetGameHighScores {
    type ReturnType = types::MessagesHighScores;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetInlineGameHighScores {
    pub id: Box<InputBotInlineMessageId>,
    pub user_id: Box<InputUser>,
}

impl Method for MessagesGetInlineGameHighScores {
    type ReturnType = types::MessagesHighScores;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetCommonChats {
    pub user_id: Box<InputUser>,
    pub max_id: i32,
    pub limit: i32,
}

impl Method for MessagesGetCommonChats {
    type ReturnType = types::MessagesChats;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetAllChats {
    pub except_ids: Vec<i32>,
}

impl Method for MessagesGetAllChats {
    type ReturnType = types::MessagesChats;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetWebPage {
    pub url: String,
    pub hash: i32,
}

impl Method for MessagesGetWebPage {
    type ReturnType = types::WebPage;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesToggleDialogPin {
    pub flags: u32,
    pub pinned: bool,
    pub peer: Box<InputDialogPeer>,
}

impl Method for MessagesToggleDialogPin {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesReorderPinnedDialogs {
    pub flags: u32,
    pub force: bool,
    pub order: Vec<InputDialogPeer>,
}

impl Method for MessagesReorderPinnedDialogs {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetPinnedDialogs;

impl Method for MessagesGetPinnedDialogs {
    type ReturnType = types::MessagesPeerDialogs;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSetBotShippingResults {
    pub flags: u32,
    pub query_id: i64,
    pub error: Option<String>,
    pub shipping_options: Option<Vec<ShippingOption>>,
}

impl Method for MessagesSetBotShippingResults {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSetBotPrecheckoutResults {
    pub flags: u32,
    pub success: bool,
    pub query_id: i64,
    pub error: Option<String>,
}

impl Method for MessagesSetBotPrecheckoutResults {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesUploadMedia {
    pub peer: Box<InputPeer>,
    pub media: Box<InputMedia>,
}

impl Method for MessagesUploadMedia {
    type ReturnType = types::MessageMedia;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSendScreenshotNotification {
    pub peer: Box<InputPeer>,
    pub reply_to_msg_id: i32,
    pub random_id: i64,
}

impl Method for MessagesSendScreenshotNotification {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetFavedStickers {
    pub hash: i32,
}

impl Method for MessagesGetFavedStickers {
    type ReturnType = types::MessagesFavedStickers;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesFaveSticker {
    pub id: Box<InputDocument>,
    pub unfave: bool,
}

impl Method for MessagesFaveSticker {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetUnreadMentions {
    pub peer: Box<InputPeer>,
    pub offset_id: i32,
    pub add_offset: i32,
    pub limit: i32,
    pub max_id: i32,
    pub min_id: i32,
}

impl Method for MessagesGetUnreadMentions {
    type ReturnType = types::MessagesMessages;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesReadMentions {
    pub peer: Box<InputPeer>,
}

impl Method for MessagesReadMentions {
    type ReturnType = types::MessagesAffectedHistory;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesGetRecentLocations {
    pub peer: Box<InputPeer>,
    pub limit: i32,
}

impl Method for MessagesGetRecentLocations {
    type ReturnType = types::MessagesMessages;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesSendMultiMedia {
    pub flags: u32,
    pub silent: bool,
    pub background: bool,
    pub clear_draft: bool,
    pub peer: Box<InputPeer>,
    pub reply_to_msg_id: Option<i32>,
    pub multi_media: Vec<InputSingleMedia>,
}

impl Method for MessagesSendMultiMedia {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct MessagesUploadEncryptedFile {
    pub peer: Box<InputEncryptedChat>,
    pub file: Box<InputEncryptedFile>,
}

impl Method for MessagesUploadEncryptedFile {
    type ReturnType = types::EncryptedFile;
}


#[derive(Debug, Clone, PartialEq)]
pub struct UpdatesGetState;

impl Method for UpdatesGetState {
    type ReturnType = types::UpdatesState;
}


#[derive(Debug, Clone, PartialEq)]
pub struct UpdatesGetDifference {
    pub flags: u32,
    pub pts: i32,
    pub pts_total_limit: Option<i32>,
    pub date: i32,
    pub qts: i32,
}

impl Method for UpdatesGetDifference {
    type ReturnType = types::UpdatesDifference;
}


#[derive(Debug, Clone, PartialEq)]
pub struct UpdatesGetChannelDifference {
    pub flags: u32,
    pub force: bool,
    pub channel: Box<InputChannel>,
    pub filter: Box<ChannelMessagesFilter>,
    pub pts: i32,
    pub limit: i32,
}

impl Method for UpdatesGetChannelDifference {
    type ReturnType = types::UpdatesChannelDifference;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PhotosUpdateProfilePhoto {
    pub id: Box<InputPhoto>,
}

impl Method for PhotosUpdateProfilePhoto {
    type ReturnType = types::UserProfilePhoto;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PhotosUploadProfilePhoto {
    pub file: Box<InputFile>,
}

impl Method for PhotosUploadProfilePhoto {
    type ReturnType = types::PhotosPhoto;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PhotosDeletePhotos {
    pub id: Vec<InputPhoto>,
}

impl Method for PhotosDeletePhotos {
    type ReturnType = i64;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PhotosGetUserPhotos {
    pub user_id: Box<InputUser>,
    pub offset: i32,
    pub max_id: i64,
    pub limit: i32,
}

impl Method for PhotosGetUserPhotos {
    type ReturnType = types::PhotosPhotos;
}


#[derive(Debug, Clone, PartialEq)]
pub struct UploadSaveFilePart {
    pub file_id: i64,
    pub file_part: i32,
    pub bytes: Vec<u8>,
}

impl Method for UploadSaveFilePart {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct UploadGetFile {
    pub location: Box<InputFileLocation>,
    pub offset: i32,
    pub limit: i32,
}

impl Method for UploadGetFile {
    type ReturnType = types::UploadFile;
}


#[derive(Debug, Clone, PartialEq)]
pub struct UploadSaveBigFilePart {
    pub file_id: i64,
    pub file_part: i32,
    pub file_total_parts: i32,
    pub bytes: Vec<u8>,
}

impl Method for UploadSaveBigFilePart {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct UploadGetWebFile {
    pub location: Box<InputWebFileLocation>,
    pub offset: i32,
    pub limit: i32,
}

impl Method for UploadGetWebFile {
    type ReturnType = types::UploadWebFile;
}


#[derive(Debug, Clone, PartialEq)]
pub struct UploadGetCdnFile {
    pub file_token: Vec<u8>,
    pub offset: i32,
    pub limit: i32,
}

impl Method for UploadGetCdnFile {
    type ReturnType = types::UploadCdnFile;
}


#[derive(Debug, Clone, PartialEq)]
pub struct UploadReuploadCdnFile {
    pub file_token: Vec<u8>,
    pub request_token: Vec<u8>,
}

impl Method for UploadReuploadCdnFile {
    type ReturnType = CdnFileHash;
}


#[derive(Debug, Clone, PartialEq)]
pub struct UploadGetCdnFileHashes {
    pub file_token: Vec<u8>,
    pub offset: i32,
}

impl Method for UploadGetCdnFileHashes {
    type ReturnType = CdnFileHash;
}


#[derive(Debug, Clone, PartialEq)]
pub struct HelpGetConfig;

impl Method for HelpGetConfig {
    type ReturnType = types::Config;
}


#[derive(Debug, Clone, PartialEq)]
pub struct HelpGetNearestDc;

impl Method for HelpGetNearestDc {
    type ReturnType = types::NearestDc;
}


#[derive(Debug, Clone, PartialEq)]
pub struct HelpGetAppUpdate;

impl Method for HelpGetAppUpdate {
    type ReturnType = types::HelpAppUpdate;
}


#[derive(Debug, Clone, PartialEq)]
pub struct HelpSaveAppLog {
    pub events: Vec<InputAppEvent>,
}

impl Method for HelpSaveAppLog {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct HelpGetInviteText;

impl Method for HelpGetInviteText {
    type ReturnType = types::HelpInviteText;
}


#[derive(Debug, Clone, PartialEq)]
pub struct HelpGetSupport;

impl Method for HelpGetSupport {
    type ReturnType = types::HelpSupport;
}


#[derive(Debug, Clone, PartialEq)]
pub struct HelpGetAppChangelog {
    pub prev_app_version: String,
}

impl Method for HelpGetAppChangelog {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct HelpGetTermsOfService;

impl Method for HelpGetTermsOfService {
    type ReturnType = types::HelpTermsOfService;
}


#[derive(Debug, Clone, PartialEq)]
pub struct HelpSetBotUpdatesStatus {
    pub pending_updates_count: i32,
    pub message: String,
}

impl Method for HelpSetBotUpdatesStatus {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct HelpGetCdnConfig;

impl Method for HelpGetCdnConfig {
    type ReturnType = types::CdnConfig;
}


#[derive(Debug, Clone, PartialEq)]
pub struct HelpGetRecentMeUrls {
    pub referer: String,
}

impl Method for HelpGetRecentMeUrls {
    type ReturnType = types::HelpRecentMeUrls;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsReadHistory {
    pub channel: Box<InputChannel>,
    pub max_id: i32,
}

impl Method for ChannelsReadHistory {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsDeleteMessages {
    pub channel: Box<InputChannel>,
    pub id: Vec<i32>,
}

impl Method for ChannelsDeleteMessages {
    type ReturnType = types::MessagesAffectedMessages;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsDeleteUserHistory {
    pub channel: Box<InputChannel>,
    pub user_id: Box<InputUser>,
}

impl Method for ChannelsDeleteUserHistory {
    type ReturnType = types::MessagesAffectedHistory;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsReportSpam {
    pub channel: Box<InputChannel>,
    pub user_id: Box<InputUser>,
    pub id: Vec<i32>,
}

impl Method for ChannelsReportSpam {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsGetMessages {
    pub channel: Box<InputChannel>,
    pub id: Vec<i32>,
}

impl Method for ChannelsGetMessages {
    type ReturnType = types::MessagesMessages;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsGetParticipants {
    pub channel: Box<InputChannel>,
    pub filter: Box<ChannelParticipantsFilter>,
    pub offset: i32,
    pub limit: i32,
    pub hash: i32,
}

impl Method for ChannelsGetParticipants {
    type ReturnType = types::ChannelsChannelParticipants;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsGetParticipant {
    pub channel: Box<InputChannel>,
    pub user_id: Box<InputUser>,
}

impl Method for ChannelsGetParticipant {
    type ReturnType = types::ChannelsChannelParticipant;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsGetChannels {
    pub id: Vec<InputChannel>,
}

impl Method for ChannelsGetChannels {
    type ReturnType = types::MessagesChats;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsGetFullChannel {
    pub channel: Box<InputChannel>,
}

impl Method for ChannelsGetFullChannel {
    type ReturnType = types::MessagesChatFull;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsCreateChannel {
    pub flags: u32,
    pub broadcast: bool,
    pub megagroup: bool,
    pub title: String,
    pub about: String,
}

impl Method for ChannelsCreateChannel {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsEditAbout {
    pub channel: Box<InputChannel>,
    pub about: String,
}

impl Method for ChannelsEditAbout {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsEditAdmin {
    pub channel: Box<InputChannel>,
    pub user_id: Box<InputUser>,
    pub admin_rights: Box<ChannelAdminRights>,
}

impl Method for ChannelsEditAdmin {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsEditTitle {
    pub channel: Box<InputChannel>,
    pub title: String,
}

impl Method for ChannelsEditTitle {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsEditPhoto {
    pub channel: Box<InputChannel>,
    pub photo: Box<InputChatPhoto>,
}

impl Method for ChannelsEditPhoto {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsCheckUsername {
    pub channel: Box<InputChannel>,
    pub username: String,
}

impl Method for ChannelsCheckUsername {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsUpdateUsername {
    pub channel: Box<InputChannel>,
    pub username: String,
}

impl Method for ChannelsUpdateUsername {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsJoinChannel {
    pub channel: Box<InputChannel>,
}

impl Method for ChannelsJoinChannel {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsLeaveChannel {
    pub channel: Box<InputChannel>,
}

impl Method for ChannelsLeaveChannel {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsInviteToChannel {
    pub channel: Box<InputChannel>,
    pub users: Vec<InputUser>,
}

impl Method for ChannelsInviteToChannel {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsExportInvite {
    pub channel: Box<InputChannel>,
}

impl Method for ChannelsExportInvite {
    type ReturnType = types::ExportedChatInvite;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsDeleteChannel {
    pub channel: Box<InputChannel>,
}

impl Method for ChannelsDeleteChannel {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsToggleInvites {
    pub channel: Box<InputChannel>,
    pub enabled: bool,
}

impl Method for ChannelsToggleInvites {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsExportMessageLink {
    pub channel: Box<InputChannel>,
    pub id: i32,
    pub grouped: bool,
}

impl Method for ChannelsExportMessageLink {
    type ReturnType = types::ExportedMessageLink;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsToggleSignatures {
    pub channel: Box<InputChannel>,
    pub enabled: bool,
}

impl Method for ChannelsToggleSignatures {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsUpdatePinnedMessage {
    pub flags: u32,
    pub silent: bool,
    pub channel: Box<InputChannel>,
    pub id: i32,
}

impl Method for ChannelsUpdatePinnedMessage {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsGetAdminedPublicChannels;

impl Method for ChannelsGetAdminedPublicChannels {
    type ReturnType = types::MessagesChats;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsEditBanned {
    pub channel: Box<InputChannel>,
    pub user_id: Box<InputUser>,
    pub banned_rights: Box<ChannelBannedRights>,
}

impl Method for ChannelsEditBanned {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsGetAdminLog {
    pub flags: u32,
    pub channel: Box<InputChannel>,
    pub q: String,
    pub events_filter: Option<Box<ChannelAdminLogEventsFilter>>,
    pub admins: Option<Vec<InputUser>>,
    pub max_id: i64,
    pub min_id: i64,
    pub limit: i32,
}

impl Method for ChannelsGetAdminLog {
    type ReturnType = types::ChannelsAdminLogResults;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsSetStickers {
    pub channel: Box<InputChannel>,
    pub stickerset: Box<InputStickerSet>,
}

impl Method for ChannelsSetStickers {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsReadMessageContents {
    pub channel: Box<InputChannel>,
    pub id: Vec<i32>,
}

impl Method for ChannelsReadMessageContents {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsDeleteHistory {
    pub channel: Box<InputChannel>,
    pub max_id: i32,
}

impl Method for ChannelsDeleteHistory {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsTogglePreHistoryHidden {
    pub channel: Box<InputChannel>,
    pub enabled: bool,
}

impl Method for ChannelsTogglePreHistoryHidden {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsGetFeed {
    pub flags: u32,
    pub offset_to_max_read: bool,
    pub feed_id: i32,
    pub offset_position: Option<Box<FeedPosition>>,
    pub add_offset: i32,
    pub limit: i32,
    pub max_position: Option<Box<FeedPosition>>,
    pub min_position: Option<Box<FeedPosition>>,
    pub sources_hash: i32,
    pub hash: i32,
}

impl Method for ChannelsGetFeed {
    type ReturnType = types::MessagesFeedMessages;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsSearchFeed {
    pub feed_id: i32,
    pub q: String,
    pub offset_date: i32,
    pub offset_peer: Box<InputPeer>,
    pub offset_id: i32,
    pub limit: i32,
}

impl Method for ChannelsSearchFeed {
    type ReturnType = types::MessagesMessages;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsGetFeedSources {
    pub flags: u32,
    pub feed_id: Option<i32>,
    pub hash: i32,
}

impl Method for ChannelsGetFeedSources {
    type ReturnType = types::ChannelsFeedSources;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsChangeFeedBroadcast {
    pub flags: u32,
    pub channel: Box<InputChannel>,
    pub feed_id: Option<i32>,
}

impl Method for ChannelsChangeFeedBroadcast {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsSetFeedBroadcasts {
    pub feed_id: i32,
    pub channels: Vec<InputChannel>,
    pub also_newly_joined: bool,
}

impl Method for ChannelsSetFeedBroadcasts {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct ChannelsReadFeed {
    pub feed_id: i32,
    pub max_position: Box<FeedPosition>,
}

impl Method for ChannelsReadFeed {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct BotsSendCustomRequest {
    pub custom_method: String,
    pub params: Box<DataJson>,
}

impl Method for BotsSendCustomRequest {
    type ReturnType = types::DataJson;
}


#[derive(Debug, Clone, PartialEq)]
pub struct BotsAnswerWebhookJsonquery {
    pub query_id: i64,
    pub data: Box<DataJson>,
}

impl Method for BotsAnswerWebhookJsonquery {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PaymentsGetPaymentForm {
    pub msg_id: i32,
}

impl Method for PaymentsGetPaymentForm {
    type ReturnType = types::PaymentsPaymentForm;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PaymentsGetPaymentReceipt {
    pub msg_id: i32,
}

impl Method for PaymentsGetPaymentReceipt {
    type ReturnType = types::PaymentsPaymentReceipt;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PaymentsValidateRequestedInfo {
    pub flags: u32,
    pub save: bool,
    pub msg_id: i32,
    pub info: Box<PaymentRequestedInfo>,
}

impl Method for PaymentsValidateRequestedInfo {
    type ReturnType = types::PaymentsValidatedRequestedInfo;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PaymentsSendPaymentForm {
    pub flags: u32,
    pub msg_id: i32,
    pub requested_info_id: Option<String>,
    pub shipping_option_id: Option<String>,
    pub credentials: Box<InputPaymentCredentials>,
}

impl Method for PaymentsSendPaymentForm {
    type ReturnType = types::PaymentsPaymentResult;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PaymentsGetSavedInfo;

impl Method for PaymentsGetSavedInfo {
    type ReturnType = types::PaymentsSavedInfo;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PaymentsClearSavedInfo {
    pub flags: u32,
    pub credentials: bool,
    pub info: bool,
}

impl Method for PaymentsClearSavedInfo {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct StickersCreateStickerSet {
    pub flags: u32,
    pub masks: bool,
    pub user_id: Box<InputUser>,
    pub title: String,
    pub short_name: String,
    pub stickers: Vec<InputStickerSetItem>,
}

impl Method for StickersCreateStickerSet {
    type ReturnType = types::MessagesStickerSet;
}


#[derive(Debug, Clone, PartialEq)]
pub struct StickersRemoveStickerFromSet {
    pub sticker: Box<InputDocument>,
}

impl Method for StickersRemoveStickerFromSet {
    type ReturnType = types::MessagesStickerSet;
}


#[derive(Debug, Clone, PartialEq)]
pub struct StickersChangeStickerPosition {
    pub sticker: Box<InputDocument>,
    pub position: i32,
}

impl Method for StickersChangeStickerPosition {
    type ReturnType = types::MessagesStickerSet;
}


#[derive(Debug, Clone, PartialEq)]
pub struct StickersAddStickerToSet {
    pub stickerset: Box<InputStickerSet>,
    pub sticker: Box<InputStickerSetItem>,
}

impl Method for StickersAddStickerToSet {
    type ReturnType = types::MessagesStickerSet;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PhoneGetCallConfig;

impl Method for PhoneGetCallConfig {
    type ReturnType = types::DataJson;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PhoneRequestCall {
    pub user_id: Box<InputUser>,
    pub random_id: i32,
    pub g_a_hash: Vec<u8>,
    pub protocol: Box<PhoneCallProtocol>,
}

impl Method for PhoneRequestCall {
    type ReturnType = types::PhonePhoneCall;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PhoneAcceptCall {
    pub peer: Box<InputPhoneCall>,
    pub g_b: Vec<u8>,
    pub protocol: Box<PhoneCallProtocol>,
}

impl Method for PhoneAcceptCall {
    type ReturnType = types::PhonePhoneCall;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PhoneConfirmCall {
    pub peer: Box<InputPhoneCall>,
    pub g_a: Vec<u8>,
    pub key_fingerprint: i64,
    pub protocol: Box<PhoneCallProtocol>,
}

impl Method for PhoneConfirmCall {
    type ReturnType = types::PhonePhoneCall;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PhoneReceivedCall {
    pub peer: Box<InputPhoneCall>,
}

impl Method for PhoneReceivedCall {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PhoneDiscardCall {
    pub peer: Box<InputPhoneCall>,
    pub duration: i32,
    pub reason: Box<PhoneCallDiscardReason>,
    pub connection_id: i64,
}

impl Method for PhoneDiscardCall {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PhoneSetCallRating {
    pub peer: Box<InputPhoneCall>,
    pub rating: i32,
    pub comment: String,
}

impl Method for PhoneSetCallRating {
    type ReturnType = types::Updates;
}


#[derive(Debug, Clone, PartialEq)]
pub struct PhoneSaveCallDebug {
    pub peer: Box<InputPhoneCall>,
    pub debug: Box<DataJson>,
}

impl Method for PhoneSaveCallDebug {
    type ReturnType = bool;
}


#[derive(Debug, Clone, PartialEq)]
pub struct LangpackGetLangPack {
    pub lang_code: String,
}

impl Method for LangpackGetLangPack {
    type ReturnType = types::LangPackDifference;
}


#[derive(Debug, Clone, PartialEq)]
pub struct LangpackGetStrings {
    pub lang_code: String,
    pub keys: Vec<String>,
}

impl Method for LangpackGetStrings {
    type ReturnType = LangPackString;
}


#[derive(Debug, Clone, PartialEq)]
pub struct LangpackGetDifference {
    pub from_version: i32,
}

impl Method for LangpackGetDifference {
    type ReturnType = types::LangPackDifference;
}


#[derive(Debug, Clone, PartialEq)]
pub struct LangpackGetLanguages;

impl Method for LangpackGetLanguages {
    type ReturnType = LangPackLanguage;
}


