impl Serializable for MessageAction {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessageAction::ChatCreate { .. } => buf.serialize(&0xa6638b9a_u32)?,
             MessageAction::ChatDeleteUser { .. } => buf.serialize(&0xb2ae9b0c_u32)?,
             MessageAction::CustomAction { .. } => buf.serialize(&0xfae69f56_u32)?,
             MessageAction::ChatMigrateTo { .. } => buf.serialize(&0x51bdb021_u32)?,
             MessageAction::PaymentSentMe { .. } => buf.serialize(&0x8f31b327_u32)?,
             MessageAction::PhoneCall { .. } => buf.serialize(&0x80e11a7f_u32)?,
             MessageAction::Empty { .. } => buf.serialize(&0xb6aef7b0_u32)?,
             MessageAction::ScreenshotTaken { .. } => buf.serialize(&0x4792929b_u32)?,
             MessageAction::ChatJoinedByLink { .. } => buf.serialize(&0xf89cf5e8_u32)?,
             MessageAction::ChatAddUser { .. } => buf.serialize(&0x488a7337_u32)?,
             MessageAction::GameScore { .. } => buf.serialize(&0x92a72876_u32)?,
             MessageAction::PaymentSent { .. } => buf.serialize(&0x40699cd0_u32)?,
             MessageAction::HistoryClear { .. } => buf.serialize(&0x9fbab604_u32)?,
             MessageAction::ChatEditTitle { .. } => buf.serialize(&0xb5a1ce5a_u32)?,
             MessageAction::ChannelMigrateFrom { .. } => buf.serialize(&0xb055eaee_u32)?,
             MessageAction::ChatDeletePhoto { .. } => buf.serialize(&0x95e3fbef_u32)?,
             MessageAction::ChatEditPhoto { .. } => buf.serialize(&0x7fcb13a8_u32)?,
             MessageAction::PinMessage { .. } => buf.serialize(&0x94bd38ed_u32)?,
             MessageAction::ChannelCreate { .. } => buf.serialize(&0x95d2ac92_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessageAction::ChatCreate {
                ref title,
                ref users,
            } => {
                buf.serialize(title)?;
                buf.serialize(users)?;
            },

            MessageAction::ChatDeleteUser {
                ref user_id,
            } => {
                buf.serialize(user_id)?;
            },

            MessageAction::CustomAction {
                ref message,
            } => {
                buf.serialize(message)?;
            },

            MessageAction::ChatMigrateTo {
                ref channel_id,
            } => {
                buf.serialize(channel_id)?;
            },

            MessageAction::PaymentSentMe {
                flags: _,
                ref currency,
                ref total_amount,
                ref payload,
                ref info,
                ref shipping_option_id,
                ref charge,
            } => {
                let mut ser_flags: u32 = 0;

                if info.is_some() {
                    ser_flags |= 1 << 0;
                }

                if shipping_option_id.is_some() {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(currency)?;
                buf.serialize(total_amount)?;
                buf.serialize(payload)?;

                if let Some(ref value) = info {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = shipping_option_id {
                    buf.serialize(value)?;
                }

                buf.serialize(charge.as_ref())?;
            },

            MessageAction::PhoneCall {
                flags: _,
                ref call_id,
                ref reason,
                ref duration,
            } => {
                let mut ser_flags: u32 = 0;

                if reason.is_some() {
                    ser_flags |= 1 << 0;
                }

                if duration.is_some() {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(call_id)?;

                if let Some(ref value) = reason {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = duration {
                    buf.serialize(value)?;
                }

            },

            MessageAction::Empty {
            } => {
            },

            MessageAction::ScreenshotTaken {
            } => {
            },

            MessageAction::ChatJoinedByLink {
                ref inviter_id,
            } => {
                buf.serialize(inviter_id)?;
            },

            MessageAction::ChatAddUser {
                ref users,
            } => {
                buf.serialize(users)?;
            },

            MessageAction::GameScore {
                ref game_id,
                ref score,
            } => {
                buf.serialize(game_id)?;
                buf.serialize(score)?;
            },

            MessageAction::PaymentSent {
                ref currency,
                ref total_amount,
            } => {
                buf.serialize(currency)?;
                buf.serialize(total_amount)?;
            },

            MessageAction::HistoryClear {
            } => {
            },

            MessageAction::ChatEditTitle {
                ref title,
            } => {
                buf.serialize(title)?;
            },

            MessageAction::ChannelMigrateFrom {
                ref title,
                ref chat_id,
            } => {
                buf.serialize(title)?;
                buf.serialize(chat_id)?;
            },

            MessageAction::ChatDeletePhoto {
            } => {
            },

            MessageAction::ChatEditPhoto {
                ref photo,
            } => {
                buf.serialize(photo.as_ref())?;
            },

            MessageAction::PinMessage {
            } => {
            },

            MessageAction::ChannelCreate {
                ref title,
            } => {
                buf.serialize(title)?;
            },
        }

        Ok(())
    }
}

impl Serializable for TopPeerCategory {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             TopPeerCategory::Correspondents { .. } => buf.serialize(&0x0637b7ed_u32)?,
             TopPeerCategory::Channels { .. } => buf.serialize(&0x161d9628_u32)?,
             TopPeerCategory::BotsPm { .. } => buf.serialize(&0xab661b5b_u32)?,
             TopPeerCategory::BotsInline { .. } => buf.serialize(&0x148677e2_u32)?,
             TopPeerCategory::Groups { .. } => buf.serialize(&0xbd17a14a_u32)?,
             TopPeerCategory::PhoneCalls { .. } => buf.serialize(&0x1e76a78c_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            TopPeerCategory::Correspondents {
            } => {
            },

            TopPeerCategory::Channels {
            } => {
            },

            TopPeerCategory::BotsPm {
            } => {
            },

            TopPeerCategory::BotsInline {
            } => {
            },

            TopPeerCategory::Groups {
            } => {
            },

            TopPeerCategory::PhoneCalls {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for ContactsResolvedPeer {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x7f077ad9_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.chats)?;
        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for StickerPack {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x12b299d4_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.emoticon)?;
        buf.serialize(&self.documents)?;

        Ok(())
    }
 }

impl Serializable for Photo {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             Photo::Empty { .. } => buf.serialize(&0x2331b22d_u32)?,
             Photo::Photo { .. } => buf.serialize(&0x9288dd29_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            Photo::Empty {
                ref id,
            } => {
                buf.serialize(id)?;
            },

            Photo::Photo {
                flags: _,
                ref has_stickers,
                ref id,
                ref access_hash,
                ref date,
                ref sizes,
            } => {
                let mut ser_flags: u32 = 0;

                if has_stickers {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
                buf.serialize(date)?;
                buf.serialize(sizes)?;
            },
        }

        Ok(())
    }
}

impl Serializable for ChannelAdminLogEvent {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3b5a3e40_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;
        buf.serialize(&self.date)?;
        buf.serialize(&self.user_id)?;
        buf.serialize(self.action.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesAffectedMessages {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x84d19185_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.pts)?;
        buf.serialize(&self.pts_count)?;

        Ok(())
    }
 }

impl Serializable for PhotosPhoto {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x20212ca8_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.photo.as_ref())?;
        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for HelpSupport {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x17c6b5f6_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.phone_number)?;
        buf.serialize(self.user.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MsgsStateReq {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xda69fb52_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.msg_ids)?;

        Ok(())
    }
 }

impl Serializable for PeerNotifySettings {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             PeerNotifySettings::Empty { .. } => buf.serialize(&0x70a68512_u32)?,
             PeerNotifySettings::Settings { .. } => buf.serialize(&0x9acda4c0_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            PeerNotifySettings::Empty {
            } => {
            },

            PeerNotifySettings::Settings {
                flags: _,
                ref show_previews,
                ref silent,
                ref mute_until,
                ref sound,
            } => {
                let mut ser_flags: u32 = 0;

                if show_previews {
                    ser_flags |= 1 << 0;
                }

                if silent {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(mute_until)?;
                buf.serialize(sound)?;
            },
        }

        Ok(())
    }
}

impl Serializable for InputBotInlineMessage {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputBotInlineMessage::MediaGeo { .. } => buf.serialize(&0xc1b15d65_u32)?,
             InputBotInlineMessage::MediaVenue { .. } => buf.serialize(&0xaaafadc8_u32)?,
             InputBotInlineMessage::Text { .. } => buf.serialize(&0x3dcd7a87_u32)?,
             InputBotInlineMessage::MediaAuto { .. } => buf.serialize(&0x292fed13_u32)?,
             InputBotInlineMessage::MediaContact { .. } => buf.serialize(&0x2daf01a7_u32)?,
             InputBotInlineMessage::Game { .. } => buf.serialize(&0x4b425864_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputBotInlineMessage::MediaGeo {
                flags: _,
                ref geo_point,
                ref period,
                ref reply_markup,
            } => {
                let mut ser_flags: u32 = 0;

                if reply_markup.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(geo_point.as_ref())?;
                buf.serialize(period)?;

                if let Some(ref value) = reply_markup {
                    buf.serialize(value.as_ref())?;
                }

            },

            InputBotInlineMessage::MediaVenue {
                flags: _,
                ref geo_point,
                ref title,
                ref address,
                ref provider,
                ref venue_id,
                ref reply_markup,
            } => {
                let mut ser_flags: u32 = 0;

                if reply_markup.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(geo_point.as_ref())?;
                buf.serialize(title)?;
                buf.serialize(address)?;
                buf.serialize(provider)?;
                buf.serialize(venue_id)?;

                if let Some(ref value) = reply_markup {
                    buf.serialize(value.as_ref())?;
                }

            },

            InputBotInlineMessage::Text {
                flags: _,
                ref no_webpage,
                ref message,
                ref entities,
                ref reply_markup,
            } => {
                let mut ser_flags: u32 = 0;

                if no_webpage {
                    ser_flags |= 1 << 0;
                }

                if entities.is_some() {
                    ser_flags |= 1 << 1;
                }

                if reply_markup.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(message)?;

                if let Some(ref value) = entities {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = reply_markup {
                    buf.serialize(value.as_ref())?;
                }

            },

            InputBotInlineMessage::MediaAuto {
                flags: _,
                ref caption,
                ref reply_markup,
            } => {
                let mut ser_flags: u32 = 0;

                if reply_markup.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(caption)?;

                if let Some(ref value) = reply_markup {
                    buf.serialize(value.as_ref())?;
                }

            },

            InputBotInlineMessage::MediaContact {
                flags: _,
                ref phone_number,
                ref first_name,
                ref last_name,
                ref reply_markup,
            } => {
                let mut ser_flags: u32 = 0;

                if reply_markup.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(phone_number)?;
                buf.serialize(first_name)?;
                buf.serialize(last_name)?;

                if let Some(ref value) = reply_markup {
                    buf.serialize(value.as_ref())?;
                }

            },

            InputBotInlineMessage::Game {
                flags: _,
                ref reply_markup,
            } => {
                let mut ser_flags: u32 = 0;

                if reply_markup.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;

                if let Some(ref value) = reply_markup {
                    buf.serialize(value.as_ref())?;
                }

            },
        }

        Ok(())
    }
}

impl Serializable for InputPhoneCall {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x1e36fded_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;
        buf.serialize(&self.access_hash)?;

        Ok(())
    }
 }

impl Serializable for MessagesFavedStickers {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessagesFavedStickers::NotModified { .. } => buf.serialize(&0x9e8fa6d3_u32)?,
             MessagesFavedStickers::Stickers { .. } => buf.serialize(&0xf37f2f16_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessagesFavedStickers::NotModified {
            } => {
            },

            MessagesFavedStickers::Stickers {
                ref hash,
                ref packs,
                ref stickers,
            } => {
                buf.serialize(hash)?;
                buf.serialize(packs)?;
                buf.serialize(stickers)?;
            },
        }

        Ok(())
    }
}

impl Serializable for InputAppEvent {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x770656a8_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.time)?;
        buf.serialize(&self.type_)?;
        buf.serialize(&self.peer)?;
        buf.serialize(&self.data)?;

        Ok(())
    }
 }

impl Serializable for ChannelAdminLogEventAction {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ChannelAdminLogEventAction::ChangeStickerSet { .. } => buf.serialize(&0xb1c3caa7_u32)?,
             ChannelAdminLogEventAction::TogglePreHistoryHidden { .. } => buf.serialize(&0x5f5c95f1_u32)?,
             ChannelAdminLogEventAction::ParticipantLeave { .. } => buf.serialize(&0xf89777f2_u32)?,
             ChannelAdminLogEventAction::ChangePhoto { .. } => buf.serialize(&0xb82f55c3_u32)?,
             ChannelAdminLogEventAction::ToggleSignatures { .. } => buf.serialize(&0x26ae0971_u32)?,
             ChannelAdminLogEventAction::ParticipantInvite { .. } => buf.serialize(&0xe31c34d8_u32)?,
             ChannelAdminLogEventAction::ChangeAbout { .. } => buf.serialize(&0x55188a2e_u32)?,
             ChannelAdminLogEventAction::EditMessage { .. } => buf.serialize(&0x709b2405_u32)?,
             ChannelAdminLogEventAction::ChangeTitle { .. } => buf.serialize(&0xe6dfb825_u32)?,
             ChannelAdminLogEventAction::ChangeUsername { .. } => buf.serialize(&0x6a4afc38_u32)?,
             ChannelAdminLogEventAction::ParticipantToggleAdmin { .. } => buf.serialize(&0xd5676710_u32)?,
             ChannelAdminLogEventAction::ParticipantToggleBan { .. } => buf.serialize(&0xe6d83d7e_u32)?,
             ChannelAdminLogEventAction::ParticipantJoin { .. } => buf.serialize(&0x183040d3_u32)?,
             ChannelAdminLogEventAction::DeleteMessage { .. } => buf.serialize(&0x42e047bb_u32)?,
             ChannelAdminLogEventAction::UpdatePinned { .. } => buf.serialize(&0xe9e82c18_u32)?,
             ChannelAdminLogEventAction::ToggleInvites { .. } => buf.serialize(&0x1b7907ae_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ChannelAdminLogEventAction::ChangeStickerSet {
                ref prev_stickerset,
                ref new_stickerset,
            } => {
                buf.serialize(prev_stickerset.as_ref())?;
                buf.serialize(new_stickerset.as_ref())?;
            },

            ChannelAdminLogEventAction::TogglePreHistoryHidden {
                ref new_value,
            } => {
                buf.serialize(new_value)?;
            },

            ChannelAdminLogEventAction::ParticipantLeave {
            } => {
            },

            ChannelAdminLogEventAction::ChangePhoto {
                ref prev_photo,
                ref new_photo,
            } => {
                buf.serialize(prev_photo.as_ref())?;
                buf.serialize(new_photo.as_ref())?;
            },

            ChannelAdminLogEventAction::ToggleSignatures {
                ref new_value,
            } => {
                buf.serialize(new_value)?;
            },

            ChannelAdminLogEventAction::ParticipantInvite {
                ref participant,
            } => {
                buf.serialize(participant.as_ref())?;
            },

            ChannelAdminLogEventAction::ChangeAbout {
                ref prev_value,
                ref new_value,
            } => {
                buf.serialize(prev_value)?;
                buf.serialize(new_value)?;
            },

            ChannelAdminLogEventAction::EditMessage {
                ref prev_message,
                ref new_message,
            } => {
                buf.serialize(prev_message.as_ref())?;
                buf.serialize(new_message.as_ref())?;
            },

            ChannelAdminLogEventAction::ChangeTitle {
                ref prev_value,
                ref new_value,
            } => {
                buf.serialize(prev_value)?;
                buf.serialize(new_value)?;
            },

            ChannelAdminLogEventAction::ChangeUsername {
                ref prev_value,
                ref new_value,
            } => {
                buf.serialize(prev_value)?;
                buf.serialize(new_value)?;
            },

            ChannelAdminLogEventAction::ParticipantToggleAdmin {
                ref prev_participant,
                ref new_participant,
            } => {
                buf.serialize(prev_participant.as_ref())?;
                buf.serialize(new_participant.as_ref())?;
            },

            ChannelAdminLogEventAction::ParticipantToggleBan {
                ref prev_participant,
                ref new_participant,
            } => {
                buf.serialize(prev_participant.as_ref())?;
                buf.serialize(new_participant.as_ref())?;
            },

            ChannelAdminLogEventAction::ParticipantJoin {
            } => {
            },

            ChannelAdminLogEventAction::DeleteMessage {
                ref message,
            } => {
                buf.serialize(message.as_ref())?;
            },

            ChannelAdminLogEventAction::UpdatePinned {
                ref message,
            } => {
                buf.serialize(message.as_ref())?;
            },

            ChannelAdminLogEventAction::ToggleInvites {
                ref new_value,
            } => {
                buf.serialize(new_value)?;
            },
        }

        Ok(())
    }
}

impl Serializable for ChannelParticipant {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ChannelParticipant::Creator { .. } => buf.serialize(&0xe3e2e1f9_u32)?,
             ChannelParticipant::Participant { .. } => buf.serialize(&0x15ebac1d_u32)?,
             ChannelParticipant::Self_ { .. } => buf.serialize(&0xa3289a6d_u32)?,
             ChannelParticipant::Admin { .. } => buf.serialize(&0xa82fa898_u32)?,
             ChannelParticipant::Banned { .. } => buf.serialize(&0x222c1886_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ChannelParticipant::Creator {
                ref user_id,
            } => {
                buf.serialize(user_id)?;
            },

            ChannelParticipant::Participant {
                ref user_id,
                ref date,
            } => {
                buf.serialize(user_id)?;
                buf.serialize(date)?;
            },

            ChannelParticipant::Self_ {
                ref user_id,
                ref inviter_id,
                ref date,
            } => {
                buf.serialize(user_id)?;
                buf.serialize(inviter_id)?;
                buf.serialize(date)?;
            },

            ChannelParticipant::Admin {
                flags: _,
                ref can_edit,
                ref user_id,
                ref inviter_id,
                ref promoted_by,
                ref date,
                ref admin_rights,
            } => {
                let mut ser_flags: u32 = 0;

                if can_edit {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(user_id)?;
                buf.serialize(inviter_id)?;
                buf.serialize(promoted_by)?;
                buf.serialize(date)?;
                buf.serialize(admin_rights.as_ref())?;
            },

            ChannelParticipant::Banned {
                flags: _,
                ref left,
                ref user_id,
                ref kicked_by,
                ref date,
                ref banned_rights,
            } => {
                let mut ser_flags: u32 = 0;

                if left {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(user_id)?;
                buf.serialize(kicked_by)?;
                buf.serialize(date)?;
                buf.serialize(banned_rights.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for ContactStatus {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xd3680c61_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.user_id)?;
        buf.serialize(self.status.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ServerDhParams {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ServerDhParams::Ok { .. } => buf.serialize(&0xd0e8075c_u32)?,
             ServerDhParams::Fail { .. } => buf.serialize(&0x79cb045d_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ServerDhParams::Ok {
                ref nonce,
                ref server_nonce,
                ref encrypted_answer,
            } => {
                buf.serialize(nonce)?;
                buf.serialize(server_nonce)?;
                buf.serialize(encrypted_answer)?;
            },

            ServerDhParams::Fail {
                ref nonce,
                ref server_nonce,
                ref new_nonce_hash,
            } => {
                buf.serialize(nonce)?;
                buf.serialize(server_nonce)?;
                buf.serialize(new_nonce_hash)?;
            },
        }

        Ok(())
    }
}

impl Serializable for PopularContact {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x5ce14175_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.client_id)?;
        buf.serialize(&self.importers)?;

        Ok(())
    }
 }

impl Serializable for InputStickeredMedia {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputStickeredMedia::Photo { .. } => buf.serialize(&0x4a992157_u32)?,
             InputStickeredMedia::Document { .. } => buf.serialize(&0x0438865b_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputStickeredMedia::Photo {
                ref id,
            } => {
                buf.serialize(id.as_ref())?;
            },

            InputStickeredMedia::Document {
                ref id,
            } => {
                buf.serialize(id.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for InputBotInlineResult {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputBotInlineResult::Photo { .. } => buf.serialize(&0xa8d864a7_u32)?,
             InputBotInlineResult::Result { .. } => buf.serialize(&0x2cbbe15a_u32)?,
             InputBotInlineResult::Game { .. } => buf.serialize(&0x4fa417f2_u32)?,
             InputBotInlineResult::Document { .. } => buf.serialize(&0xfff8fdc4_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputBotInlineResult::Photo {
                ref id,
                ref type_,
                ref photo,
                ref send_message,
            } => {
                buf.serialize(id)?;
                buf.serialize(type_)?;
                buf.serialize(photo.as_ref())?;
                buf.serialize(send_message.as_ref())?;
            },

            InputBotInlineResult::Result {
                flags: _,
                ref id,
                ref type_,
                ref title,
                ref description,
                ref url,
                ref thumb_url,
                ref content_url,
                ref content_type,
                ref w,
                ref h,
                ref duration,
                ref send_message,
            } => {
                let mut ser_flags: u32 = 0;

                if title.is_some() {
                    ser_flags |= 1 << 1;
                }

                if description.is_some() {
                    ser_flags |= 1 << 2;
                }

                if url.is_some() {
                    ser_flags |= 1 << 3;
                }

                if thumb_url.is_some() {
                    ser_flags |= 1 << 4;
                }

                if content_url.is_some() {
                    ser_flags |= 1 << 5;
                }

                if content_type.is_some() {
                    ser_flags |= 1 << 5;
                }

                if w.is_some() {
                    ser_flags |= 1 << 6;
                }

                if h.is_some() {
                    ser_flags |= 1 << 6;
                }

                if duration.is_some() {
                    ser_flags |= 1 << 7;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;
                buf.serialize(type_)?;

                if let Some(ref value) = title {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = description {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = url {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = thumb_url {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = content_url {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = content_type {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = w {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = h {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = duration {
                    buf.serialize(value)?;
                }

                buf.serialize(send_message.as_ref())?;
            },

            InputBotInlineResult::Game {
                ref id,
                ref short_name,
                ref send_message,
            } => {
                buf.serialize(id)?;
                buf.serialize(short_name)?;
                buf.serialize(send_message.as_ref())?;
            },

            InputBotInlineResult::Document {
                flags: _,
                ref id,
                ref type_,
                ref title,
                ref description,
                ref document,
                ref send_message,
            } => {
                let mut ser_flags: u32 = 0;

                if title.is_some() {
                    ser_flags |= 1 << 1;
                }

                if description.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;
                buf.serialize(type_)?;

                if let Some(ref value) = title {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = description {
                    buf.serialize(value)?;
                }

                buf.serialize(document.as_ref())?;
                buf.serialize(send_message.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for RichText {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             RichText::Bold { .. } => buf.serialize(&0x6724abc4_u32)?,
             RichText::Empty { .. } => buf.serialize(&0xdc3d824f_u32)?,
             RichText::Url { .. } => buf.serialize(&0x3c2884c1_u32)?,
             RichText::Italic { .. } => buf.serialize(&0xd912a59c_u32)?,
             RichText::Plain { .. } => buf.serialize(&0x744694e0_u32)?,
             RichText::Email { .. } => buf.serialize(&0xde5a0dd6_u32)?,
             RichText::Concat { .. } => buf.serialize(&0x7e6260d7_u32)?,
             RichText::Strike { .. } => buf.serialize(&0x9bf8bb95_u32)?,
             RichText::Underline { .. } => buf.serialize(&0xc12622c4_u32)?,
             RichText::Fixed { .. } => buf.serialize(&0x6c3f19b9_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            RichText::Bold {
                ref text,
            } => {
                buf.serialize(text.as_ref())?;
            },

            RichText::Empty {
            } => {
            },

            RichText::Url {
                ref text,
                ref url,
                ref webpage_id,
            } => {
                buf.serialize(text.as_ref())?;
                buf.serialize(url)?;
                buf.serialize(webpage_id)?;
            },

            RichText::Italic {
                ref text,
            } => {
                buf.serialize(text.as_ref())?;
            },

            RichText::Plain {
                ref text,
            } => {
                buf.serialize(text)?;
            },

            RichText::Email {
                ref text,
                ref email,
            } => {
                buf.serialize(text.as_ref())?;
                buf.serialize(email)?;
            },

            RichText::Concat {
                ref texts,
            } => {
                buf.serialize(texts)?;
            },

            RichText::Strike {
                ref text,
            } => {
                buf.serialize(text.as_ref())?;
            },

            RichText::Underline {
                ref text,
            } => {
                buf.serialize(text.as_ref())?;
            },

            RichText::Fixed {
                ref text,
            } => {
                buf.serialize(text.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for ChatParticipant {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ChatParticipant::Admin { .. } => buf.serialize(&0xe2d6e436_u32)?,
             ChatParticipant::Creator { .. } => buf.serialize(&0xda13538a_u32)?,
             ChatParticipant::Participant { .. } => buf.serialize(&0xc8d7493e_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ChatParticipant::Admin {
                ref user_id,
                ref inviter_id,
                ref date,
            } => {
                buf.serialize(user_id)?;
                buf.serialize(inviter_id)?;
                buf.serialize(date)?;
            },

            ChatParticipant::Creator {
                ref user_id,
            } => {
                buf.serialize(user_id)?;
            },

            ChatParticipant::Participant {
                ref user_id,
                ref inviter_id,
                ref date,
            } => {
                buf.serialize(user_id)?;
                buf.serialize(inviter_id)?;
                buf.serialize(date)?;
            },
        }

        Ok(())
    }
}

impl Serializable for DataJson {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x7d748d04_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.data)?;

        Ok(())
    }
 }

impl Serializable for NewSession {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             NewSession::Created { .. } => buf.serialize(&0x9ec20908_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            NewSession::Created {
                ref first_msg_id,
                ref unique_id,
                ref server_salt,
            } => {
                buf.serialize(first_msg_id)?;
                buf.serialize(unique_id)?;
                buf.serialize(server_salt)?;
            },
        }

        Ok(())
    }
}

impl Serializable for MsgResendReq {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x7d861a08_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.msg_ids)?;

        Ok(())
    }
 }

impl Serializable for ChannelAdminRights {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x5d7ceba5_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.change_info {
            ser_flags |= 1 << 0;
        }

        if self.post_messages {
            ser_flags |= 1 << 1;
        }

        if self.edit_messages {
            ser_flags |= 1 << 2;
        }

        if self.delete_messages {
            ser_flags |= 1 << 3;
        }

        if self.ban_users {
            ser_flags |= 1 << 4;
        }

        if self.invite_users {
            ser_flags |= 1 << 5;
        }

        if self.invite_link {
            ser_flags |= 1 << 6;
        }

        if self.pin_messages {
            ser_flags |= 1 << 7;
        }

        if self.add_admins {
            ser_flags |= 1 << 9;
        }

        if self.manage_call {
            ser_flags |= 1 << 10;
        }

        buf.serialize(&ser_flags)?;

        Ok(())
    }
 }

impl Serializable for AccountAuthorizations {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x1250abde_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.authorizations)?;

        Ok(())
    }
 }

impl Serializable for InputWebFileLocation {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc239d686_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.url)?;
        buf.serialize(&self.access_hash)?;

        Ok(())
    }
 }

impl Serializable for UserProfilePhoto {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             UserProfilePhoto::Photo { .. } => buf.serialize(&0xd559d8c8_u32)?,
             UserProfilePhoto::Empty { .. } => buf.serialize(&0x4f11bae1_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            UserProfilePhoto::Photo {
                ref photo_id,
                ref photo_small,
                ref photo_big,
            } => {
                buf.serialize(photo_id)?;
                buf.serialize(photo_small.as_ref())?;
                buf.serialize(photo_big.as_ref())?;
            },

            UserProfilePhoto::Empty {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for TopPeerCategoryPeers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xfb834291_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.category.as_ref())?;
        buf.serialize(&self.count)?;
        buf.serialize(&self.peers)?;

        Ok(())
    }
 }

impl Serializable for PhoneCallDiscardReason {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             PhoneCallDiscardReason::Busy { .. } => buf.serialize(&0xfaf7e8c9_u32)?,
             PhoneCallDiscardReason::Hangup { .. } => buf.serialize(&0x57adc690_u32)?,
             PhoneCallDiscardReason::Disconnect { .. } => buf.serialize(&0xe095c1a0_u32)?,
             PhoneCallDiscardReason::Missed { .. } => buf.serialize(&0x85e42301_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            PhoneCallDiscardReason::Busy {
            } => {
            },

            PhoneCallDiscardReason::Hangup {
            } => {
            },

            PhoneCallDiscardReason::Disconnect {
            } => {
            },

            PhoneCallDiscardReason::Missed {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for MessagesHighScores {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x9a3bfd99_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.scores)?;
        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for InputBotInlineMessageId {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x890c3d89_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.dc_id)?;
        buf.serialize(&self.id)?;
        buf.serialize(&self.access_hash)?;

        Ok(())
    }
 }

impl Serializable for Invoice {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc30aa358_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.test {
            ser_flags |= 1 << 0;
        }

        if self.name_requested {
            ser_flags |= 1 << 1;
        }

        if self.phone_requested {
            ser_flags |= 1 << 2;
        }

        if self.email_requested {
            ser_flags |= 1 << 3;
        }

        if self.shipping_address_requested {
            ser_flags |= 1 << 4;
        }

        if self.flexible {
            ser_flags |= 1 << 5;
        }

        if self.phone_to_provider {
            ser_flags |= 1 << 6;
        }

        if self.email_to_provider {
            ser_flags |= 1 << 7;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.currency)?;
        buf.serialize(&self.prices)?;

        Ok(())
    }
 }

impl Serializable for MessagesStickerSetInstallResult {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessagesStickerSetInstallResult::Archive { .. } => buf.serialize(&0x35e410a8_u32)?,
             MessagesStickerSetInstallResult::Success { .. } => buf.serialize(&0x38641628_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessagesStickerSetInstallResult::Archive {
                ref sets,
            } => {
                buf.serialize(sets)?;
            },

            MessagesStickerSetInstallResult::Success {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for HelpTermsOfService {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf1ee3e90_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.text)?;

        Ok(())
    }
 }

impl Serializable for MessagesAllStickers {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessagesAllStickers::Stickers { .. } => buf.serialize(&0xedfd405f_u32)?,
             MessagesAllStickers::NotModified { .. } => buf.serialize(&0xe86602c3_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessagesAllStickers::Stickers {
                ref hash,
                ref sets,
            } => {
                buf.serialize(hash)?;
                buf.serialize(sets)?;
            },

            MessagesAllStickers::NotModified {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for InputPeerNotifySettings {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x38935eb2_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.show_previews {
            ser_flags |= 1 << 0;
        }

        if self.silent {
            ser_flags |= 1 << 1;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.mute_until)?;
        buf.serialize(&self.sound)?;

        Ok(())
    }
 }

impl Serializable for SendMessageAction {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             SendMessageAction::RecordVideo { .. } => buf.serialize(&0xa187d66f_u32)?,
             SendMessageAction::UploadDocument { .. } => buf.serialize(&0xaa0cd9e4_u32)?,
             SendMessageAction::GamePlay { .. } => buf.serialize(&0xdd6a8f48_u32)?,
             SendMessageAction::ChooseContact { .. } => buf.serialize(&0x628cbc6f_u32)?,
             SendMessageAction::UploadPhoto { .. } => buf.serialize(&0xd1d34a26_u32)?,
             SendMessageAction::Typing { .. } => buf.serialize(&0x16bf744e_u32)?,
             SendMessageAction::RecordAudio { .. } => buf.serialize(&0xd52f73f7_u32)?,
             SendMessageAction::RecordRound { .. } => buf.serialize(&0x88f27fbc_u32)?,
             SendMessageAction::UploadVideo { .. } => buf.serialize(&0xe9763aec_u32)?,
             SendMessageAction::UploadRound { .. } => buf.serialize(&0x243e1c66_u32)?,
             SendMessageAction::UploadAudio { .. } => buf.serialize(&0xf351d7ab_u32)?,
             SendMessageAction::GeoLocation { .. } => buf.serialize(&0x176f8ba1_u32)?,
             SendMessageAction::Cancel { .. } => buf.serialize(&0xfd5ec8f5_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            SendMessageAction::RecordVideo {
            } => {
            },

            SendMessageAction::UploadDocument {
                ref progress,
            } => {
                buf.serialize(progress)?;
            },

            SendMessageAction::GamePlay {
            } => {
            },

            SendMessageAction::ChooseContact {
            } => {
            },

            SendMessageAction::UploadPhoto {
                ref progress,
            } => {
                buf.serialize(progress)?;
            },

            SendMessageAction::Typing {
            } => {
            },

            SendMessageAction::RecordAudio {
            } => {
            },

            SendMessageAction::RecordRound {
            } => {
            },

            SendMessageAction::UploadVideo {
                ref progress,
            } => {
                buf.serialize(progress)?;
            },

            SendMessageAction::UploadRound {
                ref progress,
            } => {
                buf.serialize(progress)?;
            },

            SendMessageAction::UploadAudio {
                ref progress,
            } => {
                buf.serialize(progress)?;
            },

            SendMessageAction::GeoLocation {
            } => {
            },

            SendMessageAction::Cancel {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for ContactsImportedContacts {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x77d01c3b_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.imported)?;
        buf.serialize(&self.popular_invites)?;
        buf.serialize(&self.retry_contacts)?;
        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for ChatFull {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ChatFull::ChannelFull { .. } => buf.serialize(&0x76af5481_u32)?,
             ChatFull::Full { .. } => buf.serialize(&0x2e02a614_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ChatFull::ChannelFull {
                flags: _,
                ref can_view_participants,
                ref can_set_username,
                ref can_set_stickers,
                ref hidden_prehistory,
                ref id,
                ref about,
                ref participants_count,
                ref admins_count,
                ref kicked_count,
                ref banned_count,
                ref read_inbox_max_id,
                ref read_outbox_max_id,
                ref unread_count,
                ref chat_photo,
                ref notify_settings,
                ref exported_invite,
                ref bot_info,
                ref migrated_from_chat_id,
                ref migrated_from_max_id,
                ref pinned_msg_id,
                ref stickerset,
                ref available_min_id,
            } => {
                let mut ser_flags: u32 = 0;

                if can_view_participants {
                    ser_flags |= 1 << 3;
                }

                if can_set_username {
                    ser_flags |= 1 << 6;
                }

                if can_set_stickers {
                    ser_flags |= 1 << 7;
                }

                if hidden_prehistory {
                    ser_flags |= 1 << 10;
                }

                if participants_count.is_some() {
                    ser_flags |= 1 << 0;
                }

                if admins_count.is_some() {
                    ser_flags |= 1 << 1;
                }

                if kicked_count.is_some() {
                    ser_flags |= 1 << 2;
                }

                if banned_count.is_some() {
                    ser_flags |= 1 << 2;
                }

                if migrated_from_chat_id.is_some() {
                    ser_flags |= 1 << 4;
                }

                if migrated_from_max_id.is_some() {
                    ser_flags |= 1 << 4;
                }

                if pinned_msg_id.is_some() {
                    ser_flags |= 1 << 5;
                }

                if stickerset.is_some() {
                    ser_flags |= 1 << 8;
                }

                if available_min_id.is_some() {
                    ser_flags |= 1 << 9;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;
                buf.serialize(about)?;

                if let Some(ref value) = participants_count {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = admins_count {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = kicked_count {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = banned_count {
                    buf.serialize(value)?;
                }

                buf.serialize(read_inbox_max_id)?;
                buf.serialize(read_outbox_max_id)?;
                buf.serialize(unread_count)?;
                buf.serialize(chat_photo.as_ref())?;
                buf.serialize(notify_settings.as_ref())?;
                buf.serialize(exported_invite.as_ref())?;
                buf.serialize(bot_info)?;

                if let Some(ref value) = migrated_from_chat_id {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = migrated_from_max_id {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = pinned_msg_id {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = stickerset {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = available_min_id {
                    buf.serialize(value)?;
                }

            },

            ChatFull::Full {
                ref id,
                ref participants,
                ref chat_photo,
                ref notify_settings,
                ref exported_invite,
                ref bot_info,
            } => {
                buf.serialize(id)?;
                buf.serialize(participants.as_ref())?;
                buf.serialize(chat_photo.as_ref())?;
                buf.serialize(notify_settings.as_ref())?;
                buf.serialize(exported_invite.as_ref())?;
                buf.serialize(bot_info)?;
            },
        }

        Ok(())
    }
}

impl Serializable for LabeledPrice {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xcb296bf8_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.label)?;
        buf.serialize(&self.amount)?;

        Ok(())
    }
 }

impl Serializable for MessagesFeaturedStickers {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessagesFeaturedStickers::Stickers { .. } => buf.serialize(&0xf89d88e5_u32)?,
             MessagesFeaturedStickers::NotModified { .. } => buf.serialize(&0x04ede3cf_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessagesFeaturedStickers::Stickers {
                ref hash,
                ref sets,
                ref unread,
            } => {
                buf.serialize(hash)?;
                buf.serialize(sets)?;
                buf.serialize(unread)?;
            },

            MessagesFeaturedStickers::NotModified {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for LangPackDifference {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf385c1f6_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.lang_code)?;
        buf.serialize(&self.from_version)?;
        buf.serialize(&self.version)?;
        buf.serialize(&self.strings)?;

        Ok(())
    }
 }

impl Serializable for EncryptedChat {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             EncryptedChat::Empty { .. } => buf.serialize(&0xab7ec0a0_u32)?,
             EncryptedChat::Discarded { .. } => buf.serialize(&0x13d6dd27_u32)?,
             EncryptedChat::Requested { .. } => buf.serialize(&0xc878527e_u32)?,
             EncryptedChat::Waiting { .. } => buf.serialize(&0x3bf703dc_u32)?,
             EncryptedChat::Chat { .. } => buf.serialize(&0xfa56ce36_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            EncryptedChat::Empty {
                ref id,
            } => {
                buf.serialize(id)?;
            },

            EncryptedChat::Discarded {
                ref id,
            } => {
                buf.serialize(id)?;
            },

            EncryptedChat::Requested {
                ref id,
                ref access_hash,
                ref date,
                ref admin_id,
                ref participant_id,
                ref g_a,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
                buf.serialize(date)?;
                buf.serialize(admin_id)?;
                buf.serialize(participant_id)?;
                buf.serialize(g_a)?;
            },

            EncryptedChat::Waiting {
                ref id,
                ref access_hash,
                ref date,
                ref admin_id,
                ref participant_id,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
                buf.serialize(date)?;
                buf.serialize(admin_id)?;
                buf.serialize(participant_id)?;
            },

            EncryptedChat::Chat {
                ref id,
                ref access_hash,
                ref date,
                ref admin_id,
                ref participant_id,
                ref g_a_or_b,
                ref key_fingerprint,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
                buf.serialize(date)?;
                buf.serialize(admin_id)?;
                buf.serialize(participant_id)?;
                buf.serialize(g_a_or_b)?;
                buf.serialize(key_fingerprint)?;
            },
        }

        Ok(())
    }
}

impl Serializable for FoundGif {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             FoundGif::Gif { .. } => buf.serialize(&0x162ecc1f_u32)?,
             FoundGif::Cached { .. } => buf.serialize(&0x9c750409_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            FoundGif::Gif {
                ref url,
                ref thumb_url,
                ref content_url,
                ref content_type,
                ref w,
                ref h,
            } => {
                buf.serialize(url)?;
                buf.serialize(thumb_url)?;
                buf.serialize(content_url)?;
                buf.serialize(content_type)?;
                buf.serialize(w)?;
                buf.serialize(h)?;
            },

            FoundGif::Cached {
                ref url,
                ref photo,
                ref document,
            } => {
                buf.serialize(url)?;
                buf.serialize(photo.as_ref())?;
                buf.serialize(document.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for FeedBroadcasts {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             FeedBroadcasts::Ungrouped { .. } => buf.serialize(&0x9a687cba_u32)?,
             FeedBroadcasts::Broadcasts { .. } => buf.serialize(&0x4f4feaf1_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            FeedBroadcasts::Ungrouped {
                ref channels,
            } => {
                buf.serialize(channels)?;
            },

            FeedBroadcasts::Broadcasts {
                ref feed_id,
                ref channels,
            } => {
                buf.serialize(feed_id)?;
                buf.serialize(channels)?;
            },
        }

        Ok(())
    }
}

impl Serializable for Updates {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             Updates::UpdateShortChatMessage { .. } => buf.serialize(&0x16812688_u32)?,
             Updates::UpdateShortMessage { .. } => buf.serialize(&0x914fbf11_u32)?,
             Updates::TooLong { .. } => buf.serialize(&0xe317af7e_u32)?,
             Updates::UpdateShortSentMessage { .. } => buf.serialize(&0x11f1331c_u32)?,
             Updates::Updates { .. } => buf.serialize(&0x74ae4240_u32)?,
             Updates::UpdateShort { .. } => buf.serialize(&0x78d4dec1_u32)?,
             Updates::Combined { .. } => buf.serialize(&0x725b04c3_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            Updates::UpdateShortChatMessage {
                flags: _,
                ref out,
                ref mentioned,
                ref media_unread,
                ref silent,
                ref id,
                ref from_id,
                ref chat_id,
                ref message,
                ref pts,
                ref pts_count,
                ref date,
                ref fwd_from,
                ref via_bot_id,
                ref reply_to_msg_id,
                ref entities,
            } => {
                let mut ser_flags: u32 = 0;

                if out {
                    ser_flags |= 1 << 1;
                }

                if mentioned {
                    ser_flags |= 1 << 4;
                }

                if media_unread {
                    ser_flags |= 1 << 5;
                }

                if silent {
                    ser_flags |= 1 << 13;
                }

                if fwd_from.is_some() {
                    ser_flags |= 1 << 2;
                }

                if via_bot_id.is_some() {
                    ser_flags |= 1 << 11;
                }

                if reply_to_msg_id.is_some() {
                    ser_flags |= 1 << 3;
                }

                if entities.is_some() {
                    ser_flags |= 1 << 7;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;
                buf.serialize(from_id)?;
                buf.serialize(chat_id)?;
                buf.serialize(message)?;
                buf.serialize(pts)?;
                buf.serialize(pts_count)?;
                buf.serialize(date)?;

                if let Some(ref value) = fwd_from {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = via_bot_id {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = reply_to_msg_id {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = entities {
                    buf.serialize(value)?;
                }

            },

            Updates::UpdateShortMessage {
                flags: _,
                ref out,
                ref mentioned,
                ref media_unread,
                ref silent,
                ref id,
                ref user_id,
                ref message,
                ref pts,
                ref pts_count,
                ref date,
                ref fwd_from,
                ref via_bot_id,
                ref reply_to_msg_id,
                ref entities,
            } => {
                let mut ser_flags: u32 = 0;

                if out {
                    ser_flags |= 1 << 1;
                }

                if mentioned {
                    ser_flags |= 1 << 4;
                }

                if media_unread {
                    ser_flags |= 1 << 5;
                }

                if silent {
                    ser_flags |= 1 << 13;
                }

                if fwd_from.is_some() {
                    ser_flags |= 1 << 2;
                }

                if via_bot_id.is_some() {
                    ser_flags |= 1 << 11;
                }

                if reply_to_msg_id.is_some() {
                    ser_flags |= 1 << 3;
                }

                if entities.is_some() {
                    ser_flags |= 1 << 7;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;
                buf.serialize(user_id)?;
                buf.serialize(message)?;
                buf.serialize(pts)?;
                buf.serialize(pts_count)?;
                buf.serialize(date)?;

                if let Some(ref value) = fwd_from {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = via_bot_id {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = reply_to_msg_id {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = entities {
                    buf.serialize(value)?;
                }

            },

            Updates::TooLong {
            } => {
            },

            Updates::UpdateShortSentMessage {
                flags: _,
                ref out,
                ref id,
                ref pts,
                ref pts_count,
                ref date,
                ref media,
                ref entities,
            } => {
                let mut ser_flags: u32 = 0;

                if out {
                    ser_flags |= 1 << 1;
                }

                if media.is_some() {
                    ser_flags |= 1 << 9;
                }

                if entities.is_some() {
                    ser_flags |= 1 << 7;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;
                buf.serialize(pts)?;
                buf.serialize(pts_count)?;
                buf.serialize(date)?;

                if let Some(ref value) = media {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = entities {
                    buf.serialize(value)?;
                }

            },

            Updates::Updates {
                ref updates,
                ref users,
                ref chats,
                ref date,
                ref seq,
            } => {
                buf.serialize(updates)?;
                buf.serialize(users)?;
                buf.serialize(chats)?;
                buf.serialize(date)?;
                buf.serialize(seq)?;
            },

            Updates::UpdateShort {
                ref update,
                ref date,
            } => {
                buf.serialize(update.as_ref())?;
                buf.serialize(date)?;
            },

            Updates::Combined {
                ref updates,
                ref users,
                ref chats,
                ref date,
                ref seq_start,
                ref seq,
            } => {
                buf.serialize(updates)?;
                buf.serialize(users)?;
                buf.serialize(chats)?;
                buf.serialize(date)?;
                buf.serialize(seq_start)?;
                buf.serialize(seq)?;
            },
        }

        Ok(())
    }
}

impl Serializable for PhotosPhotos {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             PhotosPhotos::PhotosSlice { .. } => buf.serialize(&0x15051f54_u32)?,
             PhotosPhotos::Photos { .. } => buf.serialize(&0x8dca6aa5_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            PhotosPhotos::PhotosSlice {
                ref count,
                ref photos,
                ref users,
            } => {
                buf.serialize(count)?;
                buf.serialize(photos)?;
                buf.serialize(users)?;
            },

            PhotosPhotos::Photos {
                ref photos,
                ref users,
            } => {
                buf.serialize(photos)?;
                buf.serialize(users)?;
            },
        }

        Ok(())
    }
}

impl Serializable for RpcDropAnswer {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             RpcDropAnswer::AnswerUnknown { .. } => buf.serialize(&0x5e2ad36e_u32)?,
             RpcDropAnswer::AnswerDroppedRunning { .. } => buf.serialize(&0xcd78e586_u32)?,
             RpcDropAnswer::AnswerDropped { .. } => buf.serialize(&0xa43ad8b7_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            RpcDropAnswer::AnswerUnknown {
            } => {
            },

            RpcDropAnswer::AnswerDroppedRunning {
            } => {
            },

            RpcDropAnswer::AnswerDropped {
                ref msg_id,
                ref seq_no,
                ref bytes,
            } => {
                buf.serialize(msg_id)?;
                buf.serialize(seq_no)?;
                buf.serialize(bytes)?;
            },
        }

        Ok(())
    }
}

impl Serializable for InputChatPhoto {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputChatPhoto::Photo { .. } => buf.serialize(&0x8953ad37_u32)?,
             InputChatPhoto::Uploaded { .. } => buf.serialize(&0x927c55b4_u32)?,
             InputChatPhoto::Empty { .. } => buf.serialize(&0x1ca48f57_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputChatPhoto::Photo {
                ref id,
            } => {
                buf.serialize(id.as_ref())?;
            },

            InputChatPhoto::Uploaded {
                ref file,
            } => {
                buf.serialize(file.as_ref())?;
            },

            InputChatPhoto::Empty {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for InputDocument {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputDocument::Document { .. } => buf.serialize(&0x18798952_u32)?,
             InputDocument::Empty { .. } => buf.serialize(&0x72f0eaae_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputDocument::Document {
                ref id,
                ref access_hash,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
            },

            InputDocument::Empty {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for ImportedContact {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xd0028438_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.user_id)?;
        buf.serialize(&self.client_id)?;

        Ok(())
    }
 }

impl Serializable for UploadWebFile {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x21e753bc_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.size)?;
        buf.serialize(&self.mime_type)?;
        buf.serialize(self.file_type.as_ref())?;
        buf.serialize(&self.mtime)?;
        buf.serialize(&self.bytes)?;

        Ok(())
    }
 }

impl Serializable for MsgsAck {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x62d6b459_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.msg_ids)?;

        Ok(())
    }
 }

impl Serializable for InputFile {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputFile::Big { .. } => buf.serialize(&0xfa4f0bb5_u32)?,
             InputFile::File { .. } => buf.serialize(&0xf52ff27f_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputFile::Big {
                ref id,
                ref parts,
                ref name,
            } => {
                buf.serialize(id)?;
                buf.serialize(parts)?;
                buf.serialize(name)?;
            },

            InputFile::File {
                ref id,
                ref parts,
                ref name,
                ref md5_checksum,
            } => {
                buf.serialize(id)?;
                buf.serialize(parts)?;
                buf.serialize(name)?;
                buf.serialize(md5_checksum)?;
            },
        }

        Ok(())
    }
}

impl Serializable for PaymentsPaymentReceipt {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x500911e1_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.info.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.shipping.is_some() {
            ser_flags |= 1 << 1;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.date)?;
        buf.serialize(&self.bot_id)?;
        buf.serialize(self.invoice.as_ref())?;
        buf.serialize(&self.provider_id)?;

        if let Some(ref value) = self.info {
            buf.serialize(value.as_ref())?;
        }


        if let Some(ref value) = self.shipping {
            buf.serialize(value.as_ref())?;
        }

        buf.serialize(&self.currency)?;
        buf.serialize(&self.total_amount)?;
        buf.serialize(&self.credentials_title)?;
        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for MessagesFeedMessages {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessagesFeedMessages::NotModified { .. } => buf.serialize(&0x4678d0cf_u32)?,
             MessagesFeedMessages::Messages { .. } => buf.serialize(&0x55c3a1b1_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessagesFeedMessages::NotModified {
            } => {
            },

            MessagesFeedMessages::Messages {
                flags: _,
                ref max_position,
                ref min_position,
                ref read_max_position,
                ref messages,
                ref chats,
                ref users,
            } => {
                let mut ser_flags: u32 = 0;

                if max_position.is_some() {
                    ser_flags |= 1 << 0;
                }

                if min_position.is_some() {
                    ser_flags |= 1 << 1;
                }

                if read_max_position.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;

                if let Some(ref value) = max_position {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = min_position {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = read_max_position {
                    buf.serialize(value.as_ref())?;
                }

                buf.serialize(messages)?;
                buf.serialize(chats)?;
                buf.serialize(users)?;
            },
        }

        Ok(())
    }
}

impl Serializable for HttpWait {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x9299359f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.max_delay)?;
        buf.serialize(&self.wait_after)?;
        buf.serialize(&self.max_wait)?;

        Ok(())
    }
 }

impl Serializable for DisabledFeature {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xae636f24_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.feature)?;
        buf.serialize(&self.description)?;

        Ok(())
    }
 }

impl Serializable for PrivacyKey {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             PrivacyKey::PhoneCall { .. } => buf.serialize(&0x3d662b7b_u32)?,
             PrivacyKey::StatusTimestamp { .. } => buf.serialize(&0xbc2eab30_u32)?,
             PrivacyKey::ChatInvite { .. } => buf.serialize(&0x500e6dfa_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            PrivacyKey::PhoneCall {
            } => {
            },

            PrivacyKey::StatusTimestamp {
            } => {
            },

            PrivacyKey::ChatInvite {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for BotInlineMessage {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             BotInlineMessage::MediaVenue { .. } => buf.serialize(&0x4366232e_u32)?,
             BotInlineMessage::Text { .. } => buf.serialize(&0x8c7f65e2_u32)?,
             BotInlineMessage::MediaGeo { .. } => buf.serialize(&0xb722de65_u32)?,
             BotInlineMessage::MediaContact { .. } => buf.serialize(&0x35edb4d4_u32)?,
             BotInlineMessage::MediaAuto { .. } => buf.serialize(&0x0a74b15b_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            BotInlineMessage::MediaVenue {
                flags: _,
                ref geo,
                ref title,
                ref address,
                ref provider,
                ref venue_id,
                ref reply_markup,
            } => {
                let mut ser_flags: u32 = 0;

                if reply_markup.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(geo.as_ref())?;
                buf.serialize(title)?;
                buf.serialize(address)?;
                buf.serialize(provider)?;
                buf.serialize(venue_id)?;

                if let Some(ref value) = reply_markup {
                    buf.serialize(value.as_ref())?;
                }

            },

            BotInlineMessage::Text {
                flags: _,
                ref no_webpage,
                ref message,
                ref entities,
                ref reply_markup,
            } => {
                let mut ser_flags: u32 = 0;

                if no_webpage {
                    ser_flags |= 1 << 0;
                }

                if entities.is_some() {
                    ser_flags |= 1 << 1;
                }

                if reply_markup.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(message)?;

                if let Some(ref value) = entities {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = reply_markup {
                    buf.serialize(value.as_ref())?;
                }

            },

            BotInlineMessage::MediaGeo {
                flags: _,
                ref geo,
                ref period,
                ref reply_markup,
            } => {
                let mut ser_flags: u32 = 0;

                if reply_markup.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(geo.as_ref())?;
                buf.serialize(period)?;

                if let Some(ref value) = reply_markup {
                    buf.serialize(value.as_ref())?;
                }

            },

            BotInlineMessage::MediaContact {
                flags: _,
                ref phone_number,
                ref first_name,
                ref last_name,
                ref reply_markup,
            } => {
                let mut ser_flags: u32 = 0;

                if reply_markup.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(phone_number)?;
                buf.serialize(first_name)?;
                buf.serialize(last_name)?;

                if let Some(ref value) = reply_markup {
                    buf.serialize(value.as_ref())?;
                }

            },

            BotInlineMessage::MediaAuto {
                flags: _,
                ref caption,
                ref reply_markup,
            } => {
                let mut ser_flags: u32 = 0;

                if reply_markup.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(caption)?;

                if let Some(ref value) = reply_markup {
                    buf.serialize(value.as_ref())?;
                }

            },
        }

        Ok(())
    }
}

impl Serializable for BotInlineResult {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             BotInlineResult::Result { .. } => buf.serialize(&0x9bebaeb9_u32)?,
             BotInlineResult::Media { .. } => buf.serialize(&0x17db940b_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            BotInlineResult::Result {
                flags: _,
                ref id,
                ref type_,
                ref title,
                ref description,
                ref url,
                ref thumb_url,
                ref content_url,
                ref content_type,
                ref w,
                ref h,
                ref duration,
                ref send_message,
            } => {
                let mut ser_flags: u32 = 0;

                if title.is_some() {
                    ser_flags |= 1 << 1;
                }

                if description.is_some() {
                    ser_flags |= 1 << 2;
                }

                if url.is_some() {
                    ser_flags |= 1 << 3;
                }

                if thumb_url.is_some() {
                    ser_flags |= 1 << 4;
                }

                if content_url.is_some() {
                    ser_flags |= 1 << 5;
                }

                if content_type.is_some() {
                    ser_flags |= 1 << 5;
                }

                if w.is_some() {
                    ser_flags |= 1 << 6;
                }

                if h.is_some() {
                    ser_flags |= 1 << 6;
                }

                if duration.is_some() {
                    ser_flags |= 1 << 7;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;
                buf.serialize(type_)?;

                if let Some(ref value) = title {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = description {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = url {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = thumb_url {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = content_url {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = content_type {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = w {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = h {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = duration {
                    buf.serialize(value)?;
                }

                buf.serialize(send_message.as_ref())?;
            },

            BotInlineResult::Media {
                flags: _,
                ref id,
                ref type_,
                ref photo,
                ref document,
                ref title,
                ref description,
                ref send_message,
            } => {
                let mut ser_flags: u32 = 0;

                if photo.is_some() {
                    ser_flags |= 1 << 0;
                }

                if document.is_some() {
                    ser_flags |= 1 << 1;
                }

                if title.is_some() {
                    ser_flags |= 1 << 2;
                }

                if description.is_some() {
                    ser_flags |= 1 << 3;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;
                buf.serialize(type_)?;

                if let Some(ref value) = photo {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = document {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = title {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = description {
                    buf.serialize(value)?;
                }

                buf.serialize(send_message.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for KeyboardButton {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             KeyboardButton::SwitchInline { .. } => buf.serialize(&0x0568a748_u32)?,
             KeyboardButton::RequestGeoLocation { .. } => buf.serialize(&0xfc796b3f_u32)?,
             KeyboardButton::Url { .. } => buf.serialize(&0x258aff05_u32)?,
             KeyboardButton::Button { .. } => buf.serialize(&0xa2fa4880_u32)?,
             KeyboardButton::RequestPhone { .. } => buf.serialize(&0xb16a6c29_u32)?,
             KeyboardButton::Callback { .. } => buf.serialize(&0x683a5e46_u32)?,
             KeyboardButton::Buy { .. } => buf.serialize(&0xafd93fbb_u32)?,
             KeyboardButton::Game { .. } => buf.serialize(&0x50f41ccf_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            KeyboardButton::SwitchInline {
                flags: _,
                ref same_peer,
                ref text,
                ref query,
            } => {
                let mut ser_flags: u32 = 0;

                if same_peer {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(text)?;
                buf.serialize(query)?;
            },

            KeyboardButton::RequestGeoLocation {
                ref text,
            } => {
                buf.serialize(text)?;
            },

            KeyboardButton::Url {
                ref text,
                ref url,
            } => {
                buf.serialize(text)?;
                buf.serialize(url)?;
            },

            KeyboardButton::Button {
                ref text,
            } => {
                buf.serialize(text)?;
            },

            KeyboardButton::RequestPhone {
                ref text,
            } => {
                buf.serialize(text)?;
            },

            KeyboardButton::Callback {
                ref text,
                ref data,
            } => {
                buf.serialize(text)?;
                buf.serialize(data)?;
            },

            KeyboardButton::Buy {
                ref text,
            } => {
                buf.serialize(text)?;
            },

            KeyboardButton::Game {
                ref text,
            } => {
                buf.serialize(text)?;
            },
        }

        Ok(())
    }
}

impl Serializable for EncryptedMessage {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             EncryptedMessage::Service { .. } => buf.serialize(&0x23734b06_u32)?,
             EncryptedMessage::Message { .. } => buf.serialize(&0xed18c118_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            EncryptedMessage::Service {
                ref random_id,
                ref chat_id,
                ref date,
                ref bytes,
            } => {
                buf.serialize(random_id)?;
                buf.serialize(chat_id)?;
                buf.serialize(date)?;
                buf.serialize(bytes)?;
            },

            EncryptedMessage::Message {
                ref random_id,
                ref chat_id,
                ref date,
                ref bytes,
                ref file,
            } => {
                buf.serialize(random_id)?;
                buf.serialize(chat_id)?;
                buf.serialize(date)?;
                buf.serialize(bytes)?;
                buf.serialize(file.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for StickerSetCovered {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             StickerSetCovered::Multi { .. } => buf.serialize(&0x3407e51b_u32)?,
             StickerSetCovered::Covered { .. } => buf.serialize(&0x6410a5d2_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            StickerSetCovered::Multi {
                ref set,
                ref covers,
            } => {
                buf.serialize(set.as_ref())?;
                buf.serialize(covers)?;
            },

            StickerSetCovered::Covered {
                ref set,
                ref cover,
            } => {
                buf.serialize(set.as_ref())?;
                buf.serialize(cover.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for MessagesRecentStickers {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessagesRecentStickers::Stickers { .. } => buf.serialize(&0x5ce20970_u32)?,
             MessagesRecentStickers::NotModified { .. } => buf.serialize(&0x0b17f890_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessagesRecentStickers::Stickers {
                ref hash,
                ref stickers,
            } => {
                buf.serialize(hash)?;
                buf.serialize(stickers)?;
            },

            MessagesRecentStickers::NotModified {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for InputUser {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputUser::Empty { .. } => buf.serialize(&0xb98886cf_u32)?,
             InputUser::User { .. } => buf.serialize(&0xd8292816_u32)?,
             InputUser::Self_ { .. } => buf.serialize(&0xf7c1b13f_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputUser::Empty {
            } => {
            },

            InputUser::User {
                ref user_id,
                ref access_hash,
            } => {
                buf.serialize(user_id)?;
                buf.serialize(access_hash)?;
            },

            InputUser::Self_ {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for ChannelAdminLogEventsFilter {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xea107ae4_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.join {
            ser_flags |= 1 << 0;
        }

        if self.leave {
            ser_flags |= 1 << 1;
        }

        if self.invite {
            ser_flags |= 1 << 2;
        }

        if self.ban {
            ser_flags |= 1 << 3;
        }

        if self.unban {
            ser_flags |= 1 << 4;
        }

        if self.kick {
            ser_flags |= 1 << 5;
        }

        if self.unkick {
            ser_flags |= 1 << 6;
        }

        if self.promote {
            ser_flags |= 1 << 7;
        }

        if self.demote {
            ser_flags |= 1 << 8;
        }

        if self.info {
            ser_flags |= 1 << 9;
        }

        if self.settings {
            ser_flags |= 1 << 10;
        }

        if self.pinned {
            ser_flags |= 1 << 11;
        }

        if self.edit {
            ser_flags |= 1 << 12;
        }

        if self.delete {
            ser_flags |= 1 << 13;
        }

        buf.serialize(&ser_flags)?;

        Ok(())
    }
 }

impl Serializable for InputMedia {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputMedia::DocumentExternal { .. } => buf.serialize(&0xb6f74335_u32)?,
             InputMedia::Venue { .. } => buf.serialize(&0xc13d1c11_u32)?,
             InputMedia::Contact { .. } => buf.serialize(&0xa6e45987_u32)?,
             InputMedia::GifExternal { .. } => buf.serialize(&0x4843b0fd_u32)?,
             InputMedia::Document { .. } => buf.serialize(&0x5acb668e_u32)?,
             InputMedia::PhotoExternal { .. } => buf.serialize(&0x0922aec1_u32)?,
             InputMedia::Photo { .. } => buf.serialize(&0x81fa373a_u32)?,
             InputMedia::UploadedDocument { .. } => buf.serialize(&0xe39621fd_u32)?,
             InputMedia::GeoPoint { .. } => buf.serialize(&0xf9c44144_u32)?,
             InputMedia::Invoice { .. } => buf.serialize(&0xf4e096c3_u32)?,
             InputMedia::Empty { .. } => buf.serialize(&0x9664f57f_u32)?,
             InputMedia::GeoLive { .. } => buf.serialize(&0x7b1a118f_u32)?,
             InputMedia::Game { .. } => buf.serialize(&0xd33f43f3_u32)?,
             InputMedia::UploadedPhoto { .. } => buf.serialize(&0x2f37e231_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputMedia::DocumentExternal {
                flags: _,
                ref url,
                ref caption,
                ref ttl_seconds,
            } => {
                let mut ser_flags: u32 = 0;

                if ttl_seconds.is_some() {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(url)?;
                buf.serialize(caption)?;

                if let Some(ref value) = ttl_seconds {
                    buf.serialize(value)?;
                }

            },

            InputMedia::Venue {
                ref geo_point,
                ref title,
                ref address,
                ref provider,
                ref venue_id,
                ref venue_type,
            } => {
                buf.serialize(geo_point.as_ref())?;
                buf.serialize(title)?;
                buf.serialize(address)?;
                buf.serialize(provider)?;
                buf.serialize(venue_id)?;
                buf.serialize(venue_type)?;
            },

            InputMedia::Contact {
                ref phone_number,
                ref first_name,
                ref last_name,
            } => {
                buf.serialize(phone_number)?;
                buf.serialize(first_name)?;
                buf.serialize(last_name)?;
            },

            InputMedia::GifExternal {
                ref url,
                ref q,
            } => {
                buf.serialize(url)?;
                buf.serialize(q)?;
            },

            InputMedia::Document {
                flags: _,
                ref id,
                ref caption,
                ref ttl_seconds,
            } => {
                let mut ser_flags: u32 = 0;

                if ttl_seconds.is_some() {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id.as_ref())?;
                buf.serialize(caption)?;

                if let Some(ref value) = ttl_seconds {
                    buf.serialize(value)?;
                }

            },

            InputMedia::PhotoExternal {
                flags: _,
                ref url,
                ref caption,
                ref ttl_seconds,
            } => {
                let mut ser_flags: u32 = 0;

                if ttl_seconds.is_some() {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(url)?;
                buf.serialize(caption)?;

                if let Some(ref value) = ttl_seconds {
                    buf.serialize(value)?;
                }

            },

            InputMedia::Photo {
                flags: _,
                ref id,
                ref caption,
                ref ttl_seconds,
            } => {
                let mut ser_flags: u32 = 0;

                if ttl_seconds.is_some() {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id.as_ref())?;
                buf.serialize(caption)?;

                if let Some(ref value) = ttl_seconds {
                    buf.serialize(value)?;
                }

            },

            InputMedia::UploadedDocument {
                flags: _,
                ref nosound_video,
                ref file,
                ref thumb,
                ref mime_type,
                ref attributes,
                ref caption,
                ref stickers,
                ref ttl_seconds,
            } => {
                let mut ser_flags: u32 = 0;

                if nosound_video {
                    ser_flags |= 1 << 3;
                }

                if thumb.is_some() {
                    ser_flags |= 1 << 2;
                }

                if stickers.is_some() {
                    ser_flags |= 1 << 0;
                }

                if ttl_seconds.is_some() {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(file.as_ref())?;

                if let Some(ref value) = thumb {
                    buf.serialize(value.as_ref())?;
                }

                buf.serialize(mime_type)?;
                buf.serialize(attributes)?;
                buf.serialize(caption)?;

                if let Some(ref value) = stickers {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = ttl_seconds {
                    buf.serialize(value)?;
                }

            },

            InputMedia::GeoPoint {
                ref geo_point,
            } => {
                buf.serialize(geo_point.as_ref())?;
            },

            InputMedia::Invoice {
                flags: _,
                ref title,
                ref description,
                ref photo,
                ref invoice,
                ref payload,
                ref provider,
                ref provider_data,
                ref start_param,
            } => {
                let mut ser_flags: u32 = 0;

                if photo.is_some() {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(title)?;
                buf.serialize(description)?;

                if let Some(ref value) = photo {
                    buf.serialize(value.as_ref())?;
                }

                buf.serialize(invoice.as_ref())?;
                buf.serialize(payload)?;
                buf.serialize(provider)?;
                buf.serialize(provider_data.as_ref())?;
                buf.serialize(start_param)?;
            },

            InputMedia::Empty {
            } => {
            },

            InputMedia::GeoLive {
                ref geo_point,
                ref period,
            } => {
                buf.serialize(geo_point.as_ref())?;
                buf.serialize(period)?;
            },

            InputMedia::Game {
                ref id,
            } => {
                buf.serialize(id.as_ref())?;
            },

            InputMedia::UploadedPhoto {
                flags: _,
                ref file,
                ref caption,
                ref stickers,
                ref ttl_seconds,
            } => {
                let mut ser_flags: u32 = 0;

                if stickers.is_some() {
                    ser_flags |= 1 << 0;
                }

                if ttl_seconds.is_some() {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(file.as_ref())?;
                buf.serialize(caption)?;

                if let Some(ref value) = stickers {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = ttl_seconds {
                    buf.serialize(value)?;
                }

            },
        }

        Ok(())
    }
}

impl Serializable for EncryptedFile {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             EncryptedFile::Empty { .. } => buf.serialize(&0xc21f497e_u32)?,
             EncryptedFile::File { .. } => buf.serialize(&0x4a70994c_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            EncryptedFile::Empty {
            } => {
            },

            EncryptedFile::File {
                ref id,
                ref access_hash,
                ref size,
                ref dc_id,
                ref key_fingerprint,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
                buf.serialize(size)?;
                buf.serialize(dc_id)?;
                buf.serialize(key_fingerprint)?;
            },
        }

        Ok(())
    }
}

impl Serializable for AuthSentCode {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x5e002502_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.phone_registered {
            ser_flags |= 1 << 0;
        }

        if self.next_type.is_some() {
            ser_flags |= 1 << 1;
        }

        if self.timeout.is_some() {
            ser_flags |= 1 << 2;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.type_.as_ref())?;
        buf.serialize(&self.phone_code_hash)?;

        if let Some(ref value) = self.next_type {
            buf.serialize(value.as_ref())?;
        }


        if let Some(ref value) = self.timeout {
            buf.serialize(value)?;
        }


        Ok(())
    }
 }

impl Serializable for PhoneCallProtocol {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xa2bb35cb_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.udp_p2p {
            ser_flags |= 1 << 0;
        }

        if self.udp_reflector {
            ser_flags |= 1 << 1;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.min_layer)?;
        buf.serialize(&self.max_layer)?;

        Ok(())
    }
 }

impl Serializable for ChannelBannedRights {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x58cf4249_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.view_messages {
            ser_flags |= 1 << 0;
        }

        if self.send_messages {
            ser_flags |= 1 << 1;
        }

        if self.send_media {
            ser_flags |= 1 << 2;
        }

        if self.send_stickers {
            ser_flags |= 1 << 3;
        }

        if self.send_gifs {
            ser_flags |= 1 << 4;
        }

        if self.send_games {
            ser_flags |= 1 << 5;
        }

        if self.send_inline {
            ser_flags |= 1 << 6;
        }

        if self.embed_links {
            ser_flags |= 1 << 7;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.until_date)?;

        Ok(())
    }
 }

impl Serializable for TopPeer {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xedcdc05b_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.rating)?;

        Ok(())
    }
 }

impl Serializable for InputSingleMedia {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x5eaa7809_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.media.as_ref())?;
        buf.serialize(&self.random_id)?;

        Ok(())
    }
 }

impl Serializable for ExportedChatInvite {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ExportedChatInvite::Exported { .. } => buf.serialize(&0xfc2e05bc_u32)?,
             ExportedChatInvite::Empty { .. } => buf.serialize(&0x69df3769_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ExportedChatInvite::Exported {
                ref link,
            } => {
                buf.serialize(link)?;
            },

            ExportedChatInvite::Empty {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for PhoneCall {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             PhoneCall::Call { .. } => buf.serialize(&0xffe6ab67_u32)?,
             PhoneCall::Accepted { .. } => buf.serialize(&0x6d003d3f_u32)?,
             PhoneCall::Waiting { .. } => buf.serialize(&0x1b8f4ad1_u32)?,
             PhoneCall::Empty { .. } => buf.serialize(&0x5366c915_u32)?,
             PhoneCall::Discarded { .. } => buf.serialize(&0x50ca4de1_u32)?,
             PhoneCall::Requested { .. } => buf.serialize(&0x83761ce4_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            PhoneCall::Call {
                ref id,
                ref access_hash,
                ref date,
                ref admin_id,
                ref participant_id,
                ref g_a_or_b,
                ref key_fingerprint,
                ref protocol,
                ref connection,
                ref alternative_connections,
                ref start_date,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
                buf.serialize(date)?;
                buf.serialize(admin_id)?;
                buf.serialize(participant_id)?;
                buf.serialize(g_a_or_b)?;
                buf.serialize(key_fingerprint)?;
                buf.serialize(protocol.as_ref())?;
                buf.serialize(connection.as_ref())?;
                buf.serialize(alternative_connections)?;
                buf.serialize(start_date)?;
            },

            PhoneCall::Accepted {
                ref id,
                ref access_hash,
                ref date,
                ref admin_id,
                ref participant_id,
                ref g_b,
                ref protocol,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
                buf.serialize(date)?;
                buf.serialize(admin_id)?;
                buf.serialize(participant_id)?;
                buf.serialize(g_b)?;
                buf.serialize(protocol.as_ref())?;
            },

            PhoneCall::Waiting {
                flags: _,
                ref id,
                ref access_hash,
                ref date,
                ref admin_id,
                ref participant_id,
                ref protocol,
                ref receive_date,
            } => {
                let mut ser_flags: u32 = 0;

                if receive_date.is_some() {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
                buf.serialize(date)?;
                buf.serialize(admin_id)?;
                buf.serialize(participant_id)?;
                buf.serialize(protocol.as_ref())?;

                if let Some(ref value) = receive_date {
                    buf.serialize(value)?;
                }

            },

            PhoneCall::Empty {
                ref id,
            } => {
                buf.serialize(id)?;
            },

            PhoneCall::Discarded {
                flags: _,
                ref need_rating,
                ref need_debug,
                ref id,
                ref reason,
                ref duration,
            } => {
                let mut ser_flags: u32 = 0;

                if need_rating {
                    ser_flags |= 1 << 2;
                }

                if need_debug {
                    ser_flags |= 1 << 3;
                }

                if reason.is_some() {
                    ser_flags |= 1 << 0;
                }

                if duration.is_some() {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;

                if let Some(ref value) = reason {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = duration {
                    buf.serialize(value)?;
                }

            },

            PhoneCall::Requested {
                ref id,
                ref access_hash,
                ref date,
                ref admin_id,
                ref participant_id,
                ref g_a_hash,
                ref protocol,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
                buf.serialize(date)?;
                buf.serialize(admin_id)?;
                buf.serialize(participant_id)?;
                buf.serialize(g_a_hash)?;
                buf.serialize(protocol.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for Chat {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             Chat::Channel { .. } => buf.serialize(&0xc88974ac_u32)?,
             Chat::Empty { .. } => buf.serialize(&0x9ba2d800_u32)?,
             Chat::Chat { .. } => buf.serialize(&0xd91cdd54_u32)?,
             Chat::Forbidden { .. } => buf.serialize(&0x07328bdb_u32)?,
             Chat::ChannelForbidden { .. } => buf.serialize(&0x289da732_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            Chat::Channel {
                flags: _,
                ref creator,
                ref left,
                ref editor,
                ref broadcast,
                ref verified,
                ref megagroup,
                ref restricted,
                ref democracy,
                ref signatures,
                ref min,
                ref id,
                ref access_hash,
                ref title,
                ref username,
                ref photo,
                ref date,
                ref version,
                ref restriction_reason,
                ref admin_rights,
                ref banned_rights,
                ref participants_count,
                ref feed_id,
            } => {
                let mut ser_flags: u32 = 0;

                if creator {
                    ser_flags |= 1 << 0;
                }

                if left {
                    ser_flags |= 1 << 2;
                }

                if editor {
                    ser_flags |= 1 << 3;
                }

                if broadcast {
                    ser_flags |= 1 << 5;
                }

                if verified {
                    ser_flags |= 1 << 7;
                }

                if megagroup {
                    ser_flags |= 1 << 8;
                }

                if restricted {
                    ser_flags |= 1 << 9;
                }

                if democracy {
                    ser_flags |= 1 << 10;
                }

                if signatures {
                    ser_flags |= 1 << 11;
                }

                if min {
                    ser_flags |= 1 << 12;
                }

                if access_hash.is_some() {
                    ser_flags |= 1 << 13;
                }

                if username.is_some() {
                    ser_flags |= 1 << 6;
                }

                if restriction_reason.is_some() {
                    ser_flags |= 1 << 9;
                }

                if admin_rights.is_some() {
                    ser_flags |= 1 << 14;
                }

                if banned_rights.is_some() {
                    ser_flags |= 1 << 15;
                }

                if participants_count.is_some() {
                    ser_flags |= 1 << 17;
                }

                if feed_id.is_some() {
                    ser_flags |= 1 << 18;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;

                if let Some(ref value) = access_hash {
                    buf.serialize(value)?;
                }

                buf.serialize(title)?;

                if let Some(ref value) = username {
                    buf.serialize(value)?;
                }

                buf.serialize(photo.as_ref())?;
                buf.serialize(date)?;
                buf.serialize(version)?;

                if let Some(ref value) = restriction_reason {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = admin_rights {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = banned_rights {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = participants_count {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = feed_id {
                    buf.serialize(value)?;
                }

            },

            Chat::Empty {
                ref id,
            } => {
                buf.serialize(id)?;
            },

            Chat::Chat {
                flags: _,
                ref creator,
                ref kicked,
                ref left,
                ref admins_enabled,
                ref admin,
                ref deactivated,
                ref id,
                ref title,
                ref photo,
                ref participants_count,
                ref date,
                ref version,
                ref migrated_to,
            } => {
                let mut ser_flags: u32 = 0;

                if creator {
                    ser_flags |= 1 << 0;
                }

                if kicked {
                    ser_flags |= 1 << 1;
                }

                if left {
                    ser_flags |= 1 << 2;
                }

                if admins_enabled {
                    ser_flags |= 1 << 3;
                }

                if admin {
                    ser_flags |= 1 << 4;
                }

                if deactivated {
                    ser_flags |= 1 << 5;
                }

                if migrated_to.is_some() {
                    ser_flags |= 1 << 6;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;
                buf.serialize(title)?;
                buf.serialize(photo.as_ref())?;
                buf.serialize(participants_count)?;
                buf.serialize(date)?;
                buf.serialize(version)?;

                if let Some(ref value) = migrated_to {
                    buf.serialize(value.as_ref())?;
                }

            },

            Chat::Forbidden {
                ref id,
                ref title,
            } => {
                buf.serialize(id)?;
                buf.serialize(title)?;
            },

            Chat::ChannelForbidden {
                flags: _,
                ref broadcast,
                ref megagroup,
                ref id,
                ref access_hash,
                ref title,
                ref until_date,
            } => {
                let mut ser_flags: u32 = 0;

                if broadcast {
                    ser_flags |= 1 << 5;
                }

                if megagroup {
                    ser_flags |= 1 << 8;
                }

                if until_date.is_some() {
                    ser_flags |= 1 << 16;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
                buf.serialize(title)?;

                if let Some(ref value) = until_date {
                    buf.serialize(value)?;
                }

            },
        }

        Ok(())
    }
}

impl Serializable for MessagesChatFull {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe5d7d19c_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.full_chat.as_ref())?;
        buf.serialize(&self.chats)?;
        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for PrivacyRule {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             PrivacyRule::ValueAllowContacts { .. } => buf.serialize(&0xfffe1bac_u32)?,
             PrivacyRule::ValueDisallowUsers { .. } => buf.serialize(&0x0c7f49b7_u32)?,
             PrivacyRule::ValueDisallowContacts { .. } => buf.serialize(&0xf888fa1a_u32)?,
             PrivacyRule::ValueAllowUsers { .. } => buf.serialize(&0x4d5bbe0c_u32)?,
             PrivacyRule::ValueAllowAll { .. } => buf.serialize(&0x65427b82_u32)?,
             PrivacyRule::ValueDisallowAll { .. } => buf.serialize(&0x8b73e763_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            PrivacyRule::ValueAllowContacts {
            } => {
            },

            PrivacyRule::ValueDisallowUsers {
                ref users,
            } => {
                buf.serialize(users)?;
            },

            PrivacyRule::ValueDisallowContacts {
            } => {
            },

            PrivacyRule::ValueAllowUsers {
                ref users,
            } => {
                buf.serialize(users)?;
            },

            PrivacyRule::ValueAllowAll {
            } => {
            },

            PrivacyRule::ValueDisallowAll {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for AuthAuthorization {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xcd050916_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.tmp_sessions.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;

        if let Some(ref value) = self.tmp_sessions {
            buf.serialize(value)?;
        }

        buf.serialize(self.user.as_ref())?;

        Ok(())
    }
 }

impl Serializable for InputContact {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputContact::Phone { .. } => buf.serialize(&0xf392b7f4_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputContact::Phone {
                ref client_id,
                ref phone,
                ref first_name,
                ref last_name,
            } => {
                buf.serialize(client_id)?;
                buf.serialize(phone)?;
                buf.serialize(first_name)?;
                buf.serialize(last_name)?;
            },
        }

        Ok(())
    }
}

impl Serializable for InputNotifyPeer {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputNotifyPeer::Users { .. } => buf.serialize(&0x193b4417_u32)?,
             InputNotifyPeer::Chats { .. } => buf.serialize(&0x4a95e84e_u32)?,
             InputNotifyPeer::Peer { .. } => buf.serialize(&0xb8bc5b0c_u32)?,
             InputNotifyPeer::All { .. } => buf.serialize(&0xa429b886_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputNotifyPeer::Users {
            } => {
            },

            InputNotifyPeer::Chats {
            } => {
            },

            InputNotifyPeer::Peer {
                ref peer,
            } => {
                buf.serialize(peer.as_ref())?;
            },

            InputNotifyPeer::All {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for BotCommand {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc27ac8c7_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.command)?;
        buf.serialize(&self.description)?;

        Ok(())
    }
 }

impl Serializable for MessageMedia {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessageMedia::Invoice { .. } => buf.serialize(&0x84551347_u32)?,
             MessageMedia::Contact { .. } => buf.serialize(&0x5e7d2f39_u32)?,
             MessageMedia::Empty { .. } => buf.serialize(&0x3ded6320_u32)?,
             MessageMedia::Document { .. } => buf.serialize(&0x7c4414d3_u32)?,
             MessageMedia::Venue { .. } => buf.serialize(&0x2ec0533f_u32)?,
             MessageMedia::Photo { .. } => buf.serialize(&0xb5223b0f_u32)?,
             MessageMedia::WebPage { .. } => buf.serialize(&0xa32dd600_u32)?,
             MessageMedia::GeoLive { .. } => buf.serialize(&0x7c3c2609_u32)?,
             MessageMedia::Unsupported { .. } => buf.serialize(&0x9f84f49e_u32)?,
             MessageMedia::Geo { .. } => buf.serialize(&0x56e0d474_u32)?,
             MessageMedia::Game { .. } => buf.serialize(&0xfdb19008_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessageMedia::Invoice {
                flags: _,
                ref shipping_address_requested,
                ref test,
                ref title,
                ref description,
                ref photo,
                ref receipt_msg_id,
                ref currency,
                ref total_amount,
                ref start_param,
            } => {
                let mut ser_flags: u32 = 0;

                if shipping_address_requested {
                    ser_flags |= 1 << 1;
                }

                if test {
                    ser_flags |= 1 << 3;
                }

                if photo.is_some() {
                    ser_flags |= 1 << 0;
                }

                if receipt_msg_id.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(title)?;
                buf.serialize(description)?;

                if let Some(ref value) = photo {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = receipt_msg_id {
                    buf.serialize(value)?;
                }

                buf.serialize(currency)?;
                buf.serialize(total_amount)?;
                buf.serialize(start_param)?;
            },

            MessageMedia::Contact {
                ref phone_number,
                ref first_name,
                ref last_name,
                ref user_id,
            } => {
                buf.serialize(phone_number)?;
                buf.serialize(first_name)?;
                buf.serialize(last_name)?;
                buf.serialize(user_id)?;
            },

            MessageMedia::Empty {
            } => {
            },

            MessageMedia::Document {
                flags: _,
                ref document,
                ref caption,
                ref ttl_seconds,
            } => {
                let mut ser_flags: u32 = 0;

                if document.is_some() {
                    ser_flags |= 1 << 0;
                }

                if caption.is_some() {
                    ser_flags |= 1 << 1;
                }

                if ttl_seconds.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;

                if let Some(ref value) = document {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = caption {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = ttl_seconds {
                    buf.serialize(value)?;
                }

            },

            MessageMedia::Venue {
                ref geo,
                ref title,
                ref address,
                ref provider,
                ref venue_id,
                ref venue_type,
            } => {
                buf.serialize(geo.as_ref())?;
                buf.serialize(title)?;
                buf.serialize(address)?;
                buf.serialize(provider)?;
                buf.serialize(venue_id)?;
                buf.serialize(venue_type)?;
            },

            MessageMedia::Photo {
                flags: _,
                ref photo,
                ref caption,
                ref ttl_seconds,
            } => {
                let mut ser_flags: u32 = 0;

                if photo.is_some() {
                    ser_flags |= 1 << 0;
                }

                if caption.is_some() {
                    ser_flags |= 1 << 1;
                }

                if ttl_seconds.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;

                if let Some(ref value) = photo {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = caption {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = ttl_seconds {
                    buf.serialize(value)?;
                }

            },

            MessageMedia::WebPage {
                ref webpage,
            } => {
                buf.serialize(webpage.as_ref())?;
            },

            MessageMedia::GeoLive {
                ref geo,
                ref period,
            } => {
                buf.serialize(geo.as_ref())?;
                buf.serialize(period)?;
            },

            MessageMedia::Unsupported {
            } => {
            },

            MessageMedia::Geo {
                ref geo,
            } => {
                buf.serialize(geo.as_ref())?;
            },

            MessageMedia::Game {
                ref game,
            } => {
                buf.serialize(game.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for InputDialogPeer {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputDialogPeer::Peer { .. } => buf.serialize(&0xfcaafeb7_u32)?,
             InputDialogPeer::Feed { .. } => buf.serialize(&0x2c38b8cf_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputDialogPeer::Peer {
                ref peer,
            } => {
                buf.serialize(peer.as_ref())?;
            },

            InputDialogPeer::Feed {
                ref feed_id,
            } => {
                buf.serialize(feed_id)?;
            },
        }

        Ok(())
    }
}

impl Serializable for ClientDhInnerData {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x6643b654_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.nonce)?;
        buf.serialize(&self.server_nonce)?;
        buf.serialize(&self.retry_id)?;
        buf.serialize(&self.g_b)?;

        Ok(())
    }
 }

impl Serializable for Peer {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             Peer::Channel { .. } => buf.serialize(&0xbddde532_u32)?,
             Peer::User { .. } => buf.serialize(&0x9db1bc6d_u32)?,
             Peer::Chat { .. } => buf.serialize(&0xbad0e5bb_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            Peer::Channel {
                ref channel_id,
            } => {
                buf.serialize(channel_id)?;
            },

            Peer::User {
                ref user_id,
            } => {
                buf.serialize(user_id)?;
            },

            Peer::Chat {
                ref chat_id,
            } => {
                buf.serialize(chat_id)?;
            },
        }

        Ok(())
    }
}

impl Serializable for PaymentsPaymentForm {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3f56aea3_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.can_save_credentials {
            ser_flags |= 1 << 2;
        }

        if self.password_missing {
            ser_flags |= 1 << 3;
        }

        if self.native_provider.is_some() {
            ser_flags |= 1 << 4;
        }

        if self.native_params.is_some() {
            ser_flags |= 1 << 4;
        }

        if self.saved_info.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.saved_credentials.is_some() {
            ser_flags |= 1 << 1;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.bot_id)?;
        buf.serialize(self.invoice.as_ref())?;
        buf.serialize(&self.provider_id)?;
        buf.serialize(&self.url)?;

        if let Some(ref value) = self.native_provider {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.native_params {
            buf.serialize(value.as_ref())?;
        }


        if let Some(ref value) = self.saved_info {
            buf.serialize(value.as_ref())?;
        }


        if let Some(ref value) = self.saved_credentials {
            buf.serialize(value.as_ref())?;
        }

        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for AccountDaysTtl {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xb8d0afdf_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.days)?;

        Ok(())
    }
 }

impl Serializable for AccountTmpPassword {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xdb64fd34_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.tmp_password)?;
        buf.serialize(&self.valid_until)?;

        Ok(())
    }
 }

impl Serializable for Message {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             Message::Message { .. } => buf.serialize(&0x44f9b43d_u32)?,
             Message::Empty { .. } => buf.serialize(&0x83e5de54_u32)?,
             Message::Service { .. } => buf.serialize(&0x9e19a1f6_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            Message::Message {
                flags: _,
                ref out,
                ref mentioned,
                ref media_unread,
                ref silent,
                ref post,
                ref id,
                ref from_id,
                ref to_id,
                ref fwd_from,
                ref via_bot_id,
                ref reply_to_msg_id,
                ref date,
                ref message,
                ref media,
                ref reply_markup,
                ref entities,
                ref views,
                ref edit_date,
                ref post_author,
                ref grouped_id,
            } => {
                let mut ser_flags: u32 = 0;

                if out {
                    ser_flags |= 1 << 1;
                }

                if mentioned {
                    ser_flags |= 1 << 4;
                }

                if media_unread {
                    ser_flags |= 1 << 5;
                }

                if silent {
                    ser_flags |= 1 << 13;
                }

                if post {
                    ser_flags |= 1 << 14;
                }

                if from_id.is_some() {
                    ser_flags |= 1 << 8;
                }

                if fwd_from.is_some() {
                    ser_flags |= 1 << 2;
                }

                if via_bot_id.is_some() {
                    ser_flags |= 1 << 11;
                }

                if reply_to_msg_id.is_some() {
                    ser_flags |= 1 << 3;
                }

                if media.is_some() {
                    ser_flags |= 1 << 9;
                }

                if reply_markup.is_some() {
                    ser_flags |= 1 << 6;
                }

                if entities.is_some() {
                    ser_flags |= 1 << 7;
                }

                if views.is_some() {
                    ser_flags |= 1 << 10;
                }

                if edit_date.is_some() {
                    ser_flags |= 1 << 15;
                }

                if post_author.is_some() {
                    ser_flags |= 1 << 16;
                }

                if grouped_id.is_some() {
                    ser_flags |= 1 << 17;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;

                if let Some(ref value) = from_id {
                    buf.serialize(value)?;
                }

                buf.serialize(to_id.as_ref())?;

                if let Some(ref value) = fwd_from {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = via_bot_id {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = reply_to_msg_id {
                    buf.serialize(value)?;
                }

                buf.serialize(date)?;
                buf.serialize(message)?;

                if let Some(ref value) = media {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = reply_markup {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = entities {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = views {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = edit_date {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = post_author {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = grouped_id {
                    buf.serialize(value)?;
                }

            },

            Message::Empty {
                ref id,
            } => {
                buf.serialize(id)?;
            },

            Message::Service {
                flags: _,
                ref out,
                ref mentioned,
                ref media_unread,
                ref silent,
                ref post,
                ref id,
                ref from_id,
                ref to_id,
                ref reply_to_msg_id,
                ref date,
                ref action,
            } => {
                let mut ser_flags: u32 = 0;

                if out {
                    ser_flags |= 1 << 1;
                }

                if mentioned {
                    ser_flags |= 1 << 4;
                }

                if media_unread {
                    ser_flags |= 1 << 5;
                }

                if silent {
                    ser_flags |= 1 << 13;
                }

                if post {
                    ser_flags |= 1 << 14;
                }

                if from_id.is_some() {
                    ser_flags |= 1 << 8;
                }

                if reply_to_msg_id.is_some() {
                    ser_flags |= 1 << 3;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;

                if let Some(ref value) = from_id {
                    buf.serialize(value)?;
                }

                buf.serialize(to_id.as_ref())?;

                if let Some(ref value) = reply_to_msg_id {
                    buf.serialize(value)?;
                }

                buf.serialize(date)?;
                buf.serialize(action.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for UpdatesState {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xa56c2a3e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.pts)?;
        buf.serialize(&self.qts)?;
        buf.serialize(&self.date)?;
        buf.serialize(&self.seq)?;
        buf.serialize(&self.unread_count)?;

        Ok(())
    }
 }

impl Serializable for AuthPasswordRecovery {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x137948a5_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.email_pattern)?;

        Ok(())
    }
 }

impl Serializable for InputStickerSet {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputStickerSet::Id { .. } => buf.serialize(&0x9de7a269_u32)?,
             InputStickerSet::ShortName { .. } => buf.serialize(&0x861cc8a0_u32)?,
             InputStickerSet::Empty { .. } => buf.serialize(&0xffb62b95_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputStickerSet::Id {
                ref id,
                ref access_hash,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
            },

            InputStickerSet::ShortName {
                ref short_name,
            } => {
                buf.serialize(short_name)?;
            },

            InputStickerSet::Empty {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for MessageEntity {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessageEntity::Code { .. } => buf.serialize(&0x28a20571_u32)?,
             MessageEntity::Unknown { .. } => buf.serialize(&0xbb92ba95_u32)?,
             MessageEntity::Mention { .. } => buf.serialize(&0xfa04579d_u32)?,
             MessageEntity::Hashtag { .. } => buf.serialize(&0x6f635b0d_u32)?,
             MessageEntity::MentionName { .. } => buf.serialize(&0x352dca58_u32)?,
             MessageEntity::Email { .. } => buf.serialize(&0x64e475c2_u32)?,
             MessageEntity::Bold { .. } => buf.serialize(&0xbd610bc9_u32)?,
             MessageEntity::BotCommand { .. } => buf.serialize(&0x6cef8ac7_u32)?,
             MessageEntity::TextUrl { .. } => buf.serialize(&0x76a6d327_u32)?,
             MessageEntity::Italic { .. } => buf.serialize(&0x826f8b60_u32)?,
             MessageEntity::Pre { .. } => buf.serialize(&0x73924be0_u32)?,
             MessageEntity::InputMentionName { .. } => buf.serialize(&0x208e68c9_u32)?,
             MessageEntity::Url { .. } => buf.serialize(&0x6ed02538_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessageEntity::Code {
                ref offset,
                ref length,
            } => {
                buf.serialize(offset)?;
                buf.serialize(length)?;
            },

            MessageEntity::Unknown {
                ref offset,
                ref length,
            } => {
                buf.serialize(offset)?;
                buf.serialize(length)?;
            },

            MessageEntity::Mention {
                ref offset,
                ref length,
            } => {
                buf.serialize(offset)?;
                buf.serialize(length)?;
            },

            MessageEntity::Hashtag {
                ref offset,
                ref length,
            } => {
                buf.serialize(offset)?;
                buf.serialize(length)?;
            },

            MessageEntity::MentionName {
                ref offset,
                ref length,
                ref user_id,
            } => {
                buf.serialize(offset)?;
                buf.serialize(length)?;
                buf.serialize(user_id)?;
            },

            MessageEntity::Email {
                ref offset,
                ref length,
            } => {
                buf.serialize(offset)?;
                buf.serialize(length)?;
            },

            MessageEntity::Bold {
                ref offset,
                ref length,
            } => {
                buf.serialize(offset)?;
                buf.serialize(length)?;
            },

            MessageEntity::BotCommand {
                ref offset,
                ref length,
            } => {
                buf.serialize(offset)?;
                buf.serialize(length)?;
            },

            MessageEntity::TextUrl {
                ref offset,
                ref length,
                ref url,
            } => {
                buf.serialize(offset)?;
                buf.serialize(length)?;
                buf.serialize(url)?;
            },

            MessageEntity::Italic {
                ref offset,
                ref length,
            } => {
                buf.serialize(offset)?;
                buf.serialize(length)?;
            },

            MessageEntity::Pre {
                ref offset,
                ref length,
                ref language,
            } => {
                buf.serialize(offset)?;
                buf.serialize(length)?;
                buf.serialize(language)?;
            },

            MessageEntity::InputMentionName {
                ref offset,
                ref length,
                ref user_id,
            } => {
                buf.serialize(offset)?;
                buf.serialize(length)?;
                buf.serialize(user_id.as_ref())?;
            },

            MessageEntity::Url {
                ref offset,
                ref length,
            } => {
                buf.serialize(offset)?;
                buf.serialize(length)?;
            },
        }

        Ok(())
    }
}

impl Serializable for MsgsStateInfo {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x04deb57d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.req_msg_id)?;
        buf.serialize(&self.info)?;

        Ok(())
    }
 }

impl Serializable for DocumentAttribute {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             DocumentAttribute::Sticker { .. } => buf.serialize(&0x6319d612_u32)?,
             DocumentAttribute::Video { .. } => buf.serialize(&0x0ef02ce6_u32)?,
             DocumentAttribute::ImageSize { .. } => buf.serialize(&0x6c37c15c_u32)?,
             DocumentAttribute::Filename { .. } => buf.serialize(&0x15590068_u32)?,
             DocumentAttribute::HasStickers { .. } => buf.serialize(&0x9801d2f7_u32)?,
             DocumentAttribute::Animated { .. } => buf.serialize(&0x11b58939_u32)?,
             DocumentAttribute::Audio { .. } => buf.serialize(&0x9852f9c6_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            DocumentAttribute::Sticker {
                flags: _,
                ref mask,
                ref alt,
                ref stickerset,
                ref mask_coords,
            } => {
                let mut ser_flags: u32 = 0;

                if mask {
                    ser_flags |= 1 << 1;
                }

                if mask_coords.is_some() {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(alt)?;
                buf.serialize(stickerset.as_ref())?;

                if let Some(ref value) = mask_coords {
                    buf.serialize(value.as_ref())?;
                }

            },

            DocumentAttribute::Video {
                flags: _,
                ref round_message,
                ref duration,
                ref w,
                ref h,
            } => {
                let mut ser_flags: u32 = 0;

                if round_message {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(duration)?;
                buf.serialize(w)?;
                buf.serialize(h)?;
            },

            DocumentAttribute::ImageSize {
                ref w,
                ref h,
            } => {
                buf.serialize(w)?;
                buf.serialize(h)?;
            },

            DocumentAttribute::Filename {
                ref file_name,
            } => {
                buf.serialize(file_name)?;
            },

            DocumentAttribute::HasStickers {
            } => {
            },

            DocumentAttribute::Animated {
            } => {
            },

            DocumentAttribute::Audio {
                flags: _,
                ref voice,
                ref duration,
                ref title,
                ref performer,
                ref waveform,
            } => {
                let mut ser_flags: u32 = 0;

                if voice {
                    ser_flags |= 1 << 10;
                }

                if title.is_some() {
                    ser_flags |= 1 << 0;
                }

                if performer.is_some() {
                    ser_flags |= 1 << 1;
                }

                if waveform.is_some() {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(duration)?;

                if let Some(ref value) = title {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = performer {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = waveform {
                    buf.serialize(value)?;
                }

            },
        }

        Ok(())
    }
}

impl Serializable for HelpRecentMeUrls {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x0e0310d7_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.urls)?;
        buf.serialize(&self.chats)?;
        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for ChannelsChannelParticipant {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xd0d9b163_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.participant.as_ref())?;
        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for AuthCheckedPhone {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x811ea28e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.phone_registered)?;

        Ok(())
    }
 }

impl Serializable for Game {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xbdf9653b_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.document.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.id)?;
        buf.serialize(&self.access_hash)?;
        buf.serialize(&self.short_name)?;
        buf.serialize(&self.title)?;
        buf.serialize(&self.description)?;
        buf.serialize(self.photo.as_ref())?;

        if let Some(ref value) = self.document {
            buf.serialize(value.as_ref())?;
        }


        Ok(())
    }
 }

impl Serializable for DcOption {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x05d8c6cc_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.ipv6 {
            ser_flags |= 1 << 0;
        }

        if self.media_only {
            ser_flags |= 1 << 1;
        }

        if self.tcpo_only {
            ser_flags |= 1 << 2;
        }

        if self.cdn {
            ser_flags |= 1 << 3;
        }

        if self.static_ {
            ser_flags |= 1 << 4;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.id)?;
        buf.serialize(&self.ip_address)?;
        buf.serialize(&self.port)?;

        Ok(())
    }
 }

impl Serializable for PaymentsPaymentResult {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             PaymentsPaymentResult::VerficationNeeded { .. } => buf.serialize(&0x6b56b921_u32)?,
             PaymentsPaymentResult::Result { .. } => buf.serialize(&0x4e5f810d_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            PaymentsPaymentResult::VerficationNeeded {
                ref url,
            } => {
                buf.serialize(url)?;
            },

            PaymentsPaymentResult::Result {
                ref updates,
            } => {
                buf.serialize(updates.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for MessagesArchivedStickers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x4fcba9c8_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.count)?;
        buf.serialize(&self.sets)?;

        Ok(())
    }
 }

impl Serializable for UpdatesChannelDifference {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             UpdatesChannelDifference::Difference { .. } => buf.serialize(&0x2064674e_u32)?,
             UpdatesChannelDifference::Empty { .. } => buf.serialize(&0x3e11affb_u32)?,
             UpdatesChannelDifference::TooLong { .. } => buf.serialize(&0x6a9d7b35_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            UpdatesChannelDifference::Difference {
                flags: _,
                ref final_,
                ref pts,
                ref timeout,
                ref new_messages,
                ref other_updates,
                ref chats,
                ref users,
            } => {
                let mut ser_flags: u32 = 0;

                if final_ {
                    ser_flags |= 1 << 0;
                }

                if timeout.is_some() {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(pts)?;

                if let Some(ref value) = timeout {
                    buf.serialize(value)?;
                }

                buf.serialize(new_messages)?;
                buf.serialize(other_updates)?;
                buf.serialize(chats)?;
                buf.serialize(users)?;
            },

            UpdatesChannelDifference::Empty {
                flags: _,
                ref final_,
                ref pts,
                ref timeout,
            } => {
                let mut ser_flags: u32 = 0;

                if final_ {
                    ser_flags |= 1 << 0;
                }

                if timeout.is_some() {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(pts)?;

                if let Some(ref value) = timeout {
                    buf.serialize(value)?;
                }

            },

            UpdatesChannelDifference::TooLong {
                flags: _,
                ref final_,
                ref pts,
                ref timeout,
                ref top_message,
                ref read_inbox_max_id,
                ref read_outbox_max_id,
                ref unread_count,
                ref unread_mentions_count,
                ref messages,
                ref chats,
                ref users,
            } => {
                let mut ser_flags: u32 = 0;

                if final_ {
                    ser_flags |= 1 << 0;
                }

                if timeout.is_some() {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(pts)?;

                if let Some(ref value) = timeout {
                    buf.serialize(value)?;
                }

                buf.serialize(top_message)?;
                buf.serialize(read_inbox_max_id)?;
                buf.serialize(read_outbox_max_id)?;
                buf.serialize(unread_count)?;
                buf.serialize(unread_mentions_count)?;
                buf.serialize(messages)?;
                buf.serialize(chats)?;
                buf.serialize(users)?;
            },
        }

        Ok(())
    }
}

impl Serializable for Authorization {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x7bf2e6f6_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.hash)?;
        let mut ser_flags: u32 = 0;

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.device_model)?;
        buf.serialize(&self.platform)?;
        buf.serialize(&self.system_version)?;
        buf.serialize(&self.api_id)?;
        buf.serialize(&self.app_name)?;
        buf.serialize(&self.app_version)?;
        buf.serialize(&self.date_created)?;
        buf.serialize(&self.date_active)?;
        buf.serialize(&self.ip)?;
        buf.serialize(&self.country)?;
        buf.serialize(&self.region)?;

        Ok(())
    }
 }

impl Serializable for NearestDc {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x8e1a1775_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.country)?;
        buf.serialize(&self.this_dc)?;
        buf.serialize(&self.nearest_dc)?;

        Ok(())
    }
 }

impl Serializable for AccountPasswordInputSettings {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x86916deb_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.new_salt.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.new_password_hash.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.hint.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.email.is_some() {
            ser_flags |= 1 << 1;
        }

        buf.serialize(&ser_flags)?;

        if let Some(ref value) = self.new_salt {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.new_password_hash {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.hint {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.email {
            buf.serialize(value)?;
        }


        Ok(())
    }
 }

impl Serializable for WebDocument {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc61acbd8_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.url)?;
        buf.serialize(&self.access_hash)?;
        buf.serialize(&self.size)?;
        buf.serialize(&self.mime_type)?;
        buf.serialize(&self.attributes)?;
        buf.serialize(&self.dc_id)?;

        Ok(())
    }
 }

impl Serializable for FutureSalts {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xae500895_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.req_msg_id)?;
        buf.serialize(&self.now)?;
        buf.serialize(&self.salts)?;

        Ok(())
    }
 }

impl Serializable for Page {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             Page::Full { .. } => buf.serialize(&0x556ec7aa_u32)?,
             Page::Part { .. } => buf.serialize(&0x8e3f9ebe_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            Page::Full {
                ref blocks,
                ref photos,
                ref documents,
            } => {
                buf.serialize(blocks)?;
                buf.serialize(photos)?;
                buf.serialize(documents)?;
            },

            Page::Part {
                ref blocks,
                ref photos,
                ref documents,
            } => {
                buf.serialize(blocks)?;
                buf.serialize(photos)?;
                buf.serialize(documents)?;
            },
        }

        Ok(())
    }
}

impl Serializable for PhoneConnection {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x9d4c17c0_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;
        buf.serialize(&self.ip)?;
        buf.serialize(&self.ipv6)?;
        buf.serialize(&self.port)?;
        buf.serialize(&self.peer_tag)?;

        Ok(())
    }
 }

impl Serializable for HelpInviteText {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x18cb9f78_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.message)?;

        Ok(())
    }
 }

impl Serializable for UpdatesDifference {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             UpdatesDifference::Slice { .. } => buf.serialize(&0xa8fb1981_u32)?,
             UpdatesDifference::TooLong { .. } => buf.serialize(&0x4afe8f6d_u32)?,
             UpdatesDifference::Empty { .. } => buf.serialize(&0x5d75a138_u32)?,
             UpdatesDifference::Difference { .. } => buf.serialize(&0x00f49ca0_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            UpdatesDifference::Slice {
                ref new_messages,
                ref new_encrypted_messages,
                ref other_updates,
                ref chats,
                ref users,
                ref intermediate_state,
            } => {
                buf.serialize(new_messages)?;
                buf.serialize(new_encrypted_messages)?;
                buf.serialize(other_updates)?;
                buf.serialize(chats)?;
                buf.serialize(users)?;
                buf.serialize(intermediate_state.as_ref())?;
            },

            UpdatesDifference::TooLong {
                ref pts,
            } => {
                buf.serialize(pts)?;
            },

            UpdatesDifference::Empty {
                ref date,
                ref seq,
            } => {
                buf.serialize(date)?;
                buf.serialize(seq)?;
            },

            UpdatesDifference::Difference {
                ref new_messages,
                ref new_encrypted_messages,
                ref other_updates,
                ref chats,
                ref users,
                ref state,
            } => {
                buf.serialize(new_messages)?;
                buf.serialize(new_encrypted_messages)?;
                buf.serialize(other_updates)?;
                buf.serialize(chats)?;
                buf.serialize(users)?;
                buf.serialize(state.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for CdnConfig {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x5725e40a_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.public_keys)?;

        Ok(())
    }
 }

impl Serializable for MessagesMessageEditData {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x26b5dde6_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.caption {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;

        Ok(())
    }
 }

impl Serializable for MessagesBotCallbackAnswer {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x36585ea4_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.alert {
            ser_flags |= 1 << 1;
        }

        if self.has_url {
            ser_flags |= 1 << 3;
        }

        if self.native_ui {
            ser_flags |= 1 << 4;
        }

        if self.message.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.url.is_some() {
            ser_flags |= 1 << 2;
        }

        buf.serialize(&ser_flags)?;

        if let Some(ref value) = self.message {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.url {
            buf.serialize(value)?;
        }

        buf.serialize(&self.cache_time)?;

        Ok(())
    }
 }

impl Serializable for CdnPublicKey {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc982eaba_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.dc_id)?;
        buf.serialize(&self.public_key)?;

        Ok(())
    }
 }

impl Serializable for StickerSet {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xcd303b41_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.installed {
            ser_flags |= 1 << 0;
        }

        if self.archived {
            ser_flags |= 1 << 1;
        }

        if self.official {
            ser_flags |= 1 << 2;
        }

        if self.masks {
            ser_flags |= 1 << 3;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.id)?;
        buf.serialize(&self.access_hash)?;
        buf.serialize(&self.title)?;
        buf.serialize(&self.short_name)?;
        buf.serialize(&self.count)?;
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for WallPaper {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             WallPaper::Solid { .. } => buf.serialize(&0x63117f24_u32)?,
             WallPaper::Paper { .. } => buf.serialize(&0xccb03657_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            WallPaper::Solid {
                ref id,
                ref title,
                ref bg_color,
                ref color,
            } => {
                buf.serialize(id)?;
                buf.serialize(title)?;
                buf.serialize(bg_color)?;
                buf.serialize(color)?;
            },

            WallPaper::Paper {
                ref id,
                ref title,
                ref sizes,
                ref color,
            } => {
                buf.serialize(id)?;
                buf.serialize(title)?;
                buf.serialize(sizes)?;
                buf.serialize(color)?;
            },
        }

        Ok(())
    }
}

impl Serializable for User {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             User::User { .. } => buf.serialize(&0x2e13f4c3_u32)?,
             User::Empty { .. } => buf.serialize(&0x200250ba_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            User::User {
                flags: _,
                ref self_,
                ref contact,
                ref mutual_contact,
                ref deleted,
                ref bot,
                ref bot_chat_history,
                ref bot_nochats,
                ref verified,
                ref restricted,
                ref min,
                ref bot_inline_geo,
                ref id,
                ref access_hash,
                ref first_name,
                ref last_name,
                ref username,
                ref phone,
                ref photo,
                ref status,
                ref bot_info_version,
                ref restriction_reason,
                ref bot_inline_placeholder,
                ref lang_code,
            } => {
                let mut ser_flags: u32 = 0;

                if self_ {
                    ser_flags |= 1 << 10;
                }

                if contact {
                    ser_flags |= 1 << 11;
                }

                if mutual_contact {
                    ser_flags |= 1 << 12;
                }

                if deleted {
                    ser_flags |= 1 << 13;
                }

                if bot {
                    ser_flags |= 1 << 14;
                }

                if bot_chat_history {
                    ser_flags |= 1 << 15;
                }

                if bot_nochats {
                    ser_flags |= 1 << 16;
                }

                if verified {
                    ser_flags |= 1 << 17;
                }

                if restricted {
                    ser_flags |= 1 << 18;
                }

                if min {
                    ser_flags |= 1 << 20;
                }

                if bot_inline_geo {
                    ser_flags |= 1 << 21;
                }

                if access_hash.is_some() {
                    ser_flags |= 1 << 0;
                }

                if first_name.is_some() {
                    ser_flags |= 1 << 1;
                }

                if last_name.is_some() {
                    ser_flags |= 1 << 2;
                }

                if username.is_some() {
                    ser_flags |= 1 << 3;
                }

                if phone.is_some() {
                    ser_flags |= 1 << 4;
                }

                if photo.is_some() {
                    ser_flags |= 1 << 5;
                }

                if status.is_some() {
                    ser_flags |= 1 << 6;
                }

                if bot_info_version.is_some() {
                    ser_flags |= 1 << 14;
                }

                if restriction_reason.is_some() {
                    ser_flags |= 1 << 18;
                }

                if bot_inline_placeholder.is_some() {
                    ser_flags |= 1 << 19;
                }

                if lang_code.is_some() {
                    ser_flags |= 1 << 22;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;

                if let Some(ref value) = access_hash {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = first_name {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = last_name {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = username {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = phone {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = photo {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = status {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = bot_info_version {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = restriction_reason {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = bot_inline_placeholder {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = lang_code {
                    buf.serialize(value)?;
                }

            },

            User::Empty {
                ref id,
            } => {
                buf.serialize(id)?;
            },
        }

        Ok(())
    }
}

impl Serializable for InputEncryptedChat {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf141b5e1_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.chat_id)?;
        buf.serialize(&self.access_hash)?;

        Ok(())
    }
 }

impl Serializable for MessageRange {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x0ae30253_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.min_id)?;
        buf.serialize(&self.max_id)?;

        Ok(())
    }
 }

impl Serializable for MessagesAffectedHistory {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xb45c69d1_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.pts)?;
        buf.serialize(&self.pts_count)?;
        buf.serialize(&self.offset)?;

        Ok(())
    }
 }

impl Serializable for ContactsTopPeers {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ContactsTopPeers::Peers { .. } => buf.serialize(&0x70b772a8_u32)?,
             ContactsTopPeers::NotModified { .. } => buf.serialize(&0xde266ef5_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ContactsTopPeers::Peers {
                ref categories,
                ref chats,
                ref users,
            } => {
                buf.serialize(categories)?;
                buf.serialize(chats)?;
                buf.serialize(users)?;
            },

            ContactsTopPeers::NotModified {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for AccountPrivacyRules {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x554abb6f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.rules)?;
        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for InputFileLocation {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputFileLocation::Encrypted { .. } => buf.serialize(&0xf5235d55_u32)?,
             InputFileLocation::Document { .. } => buf.serialize(&0x430f0724_u32)?,
             InputFileLocation::Location { .. } => buf.serialize(&0x14637196_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputFileLocation::Encrypted {
                ref id,
                ref access_hash,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
            },

            InputFileLocation::Document {
                ref id,
                ref access_hash,
                ref version,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
                buf.serialize(version)?;
            },

            InputFileLocation::Location {
                ref volume_id,
                ref local_id,
                ref secret,
            } => {
                buf.serialize(volume_id)?;
                buf.serialize(local_id)?;
                buf.serialize(secret)?;
            },
        }

        Ok(())
    }
}

impl Serializable for PaymentsValidatedRequestedInfo {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xd1451883_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.id.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.shipping_options.is_some() {
            ser_flags |= 1 << 1;
        }

        buf.serialize(&ser_flags)?;

        if let Some(ref value) = self.id {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.shipping_options {
            buf.serialize(value)?;
        }


        Ok(())
    }
 }

impl Serializable for ResPq {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x05162463_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.nonce)?;
        buf.serialize(&self.server_nonce)?;
        buf.serialize(&self.pq)?;
        buf.serialize(&self.server_public_key_fingerprints)?;

        Ok(())
    }
 }

impl Serializable for HighScore {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x58fffcd0_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.pos)?;
        buf.serialize(&self.user_id)?;
        buf.serialize(&self.score)?;

        Ok(())
    }
 }

impl Serializable for UploadFile {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             UploadFile::CdnRedirect { .. } => buf.serialize(&0xea52fe5a_u32)?,
             UploadFile::File { .. } => buf.serialize(&0x096a18d5_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            UploadFile::CdnRedirect {
                ref dc_id,
                ref file_token,
                ref encryption_key,
                ref encryption_iv,
                ref cdn_file_hashes,
            } => {
                buf.serialize(dc_id)?;
                buf.serialize(file_token)?;
                buf.serialize(encryption_key)?;
                buf.serialize(encryption_iv)?;
                buf.serialize(cdn_file_hashes)?;
            },

            UploadFile::File {
                ref type_,
                ref mtime,
                ref bytes,
            } => {
                buf.serialize(type_.as_ref())?;
                buf.serialize(mtime)?;
                buf.serialize(bytes)?;
            },
        }

        Ok(())
    }
}

impl Serializable for PaymentSavedCredentials {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             PaymentSavedCredentials::Card { .. } => buf.serialize(&0xcdc27a1f_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            PaymentSavedCredentials::Card {
                ref id,
                ref title,
            } => {
                buf.serialize(id)?;
                buf.serialize(title)?;
            },
        }

        Ok(())
    }
}

impl Serializable for MessagesBotResults {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x947ca848_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.gallery {
            ser_flags |= 1 << 0;
        }

        if self.next_offset.is_some() {
            ser_flags |= 1 << 1;
        }

        if self.switch_pm.is_some() {
            ser_flags |= 1 << 2;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.query_id)?;

        if let Some(ref value) = self.next_offset {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.switch_pm {
            buf.serialize(value.as_ref())?;
        }

        buf.serialize(&self.results)?;
        buf.serialize(&self.cache_time)?;
        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for HelpAppUpdate {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             HelpAppUpdate::No { .. } => buf.serialize(&0xc45a6536_u32)?,
             HelpAppUpdate::Update { .. } => buf.serialize(&0x8987f311_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            HelpAppUpdate::No {
            } => {
            },

            HelpAppUpdate::Update {
                ref id,
                ref critical,
                ref url,
                ref text,
            } => {
                buf.serialize(id)?;
                buf.serialize(critical)?;
                buf.serialize(url)?;
                buf.serialize(text)?;
            },
        }

        Ok(())
    }
}

impl Serializable for InputEncryptedFile {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputEncryptedFile::BigUploaded { .. } => buf.serialize(&0x2dc173c8_u32)?,
             InputEncryptedFile::File { .. } => buf.serialize(&0x5a17b5e5_u32)?,
             InputEncryptedFile::Empty { .. } => buf.serialize(&0x1837c364_u32)?,
             InputEncryptedFile::Uploaded { .. } => buf.serialize(&0x64bd0306_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputEncryptedFile::BigUploaded {
                ref id,
                ref parts,
                ref key_fingerprint,
            } => {
                buf.serialize(id)?;
                buf.serialize(parts)?;
                buf.serialize(key_fingerprint)?;
            },

            InputEncryptedFile::File {
                ref id,
                ref access_hash,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
            },

            InputEncryptedFile::Empty {
            } => {
            },

            InputEncryptedFile::Uploaded {
                ref id,
                ref parts,
                ref md5_checksum,
                ref key_fingerprint,
            } => {
                buf.serialize(id)?;
                buf.serialize(parts)?;
                buf.serialize(md5_checksum)?;
                buf.serialize(key_fingerprint)?;
            },
        }

        Ok(())
    }
}

impl Serializable for InputPhoto {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputPhoto::Photo { .. } => buf.serialize(&0xfb95c6c4_u32)?,
             InputPhoto::Empty { .. } => buf.serialize(&0x1cd7bf0d_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputPhoto::Photo {
                ref id,
                ref access_hash,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
            },

            InputPhoto::Empty {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for Null {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x56730bcc_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for LangPackString {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             LangPackString::Pluralized { .. } => buf.serialize(&0x6c47ac9f_u32)?,
             LangPackString::String { .. } => buf.serialize(&0xcad181f6_u32)?,
             LangPackString::Deleted { .. } => buf.serialize(&0x2979eeb2_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            LangPackString::Pluralized {
                flags: _,
                ref key,
                ref zero_value,
                ref one_value,
                ref two_value,
                ref few_value,
                ref many_value,
                ref other_value,
            } => {
                let mut ser_flags: u32 = 0;

                if zero_value.is_some() {
                    ser_flags |= 1 << 0;
                }

                if one_value.is_some() {
                    ser_flags |= 1 << 1;
                }

                if two_value.is_some() {
                    ser_flags |= 1 << 2;
                }

                if few_value.is_some() {
                    ser_flags |= 1 << 3;
                }

                if many_value.is_some() {
                    ser_flags |= 1 << 4;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(key)?;

                if let Some(ref value) = zero_value {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = one_value {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = two_value {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = few_value {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = many_value {
                    buf.serialize(value)?;
                }

                buf.serialize(other_value)?;
            },

            LangPackString::String {
                ref key,
                ref value,
            } => {
                buf.serialize(key)?;
                buf.serialize(value)?;
            },

            LangPackString::Deleted {
                ref key,
            } => {
                buf.serialize(key)?;
            },
        }

        Ok(())
    }
}

impl Serializable for MessagesFoundGifs {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x450a1c0a_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.next_offset)?;
        buf.serialize(&self.results)?;

        Ok(())
    }
 }

impl Serializable for InputPeerNotifyEvents {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputPeerNotifyEvents::Empty { .. } => buf.serialize(&0xf03064d8_u32)?,
             InputPeerNotifyEvents::All { .. } => buf.serialize(&0xe86a2c74_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputPeerNotifyEvents::Empty {
            } => {
            },

            InputPeerNotifyEvents::All {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for MsgsAllInfo {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x8cc0d131_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.msg_ids)?;
        buf.serialize(&self.info)?;

        Ok(())
    }
 }

impl Serializable for InputPrivacyKey {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputPrivacyKey::ChatInvite { .. } => buf.serialize(&0xbdfb0426_u32)?,
             InputPrivacyKey::StatusTimestamp { .. } => buf.serialize(&0x4f96cb18_u32)?,
             InputPrivacyKey::PhoneCall { .. } => buf.serialize(&0xfabadc5f_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputPrivacyKey::ChatInvite {
            } => {
            },

            InputPrivacyKey::StatusTimestamp {
            } => {
            },

            InputPrivacyKey::PhoneCall {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for MessagesSentEncryptedMessage {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessagesSentEncryptedMessage::Message { .. } => buf.serialize(&0x560f8935_u32)?,
             MessagesSentEncryptedMessage::File { .. } => buf.serialize(&0x9493ff32_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessagesSentEncryptedMessage::Message {
                ref date,
            } => {
                buf.serialize(date)?;
            },

            MessagesSentEncryptedMessage::File {
                ref date,
                ref file,
            } => {
                buf.serialize(date)?;
                buf.serialize(file.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for Config {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x9c840964_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.phonecalls_enabled {
            ser_flags |= 1 << 1;
        }

        if self.default_p2p_contacts {
            ser_flags |= 1 << 3;
        }

        if self.tmp_sessions.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.suggested_lang_code.is_some() {
            ser_flags |= 1 << 2;
        }

        if self.lang_pack_version.is_some() {
            ser_flags |= 1 << 2;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.date)?;
        buf.serialize(&self.expires)?;
        buf.serialize(&self.test_mode)?;
        buf.serialize(&self.this_dc)?;
        buf.serialize(&self.dc_options)?;
        buf.serialize(&self.chat_size_max)?;
        buf.serialize(&self.megagroup_size_max)?;
        buf.serialize(&self.forwarded_count_max)?;
        buf.serialize(&self.online_update_period_ms)?;
        buf.serialize(&self.offline_blur_timeout_ms)?;
        buf.serialize(&self.offline_idle_timeout_ms)?;
        buf.serialize(&self.online_cloud_timeout_ms)?;
        buf.serialize(&self.notify_cloud_delay_ms)?;
        buf.serialize(&self.notify_default_delay_ms)?;
        buf.serialize(&self.chat_big_size)?;
        buf.serialize(&self.push_chat_period_ms)?;
        buf.serialize(&self.push_chat_limit)?;
        buf.serialize(&self.saved_gifs_limit)?;
        buf.serialize(&self.edit_time_limit)?;
        buf.serialize(&self.rating_e_decay)?;
        buf.serialize(&self.stickers_recent_limit)?;
        buf.serialize(&self.stickers_faved_limit)?;
        buf.serialize(&self.channels_read_media_period)?;

        if let Some(ref value) = self.tmp_sessions {
            buf.serialize(value)?;
        }

        buf.serialize(&self.pinned_dialogs_count_max)?;
        buf.serialize(&self.call_receive_timeout_ms)?;
        buf.serialize(&self.call_ring_timeout_ms)?;
        buf.serialize(&self.call_connect_timeout_ms)?;
        buf.serialize(&self.call_packet_timeout_ms)?;
        buf.serialize(&self.me_url_prefix)?;

        if let Some(ref value) = self.suggested_lang_code {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.lang_pack_version {
            buf.serialize(value)?;
        }

        buf.serialize(&self.disabled_features)?;

        Ok(())
    }
 }

impl Serializable for AuthSentCodeType {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             AuthSentCodeType::FlashCall { .. } => buf.serialize(&0xab03c6d9_u32)?,
             AuthSentCodeType::App { .. } => buf.serialize(&0x3dbb5986_u32)?,
             AuthSentCodeType::Call { .. } => buf.serialize(&0x5353e5a7_u32)?,
             AuthSentCodeType::Sms { .. } => buf.serialize(&0xc000bba2_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            AuthSentCodeType::FlashCall {
                ref pattern,
            } => {
                buf.serialize(pattern)?;
            },

            AuthSentCodeType::App {
                ref length,
            } => {
                buf.serialize(length)?;
            },

            AuthSentCodeType::Call {
                ref length,
            } => {
                buf.serialize(length)?;
            },

            AuthSentCodeType::Sms {
                ref length,
            } => {
                buf.serialize(length)?;
            },
        }

        Ok(())
    }
}

impl Serializable for ChannelMessagesFilter {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ChannelMessagesFilter::Empty { .. } => buf.serialize(&0x94d42ee7_u32)?,
             ChannelMessagesFilter::Filter { .. } => buf.serialize(&0xcd77d957_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ChannelMessagesFilter::Empty {
            } => {
            },

            ChannelMessagesFilter::Filter {
                flags: _,
                ref exclude_new_messages,
                ref ranges,
            } => {
                let mut ser_flags: u32 = 0;

                if exclude_new_messages {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(ranges)?;
            },
        }

        Ok(())
    }
}

impl Serializable for UserFull {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x0f220f3f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.blocked {
            ser_flags |= 1 << 0;
        }

        if self.phone_calls_available {
            ser_flags |= 1 << 4;
        }

        if self.phone_calls_private {
            ser_flags |= 1 << 5;
        }

        if self.about.is_some() {
            ser_flags |= 1 << 1;
        }

        if self.profile_photo.is_some() {
            ser_flags |= 1 << 2;
        }

        if self.bot_info.is_some() {
            ser_flags |= 1 << 3;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.user.as_ref())?;

        if let Some(ref value) = self.about {
            buf.serialize(value)?;
        }

        buf.serialize(self.link.as_ref())?;

        if let Some(ref value) = self.profile_photo {
            buf.serialize(value.as_ref())?;
        }

        buf.serialize(self.notify_settings.as_ref())?;

        if let Some(ref value) = self.bot_info {
            buf.serialize(value.as_ref())?;
        }

        buf.serialize(&self.common_chats_count)?;

        Ok(())
    }
 }

impl Serializable for RecentMeUrl {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             RecentMeUrl::Unknown { .. } => buf.serialize(&0x46e1d13d_u32)?,
             RecentMeUrl::User { .. } => buf.serialize(&0x8dbc3336_u32)?,
             RecentMeUrl::Chat { .. } => buf.serialize(&0xa01b22f9_u32)?,
             RecentMeUrl::ChatInvite { .. } => buf.serialize(&0xeb49081d_u32)?,
             RecentMeUrl::StickerSet { .. } => buf.serialize(&0xbc0a57dc_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            RecentMeUrl::Unknown {
                ref url,
            } => {
                buf.serialize(url)?;
            },

            RecentMeUrl::User {
                ref url,
                ref user_id,
            } => {
                buf.serialize(url)?;
                buf.serialize(user_id)?;
            },

            RecentMeUrl::Chat {
                ref url,
                ref chat_id,
            } => {
                buf.serialize(url)?;
                buf.serialize(chat_id)?;
            },

            RecentMeUrl::ChatInvite {
                ref url,
                ref chat_invite,
            } => {
                buf.serialize(url)?;
                buf.serialize(chat_invite.as_ref())?;
            },

            RecentMeUrl::StickerSet {
                ref url,
                ref set,
            } => {
                buf.serialize(url)?;
                buf.serialize(set.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for FileLocation {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             FileLocation::Unavailable { .. } => buf.serialize(&0x7c596b46_u32)?,
             FileLocation::Location { .. } => buf.serialize(&0x53d69076_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            FileLocation::Unavailable {
                ref volume_id,
                ref local_id,
                ref secret,
            } => {
                buf.serialize(volume_id)?;
                buf.serialize(local_id)?;
                buf.serialize(secret)?;
            },

            FileLocation::Location {
                ref dc_id,
                ref volume_id,
                ref local_id,
                ref secret,
            } => {
                buf.serialize(dc_id)?;
                buf.serialize(volume_id)?;
                buf.serialize(local_id)?;
                buf.serialize(secret)?;
            },
        }

        Ok(())
    }
}

impl Serializable for FeedPosition {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x5059dc73_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.date)?;
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for AccountPassword {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             AccountPassword::No { .. } => buf.serialize(&0x96dabc18_u32)?,
             AccountPassword::Password { .. } => buf.serialize(&0x7c18141c_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            AccountPassword::No {
                ref new_salt,
                ref email_unconfirmed_pattern,
            } => {
                buf.serialize(new_salt)?;
                buf.serialize(email_unconfirmed_pattern)?;
            },

            AccountPassword::Password {
                ref current_salt,
                ref new_salt,
                ref hint,
                ref has_recovery,
                ref email_unconfirmed_pattern,
            } => {
                buf.serialize(current_salt)?;
                buf.serialize(new_salt)?;
                buf.serialize(hint)?;
                buf.serialize(has_recovery)?;
                buf.serialize(email_unconfirmed_pattern)?;
            },
        }

        Ok(())
    }
}

impl Serializable for LangPackLanguage {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x117698f1_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.name)?;
        buf.serialize(&self.native_name)?;
        buf.serialize(&self.lang_code)?;

        Ok(())
    }
 }

impl Serializable for PostAddress {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x1e8caaeb_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.street_line1)?;
        buf.serialize(&self.street_line2)?;
        buf.serialize(&self.city)?;
        buf.serialize(&self.state)?;
        buf.serialize(&self.country_iso2)?;
        buf.serialize(&self.post_code)?;

        Ok(())
    }
 }

impl Serializable for UploadCdnFile {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             UploadCdnFile::ReuploadNeeded { .. } => buf.serialize(&0xeea8e46e_u32)?,
             UploadCdnFile::File { .. } => buf.serialize(&0xa99fca4f_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            UploadCdnFile::ReuploadNeeded {
                ref request_token,
            } => {
                buf.serialize(request_token)?;
            },

            UploadCdnFile::File {
                ref bytes,
            } => {
                buf.serialize(bytes)?;
            },
        }

        Ok(())
    }
}

impl Serializable for MaskCoords {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xaed6dbb2_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.n)?;
        buf.serialize(&self.x)?;
        buf.serialize(&self.y)?;
        buf.serialize(&self.zoom)?;

        Ok(())
    }
 }

impl Serializable for ReceivedNotifyMessage {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xa384b779_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;
        let mut ser_flags: u32 = 0;

        buf.serialize(&ser_flags)?;

        Ok(())
    }
 }

impl Serializable for PhonePhoneCall {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xec82e140_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.phone_call.as_ref())?;
        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for PaymentRequestedInfo {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x909c3f94_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.name.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.phone.is_some() {
            ser_flags |= 1 << 1;
        }

        if self.email.is_some() {
            ser_flags |= 1 << 2;
        }

        if self.shipping_address.is_some() {
            ser_flags |= 1 << 3;
        }

        buf.serialize(&ser_flags)?;

        if let Some(ref value) = self.name {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.phone {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.email {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.shipping_address {
            buf.serialize(value.as_ref())?;
        }


        Ok(())
    }
 }

impl Serializable for MessagesChats {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessagesChats::Slice { .. } => buf.serialize(&0x9cd81144_u32)?,
             MessagesChats::Chats { .. } => buf.serialize(&0x64ff9fd5_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessagesChats::Slice {
                ref count,
                ref chats,
            } => {
                buf.serialize(count)?;
                buf.serialize(chats)?;
            },

            MessagesChats::Chats {
                ref chats,
            } => {
                buf.serialize(chats)?;
            },
        }

        Ok(())
    }
}

impl Serializable for ShippingOption {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xb6213cdf_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;
        buf.serialize(&self.title)?;
        buf.serialize(&self.prices)?;

        Ok(())
    }
 }

impl Serializable for MessagesStickers {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessagesStickers::NotModified { .. } => buf.serialize(&0xf1749a22_u32)?,
             MessagesStickers::Stickers { .. } => buf.serialize(&0x8a8ecd32_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessagesStickers::NotModified {
            } => {
            },

            MessagesStickers::Stickers {
                ref hash,
                ref stickers,
            } => {
                buf.serialize(hash)?;
                buf.serialize(stickers)?;
            },
        }

        Ok(())
    }
}

impl Serializable for SetClientDhParamsAnswer {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             SetClientDhParamsAnswer::DhGenOk { .. } => buf.serialize(&0x3bcbf734_u32)?,
             SetClientDhParamsAnswer::DhGenRetry { .. } => buf.serialize(&0x46dc1fb9_u32)?,
             SetClientDhParamsAnswer::DhGenFail { .. } => buf.serialize(&0xa69dae02_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            SetClientDhParamsAnswer::DhGenOk {
                ref nonce,
                ref server_nonce,
                ref new_nonce_hash1,
            } => {
                buf.serialize(nonce)?;
                buf.serialize(server_nonce)?;
                buf.serialize(new_nonce_hash1)?;
            },

            SetClientDhParamsAnswer::DhGenRetry {
                ref nonce,
                ref server_nonce,
                ref new_nonce_hash2,
            } => {
                buf.serialize(nonce)?;
                buf.serialize(server_nonce)?;
                buf.serialize(new_nonce_hash2)?;
            },

            SetClientDhParamsAnswer::DhGenFail {
                ref nonce,
                ref server_nonce,
                ref new_nonce_hash3,
            } => {
                buf.serialize(nonce)?;
                buf.serialize(server_nonce)?;
                buf.serialize(new_nonce_hash3)?;
            },
        }

        Ok(())
    }
}

impl Serializable for InputGeoPoint {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputGeoPoint::Empty { .. } => buf.serialize(&0xe4c123d6_u32)?,
             InputGeoPoint::Point { .. } => buf.serialize(&0xf3b7acc9_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputGeoPoint::Empty {
            } => {
            },

            InputGeoPoint::Point {
                ref lat,
                ref long,
            } => {
                buf.serialize(lat)?;
                buf.serialize(long)?;
            },
        }

        Ok(())
    }
}

impl Serializable for IpPort {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xd433ad73_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.ipv4)?;
        buf.serialize(&self.port)?;

        Ok(())
    }
 }

impl Serializable for MessagesPeerDialogs {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3371c354_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.dialogs)?;
        buf.serialize(&self.messages)?;
        buf.serialize(&self.chats)?;
        buf.serialize(&self.users)?;
        buf.serialize(self.state.as_ref())?;

        Ok(())
    }
 }

impl Serializable for NotifyPeer {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             NotifyPeer::All { .. } => buf.serialize(&0x74d07c60_u32)?,
             NotifyPeer::Users { .. } => buf.serialize(&0xb4c83b4c_u32)?,
             NotifyPeer::Peer { .. } => buf.serialize(&0x9fd40bd8_u32)?,
             NotifyPeer::Chats { .. } => buf.serialize(&0xc007cec3_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            NotifyPeer::All {
            } => {
            },

            NotifyPeer::Users {
            } => {
            },

            NotifyPeer::Peer {
                ref peer,
            } => {
                buf.serialize(peer.as_ref())?;
            },

            NotifyPeer::Chats {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for DialogPeer {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             DialogPeer::Feed { .. } => buf.serialize(&0xda429411_u32)?,
             DialogPeer::Peer { .. } => buf.serialize(&0xe56dbf05_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            DialogPeer::Feed {
                ref feed_id,
            } => {
                buf.serialize(feed_id)?;
            },

            DialogPeer::Peer {
                ref peer,
            } => {
                buf.serialize(peer.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for RpcError {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x2144ca19_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.error_code)?;
        buf.serialize(&self.error_message)?;

        Ok(())
    }
 }

impl Serializable for ChatParticipants {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ChatParticipants::Forbidden { .. } => buf.serialize(&0xfc900c2b_u32)?,
             ChatParticipants::Participants { .. } => buf.serialize(&0x3f460fed_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ChatParticipants::Forbidden {
                flags: _,
                ref chat_id,
                ref self_participant,
            } => {
                let mut ser_flags: u32 = 0;

                if self_participant.is_some() {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(chat_id)?;

                if let Some(ref value) = self_participant {
                    buf.serialize(value.as_ref())?;
                }

            },

            ChatParticipants::Participants {
                ref chat_id,
                ref participants,
                ref version,
            } => {
                buf.serialize(chat_id)?;
                buf.serialize(participants)?;
                buf.serialize(version)?;
            },
        }

        Ok(())
    }
}

impl Serializable for DestroySessionRes {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             DestroySessionRes::Ok { .. } => buf.serialize(&0xe22045fc_u32)?,
             DestroySessionRes::None { .. } => buf.serialize(&0x62d350c9_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            DestroySessionRes::Ok {
                ref session_id,
            } => {
                buf.serialize(session_id)?;
            },

            DestroySessionRes::None {
                ref session_id,
            } => {
                buf.serialize(session_id)?;
            },
        }

        Ok(())
    }
}

impl Serializable for FutureSalt {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x0949d9dc_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.valid_since)?;
        buf.serialize(&self.valid_until)?;
        buf.serialize(&self.salt)?;

        Ok(())
    }
 }

impl Serializable for ChannelsChannelParticipants {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ChannelsChannelParticipants::NotModified { .. } => buf.serialize(&0xf0173fe9_u32)?,
             ChannelsChannelParticipants::Participants { .. } => buf.serialize(&0xf56ee2a8_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ChannelsChannelParticipants::NotModified {
            } => {
            },

            ChannelsChannelParticipants::Participants {
                ref count,
                ref participants,
                ref users,
            } => {
                buf.serialize(count)?;
                buf.serialize(participants)?;
                buf.serialize(users)?;
            },
        }

        Ok(())
    }
}

impl Serializable for Pong {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x347773c5_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.msg_id)?;
        buf.serialize(&self.ping_id)?;

        Ok(())
    }
 }

impl Serializable for HelpConfigSimple {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xd997c3c5_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.date)?;
        buf.serialize(&self.expires)?;
        buf.serialize(&self.dc_id)?;
        buf.serialize(&self.ip_port_list)?;

        Ok(())
    }
 }

impl Serializable for KeyboardButtonRow {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x77608b83_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.buttons)?;

        Ok(())
    }
 }

impl Serializable for UserStatus {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             UserStatus::Recently { .. } => buf.serialize(&0xe26f42f1_u32)?,
             UserStatus::Offline { .. } => buf.serialize(&0x008c703f_u32)?,
             UserStatus::Online { .. } => buf.serialize(&0xedb93949_u32)?,
             UserStatus::LastMonth { .. } => buf.serialize(&0x77ebc742_u32)?,
             UserStatus::Empty { .. } => buf.serialize(&0x09d05049_u32)?,
             UserStatus::LastWeek { .. } => buf.serialize(&0x07bf09fc_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            UserStatus::Recently {
            } => {
            },

            UserStatus::Offline {
                ref was_online,
            } => {
                buf.serialize(was_online)?;
            },

            UserStatus::Online {
                ref expires,
            } => {
                buf.serialize(expires)?;
            },

            UserStatus::LastMonth {
            } => {
            },

            UserStatus::Empty {
            } => {
            },

            UserStatus::LastWeek {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for ReportReason {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ReportReason::InputViolence { .. } => buf.serialize(&0x1e22c78d_u32)?,
             ReportReason::InputPornography { .. } => buf.serialize(&0x2e59d922_u32)?,
             ReportReason::InputOther { .. } => buf.serialize(&0xe1746d0a_u32)?,
             ReportReason::InputSpam { .. } => buf.serialize(&0x58dbcab8_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ReportReason::InputViolence {
            } => {
            },

            ReportReason::InputPornography {
            } => {
            },

            ReportReason::InputOther {
                ref text,
            } => {
                buf.serialize(text)?;
            },

            ReportReason::InputSpam {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for InputChannel {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputChannel::Empty { .. } => buf.serialize(&0xee8c1e86_u32)?,
             InputChannel::Channel { .. } => buf.serialize(&0xafeb712e_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputChannel::Empty {
            } => {
            },

            InputChannel::Channel {
                ref channel_id,
                ref access_hash,
            } => {
                buf.serialize(channel_id)?;
                buf.serialize(access_hash)?;
            },
        }

        Ok(())
    }
}

impl Serializable for PhotoSize {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             PhotoSize::Size { .. } => buf.serialize(&0x77bfb61b_u32)?,
             PhotoSize::Cached { .. } => buf.serialize(&0xe9a734fa_u32)?,
             PhotoSize::Empty { .. } => buf.serialize(&0x0e17e23c_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            PhotoSize::Size {
                ref type_,
                ref location,
                ref w,
                ref h,
                ref size,
            } => {
                buf.serialize(type_)?;
                buf.serialize(location.as_ref())?;
                buf.serialize(w)?;
                buf.serialize(h)?;
                buf.serialize(size)?;
            },

            PhotoSize::Cached {
                ref type_,
                ref location,
                ref w,
                ref h,
                ref bytes,
            } => {
                buf.serialize(type_)?;
                buf.serialize(location.as_ref())?;
                buf.serialize(w)?;
                buf.serialize(h)?;
                buf.serialize(bytes)?;
            },

            PhotoSize::Empty {
                ref type_,
            } => {
                buf.serialize(type_)?;
            },
        }

        Ok(())
    }
}

impl Serializable for PageBlock {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             PageBlock::Subheader { .. } => buf.serialize(&0xf12bb6e1_u32)?,
             PageBlock::Divider { .. } => buf.serialize(&0xdb20b188_u32)?,
             PageBlock::Blockquote { .. } => buf.serialize(&0x263d7c26_u32)?,
             PageBlock::Anchor { .. } => buf.serialize(&0xce0d37b0_u32)?,
             PageBlock::AuthorDate { .. } => buf.serialize(&0xbaafe5e0_u32)?,
             PageBlock::EmbedPost { .. } => buf.serialize(&0x292c7be9_u32)?,
             PageBlock::Collage { .. } => buf.serialize(&0x08b31c4f_u32)?,
             PageBlock::Unsupported { .. } => buf.serialize(&0x13567e8a_u32)?,
             PageBlock::List { .. } => buf.serialize(&0x3a58c7f4_u32)?,
             PageBlock::Preformatted { .. } => buf.serialize(&0xc070d93e_u32)?,
             PageBlock::Channel { .. } => buf.serialize(&0xef1751b5_u32)?,
             PageBlock::Embed { .. } => buf.serialize(&0xcde200d1_u32)?,
             PageBlock::Title { .. } => buf.serialize(&0x70abc3fd_u32)?,
             PageBlock::Pullquote { .. } => buf.serialize(&0x4f4456d3_u32)?,
             PageBlock::Header { .. } => buf.serialize(&0xbfd064ec_u32)?,
             PageBlock::Paragraph { .. } => buf.serialize(&0x467a0766_u32)?,
             PageBlock::Cover { .. } => buf.serialize(&0x39f23300_u32)?,
             PageBlock::Slideshow { .. } => buf.serialize(&0x130c8963_u32)?,
             PageBlock::Video { .. } => buf.serialize(&0xd9d71866_u32)?,
             PageBlock::Footer { .. } => buf.serialize(&0x48870999_u32)?,
             PageBlock::Photo { .. } => buf.serialize(&0xe9c69982_u32)?,
             PageBlock::Audio { .. } => buf.serialize(&0x31b81a7f_u32)?,
             PageBlock::Subtitle { .. } => buf.serialize(&0x8ffa9a1f_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            PageBlock::Subheader {
                ref text,
            } => {
                buf.serialize(text.as_ref())?;
            },

            PageBlock::Divider {
            } => {
            },

            PageBlock::Blockquote {
                ref text,
                ref caption,
            } => {
                buf.serialize(text.as_ref())?;
                buf.serialize(caption.as_ref())?;
            },

            PageBlock::Anchor {
                ref name,
            } => {
                buf.serialize(name)?;
            },

            PageBlock::AuthorDate {
                ref author,
                ref published_date,
            } => {
                buf.serialize(author.as_ref())?;
                buf.serialize(published_date)?;
            },

            PageBlock::EmbedPost {
                ref url,
                ref webpage_id,
                ref author_photo_id,
                ref author,
                ref date,
                ref blocks,
                ref caption,
            } => {
                buf.serialize(url)?;
                buf.serialize(webpage_id)?;
                buf.serialize(author_photo_id)?;
                buf.serialize(author)?;
                buf.serialize(date)?;
                buf.serialize(blocks)?;
                buf.serialize(caption.as_ref())?;
            },

            PageBlock::Collage {
                ref items,
                ref caption,
            } => {
                buf.serialize(items)?;
                buf.serialize(caption.as_ref())?;
            },

            PageBlock::Unsupported {
            } => {
            },

            PageBlock::List {
                ref ordered,
                ref items,
            } => {
                buf.serialize(ordered)?;
                buf.serialize(items)?;
            },

            PageBlock::Preformatted {
                ref text,
                ref language,
            } => {
                buf.serialize(text.as_ref())?;
                buf.serialize(language)?;
            },

            PageBlock::Channel {
                ref channel,
            } => {
                buf.serialize(channel.as_ref())?;
            },

            PageBlock::Embed {
                flags: _,
                ref full_width,
                ref allow_scrolling,
                ref url,
                ref html,
                ref poster_photo_id,
                ref w,
                ref h,
                ref caption,
            } => {
                let mut ser_flags: u32 = 0;

                if full_width {
                    ser_flags |= 1 << 0;
                }

                if allow_scrolling {
                    ser_flags |= 1 << 3;
                }

                if url.is_some() {
                    ser_flags |= 1 << 1;
                }

                if html.is_some() {
                    ser_flags |= 1 << 2;
                }

                if poster_photo_id.is_some() {
                    ser_flags |= 1 << 4;
                }

                buf.serialize(&ser_flags)?;

                if let Some(ref value) = url {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = html {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = poster_photo_id {
                    buf.serialize(value)?;
                }

                buf.serialize(w)?;
                buf.serialize(h)?;
                buf.serialize(caption.as_ref())?;
            },

            PageBlock::Title {
                ref text,
            } => {
                buf.serialize(text.as_ref())?;
            },

            PageBlock::Pullquote {
                ref text,
                ref caption,
            } => {
                buf.serialize(text.as_ref())?;
                buf.serialize(caption.as_ref())?;
            },

            PageBlock::Header {
                ref text,
            } => {
                buf.serialize(text.as_ref())?;
            },

            PageBlock::Paragraph {
                ref text,
            } => {
                buf.serialize(text.as_ref())?;
            },

            PageBlock::Cover {
                ref cover,
            } => {
                buf.serialize(cover.as_ref())?;
            },

            PageBlock::Slideshow {
                ref items,
                ref caption,
            } => {
                buf.serialize(items)?;
                buf.serialize(caption.as_ref())?;
            },

            PageBlock::Video {
                flags: _,
                ref autoplay,
                ref loop_,
                ref video_id,
                ref caption,
            } => {
                let mut ser_flags: u32 = 0;

                if autoplay {
                    ser_flags |= 1 << 0;
                }

                if loop_ {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(video_id)?;
                buf.serialize(caption.as_ref())?;
            },

            PageBlock::Footer {
                ref text,
            } => {
                buf.serialize(text.as_ref())?;
            },

            PageBlock::Photo {
                ref photo_id,
                ref caption,
            } => {
                buf.serialize(photo_id)?;
                buf.serialize(caption.as_ref())?;
            },

            PageBlock::Audio {
                ref audio_id,
                ref caption,
            } => {
                buf.serialize(audio_id)?;
                buf.serialize(caption.as_ref())?;
            },

            PageBlock::Subtitle {
                ref text,
            } => {
                buf.serialize(text.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for AuthExportedAuthorization {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xdf969c2d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;
        buf.serialize(&self.bytes)?;

        Ok(())
    }
 }

impl Serializable for ContactsBlocked {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ContactsBlocked::Slice { .. } => buf.serialize(&0x900802a1_u32)?,
             ContactsBlocked::Blocked { .. } => buf.serialize(&0x1c138d15_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ContactsBlocked::Slice {
                ref count,
                ref blocked,
                ref users,
            } => {
                buf.serialize(count)?;
                buf.serialize(blocked)?;
                buf.serialize(users)?;
            },

            ContactsBlocked::Blocked {
                ref blocked,
                ref users,
            } => {
                buf.serialize(blocked)?;
                buf.serialize(users)?;
            },
        }

        Ok(())
    }
}

impl Serializable for InputGame {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputGame::Id { .. } => buf.serialize(&0x032c3e77_u32)?,
             InputGame::ShortName { .. } => buf.serialize(&0xc331e80a_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputGame::Id {
                ref id,
                ref access_hash,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
            },

            InputGame::ShortName {
                ref bot_id,
                ref short_name,
            } => {
                buf.serialize(bot_id.as_ref())?;
                buf.serialize(short_name)?;
            },
        }

        Ok(())
    }
}

impl Serializable for MessagesFilter {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessagesFilter::InputGeo { .. } => buf.serialize(&0xe7026d0d_u32)?,
             MessagesFilter::InputPhotoVideo { .. } => buf.serialize(&0x56e9f0e4_u32)?,
             MessagesFilter::InputPhoneCalls { .. } => buf.serialize(&0x80c99768_u32)?,
             MessagesFilter::InputVideo { .. } => buf.serialize(&0x9fc00e65_u32)?,
             MessagesFilter::InputRoundVideo { .. } => buf.serialize(&0xb549da53_u32)?,
             MessagesFilter::InputVoice { .. } => buf.serialize(&0x50f5c392_u32)?,
             MessagesFilter::InputPhotos { .. } => buf.serialize(&0x9609a51c_u32)?,
             MessagesFilter::InputGif { .. } => buf.serialize(&0xffc86587_u32)?,
             MessagesFilter::InputRoundVoice { .. } => buf.serialize(&0x7a7c17a4_u32)?,
             MessagesFilter::InputDocument { .. } => buf.serialize(&0x9eddf188_u32)?,
             MessagesFilter::InputMusic { .. } => buf.serialize(&0x3751b49e_u32)?,
             MessagesFilter::InputEmpty { .. } => buf.serialize(&0x57e2f66c_u32)?,
             MessagesFilter::InputContacts { .. } => buf.serialize(&0xe062db83_u32)?,
             MessagesFilter::InputMyMentions { .. } => buf.serialize(&0xc1f8e69a_u32)?,
             MessagesFilter::InputChatPhotos { .. } => buf.serialize(&0x3a20ecb8_u32)?,
             MessagesFilter::InputUrl { .. } => buf.serialize(&0x7ef0dd87_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessagesFilter::InputGeo {
            } => {
            },

            MessagesFilter::InputPhotoVideo {
            } => {
            },

            MessagesFilter::InputPhoneCalls {
                flags: _,
                ref missed,
            } => {
                let mut ser_flags: u32 = 0;

                if missed {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
            },

            MessagesFilter::InputVideo {
            } => {
            },

            MessagesFilter::InputRoundVideo {
            } => {
            },

            MessagesFilter::InputVoice {
            } => {
            },

            MessagesFilter::InputPhotos {
            } => {
            },

            MessagesFilter::InputGif {
            } => {
            },

            MessagesFilter::InputRoundVoice {
            } => {
            },

            MessagesFilter::InputDocument {
            } => {
            },

            MessagesFilter::InputMusic {
            } => {
            },

            MessagesFilter::InputEmpty {
            } => {
            },

            MessagesFilter::InputContacts {
            } => {
            },

            MessagesFilter::InputMyMentions {
            } => {
            },

            MessagesFilter::InputChatPhotos {
            } => {
            },

            MessagesFilter::InputUrl {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for InputStickerSetItem {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xffa0a496_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.mask_coords.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.document.as_ref())?;
        buf.serialize(&self.emoji)?;

        if let Some(ref value) = self.mask_coords {
            buf.serialize(value.as_ref())?;
        }


        Ok(())
    }
 }

impl Serializable for MessagesDialogs {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessagesDialogs::Dialogs { .. } => buf.serialize(&0x15ba6c40_u32)?,
             MessagesDialogs::Slice { .. } => buf.serialize(&0x71e094f3_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessagesDialogs::Dialogs {
                ref dialogs,
                ref messages,
                ref chats,
                ref users,
            } => {
                buf.serialize(dialogs)?;
                buf.serialize(messages)?;
                buf.serialize(chats)?;
                buf.serialize(users)?;
            },

            MessagesDialogs::Slice {
                ref count,
                ref dialogs,
                ref messages,
                ref chats,
                ref users,
            } => {
                buf.serialize(count)?;
                buf.serialize(dialogs)?;
                buf.serialize(messages)?;
                buf.serialize(chats)?;
                buf.serialize(users)?;
            },
        }

        Ok(())
    }
}

impl Serializable for ChannelParticipantsFilter {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ChannelParticipantsFilter::Admins { .. } => buf.serialize(&0xb4608969_u32)?,
             ChannelParticipantsFilter::Search { .. } => buf.serialize(&0x0656ac4b_u32)?,
             ChannelParticipantsFilter::Bots { .. } => buf.serialize(&0xb0d1865b_u32)?,
             ChannelParticipantsFilter::Banned { .. } => buf.serialize(&0x1427a5e1_u32)?,
             ChannelParticipantsFilter::Kicked { .. } => buf.serialize(&0xa3b54985_u32)?,
             ChannelParticipantsFilter::Recent { .. } => buf.serialize(&0xde3f3c79_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ChannelParticipantsFilter::Admins {
            } => {
            },

            ChannelParticipantsFilter::Search {
                ref q,
            } => {
                buf.serialize(q)?;
            },

            ChannelParticipantsFilter::Bots {
            } => {
            },

            ChannelParticipantsFilter::Banned {
                ref q,
            } => {
                buf.serialize(q)?;
            },

            ChannelParticipantsFilter::Kicked {
                ref q,
            } => {
                buf.serialize(q)?;
            },

            ChannelParticipantsFilter::Recent {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for ChannelsAdminLogResults {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xed8af74d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.events)?;
        buf.serialize(&self.chats)?;
        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for InputPrivacyRule {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputPrivacyRule::ValueDisallowContacts { .. } => buf.serialize(&0x0ba52007_u32)?,
             InputPrivacyRule::ValueDisallowAll { .. } => buf.serialize(&0xd66b66c9_u32)?,
             InputPrivacyRule::ValueAllowContacts { .. } => buf.serialize(&0x0d09e07b_u32)?,
             InputPrivacyRule::ValueDisallowUsers { .. } => buf.serialize(&0x90110467_u32)?,
             InputPrivacyRule::ValueAllowAll { .. } => buf.serialize(&0x184b35ce_u32)?,
             InputPrivacyRule::ValueAllowUsers { .. } => buf.serialize(&0x131cc67f_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputPrivacyRule::ValueDisallowContacts {
            } => {
            },

            InputPrivacyRule::ValueDisallowAll {
            } => {
            },

            InputPrivacyRule::ValueAllowContacts {
            } => {
            },

            InputPrivacyRule::ValueDisallowUsers {
                ref users,
            } => {
                buf.serialize(users)?;
            },

            InputPrivacyRule::ValueAllowAll {
            } => {
            },

            InputPrivacyRule::ValueAllowUsers {
                ref users,
            } => {
                buf.serialize(users)?;
            },
        }

        Ok(())
    }
}

impl Serializable for Document {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             Document::Document { .. } => buf.serialize(&0x87232bc7_u32)?,
             Document::Empty { .. } => buf.serialize(&0x36f8c871_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            Document::Document {
                ref id,
                ref access_hash,
                ref date,
                ref mime_type,
                ref size,
                ref thumb,
                ref dc_id,
                ref version,
                ref attributes,
            } => {
                buf.serialize(id)?;
                buf.serialize(access_hash)?;
                buf.serialize(date)?;
                buf.serialize(mime_type)?;
                buf.serialize(size)?;
                buf.serialize(thumb.as_ref())?;
                buf.serialize(dc_id)?;
                buf.serialize(version)?;
                buf.serialize(attributes)?;
            },

            Document::Empty {
                ref id,
            } => {
                buf.serialize(id)?;
            },
        }

        Ok(())
    }
}

impl Serializable for MessagesMessages {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessagesMessages::ChannelMessages { .. } => buf.serialize(&0x99262e37_u32)?,
             MessagesMessages::MessagesSlice { .. } => buf.serialize(&0x0b446ae3_u32)?,
             MessagesMessages::MessagesNotModified { .. } => buf.serialize(&0x74535f21_u32)?,
             MessagesMessages::Messages { .. } => buf.serialize(&0x8c718e87_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessagesMessages::ChannelMessages {
                flags: _,
                ref pts,
                ref count,
                ref messages,
                ref chats,
                ref users,
            } => {
                let mut ser_flags: u32 = 0;

                buf.serialize(&ser_flags)?;
                buf.serialize(pts)?;
                buf.serialize(count)?;
                buf.serialize(messages)?;
                buf.serialize(chats)?;
                buf.serialize(users)?;
            },

            MessagesMessages::MessagesSlice {
                ref count,
                ref messages,
                ref chats,
                ref users,
            } => {
                buf.serialize(count)?;
                buf.serialize(messages)?;
                buf.serialize(chats)?;
                buf.serialize(users)?;
            },

            MessagesMessages::MessagesNotModified {
                ref count,
            } => {
                buf.serialize(count)?;
            },

            MessagesMessages::Messages {
                ref messages,
                ref chats,
                ref users,
            } => {
                buf.serialize(messages)?;
                buf.serialize(chats)?;
                buf.serialize(users)?;
            },
        }

        Ok(())
    }
}

impl Serializable for PeerSettings {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x818426cd_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.report_spam {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;

        Ok(())
    }
 }

impl Serializable for DraftMessage {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             DraftMessage::Empty { .. } => buf.serialize(&0xba4baec5_u32)?,
             DraftMessage::Message { .. } => buf.serialize(&0xfd8e711f_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            DraftMessage::Empty {
            } => {
            },

            DraftMessage::Message {
                flags: _,
                ref no_webpage,
                ref reply_to_msg_id,
                ref message,
                ref entities,
                ref date,
            } => {
                let mut ser_flags: u32 = 0;

                if no_webpage {
                    ser_flags |= 1 << 1;
                }

                if reply_to_msg_id.is_some() {
                    ser_flags |= 1 << 0;
                }

                if entities.is_some() {
                    ser_flags |= 1 << 3;
                }

                buf.serialize(&ser_flags)?;

                if let Some(ref value) = reply_to_msg_id {
                    buf.serialize(value)?;
                }

                buf.serialize(message)?;

                if let Some(ref value) = entities {
                    buf.serialize(value)?;
                }

                buf.serialize(date)?;
            },
        }

        Ok(())
    }
}

impl Serializable for MessagesStickerSet {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xb60a24a6_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.set.as_ref())?;
        buf.serialize(&self.packs)?;
        buf.serialize(&self.documents)?;

        Ok(())
    }
 }

impl Serializable for ContactsLink {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3ace484c_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.my_link.as_ref())?;
        buf.serialize(self.foreign_link.as_ref())?;
        buf.serialize(self.user.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ContactsFound {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xb3134d9d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.my_results)?;
        buf.serialize(&self.results)?;
        buf.serialize(&self.chats)?;
        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for BadMsgNotification {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             BadMsgNotification::Notification { .. } => buf.serialize(&0xa7eff811_u32)?,
             BadMsgNotification::ServerSalt { .. } => buf.serialize(&0xedab447b_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            BadMsgNotification::Notification {
                ref bad_msg_id,
                ref bad_msg_seqno,
                ref error_code,
            } => {
                buf.serialize(bad_msg_id)?;
                buf.serialize(bad_msg_seqno)?;
                buf.serialize(error_code)?;
            },

            BadMsgNotification::ServerSalt {
                ref bad_msg_id,
                ref bad_msg_seqno,
                ref error_code,
                ref new_server_salt,
            } => {
                buf.serialize(bad_msg_id)?;
                buf.serialize(bad_msg_seqno)?;
                buf.serialize(error_code)?;
                buf.serialize(new_server_salt)?;
            },
        }

        Ok(())
    }
}

impl Serializable for InputPaymentCredentials {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputPaymentCredentials::AndroidPay { .. } => buf.serialize(&0xca05d50e_u32)?,
             InputPaymentCredentials::Credentials { .. } => buf.serialize(&0x3417d728_u32)?,
             InputPaymentCredentials::ApplePay { .. } => buf.serialize(&0x0aa1c39f_u32)?,
             InputPaymentCredentials::Saved { .. } => buf.serialize(&0xc10eb2cf_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputPaymentCredentials::AndroidPay {
                ref payment_token,
                ref google_transaction_id,
            } => {
                buf.serialize(payment_token.as_ref())?;
                buf.serialize(google_transaction_id)?;
            },

            InputPaymentCredentials::Credentials {
                flags: _,
                ref save,
                ref data,
            } => {
                let mut ser_flags: u32 = 0;

                if save {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(data.as_ref())?;
            },

            InputPaymentCredentials::ApplePay {
                ref payment_data,
            } => {
                buf.serialize(payment_data.as_ref())?;
            },

            InputPaymentCredentials::Saved {
                ref id,
                ref tmp_password,
            } => {
                buf.serialize(id)?;
                buf.serialize(tmp_password)?;
            },
        }

        Ok(())
    }
}

impl Serializable for ContactBlocked {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x561bc879_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.user_id)?;
        buf.serialize(&self.date)?;

        Ok(())
    }
 }

impl Serializable for StorageFileType {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             StorageFileType::Unknown { .. } => buf.serialize(&0xaa963b05_u32)?,
             StorageFileType::Png { .. } => buf.serialize(&0x0a4f63c0_u32)?,
             StorageFileType::Pdf { .. } => buf.serialize(&0xae1e508d_u32)?,
             StorageFileType::Mp3 { .. } => buf.serialize(&0x528a0677_u32)?,
             StorageFileType::Gif { .. } => buf.serialize(&0xcae1aadf_u32)?,
             StorageFileType::Webp { .. } => buf.serialize(&0x1081464c_u32)?,
             StorageFileType::Mov { .. } => buf.serialize(&0x4b09ebbc_u32)?,
             StorageFileType::Jpeg { .. } => buf.serialize(&0x007efe0e_u32)?,
             StorageFileType::Mp4 { .. } => buf.serialize(&0xb3cea0e4_u32)?,
             StorageFileType::Partial { .. } => buf.serialize(&0x40bc6f52_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            StorageFileType::Unknown {
            } => {
            },

            StorageFileType::Png {
            } => {
            },

            StorageFileType::Pdf {
            } => {
            },

            StorageFileType::Mp3 {
            } => {
            },

            StorageFileType::Gif {
            } => {
            },

            StorageFileType::Webp {
            } => {
            },

            StorageFileType::Mov {
            } => {
            },

            StorageFileType::Jpeg {
            } => {
            },

            StorageFileType::Mp4 {
            } => {
            },

            StorageFileType::Partial {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for ChannelsFeedSources {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ChannelsFeedSources::NotModified { .. } => buf.serialize(&0x88b12a17_u32)?,
             ChannelsFeedSources::Sources { .. } => buf.serialize(&0x8e8bca3d_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ChannelsFeedSources::NotModified {
            } => {
            },

            ChannelsFeedSources::Sources {
                flags: _,
                ref newly_joined_feed,
                ref feeds,
                ref chats,
                ref users,
            } => {
                let mut ser_flags: u32 = 0;

                if newly_joined_feed.is_some() {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;

                if let Some(ref value) = newly_joined_feed {
                    buf.serialize(value)?;
                }

                buf.serialize(feeds)?;
                buf.serialize(chats)?;
                buf.serialize(users)?;
            },
        }

        Ok(())
    }
}

impl Serializable for AccountPasswordSettings {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xb7b72ab3_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.email)?;

        Ok(())
    }
 }

impl Serializable for ChatInvite {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ChatInvite::Invite { .. } => buf.serialize(&0xdb74f558_u32)?,
             ChatInvite::Already { .. } => buf.serialize(&0x5a686d7c_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ChatInvite::Invite {
                flags: _,
                ref channel,
                ref broadcast,
                ref public,
                ref megagroup,
                ref title,
                ref photo,
                ref participants_count,
                ref participants,
            } => {
                let mut ser_flags: u32 = 0;

                if channel {
                    ser_flags |= 1 << 0;
                }

                if broadcast {
                    ser_flags |= 1 << 1;
                }

                if public {
                    ser_flags |= 1 << 2;
                }

                if megagroup {
                    ser_flags |= 1 << 3;
                }

                if participants.is_some() {
                    ser_flags |= 1 << 4;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(title)?;
                buf.serialize(photo.as_ref())?;
                buf.serialize(participants_count)?;

                if let Some(ref value) = participants {
                    buf.serialize(value)?;
                }

            },

            ChatInvite::Already {
                ref chat,
            } => {
                buf.serialize(chat.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for ReplyMarkup {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ReplyMarkup::Keyboard { .. } => buf.serialize(&0x3502758c_u32)?,
             ReplyMarkup::KeyboardHide { .. } => buf.serialize(&0xa03e5b85_u32)?,
             ReplyMarkup::Inline { .. } => buf.serialize(&0x48a30254_u32)?,
             ReplyMarkup::KeyboardForceReply { .. } => buf.serialize(&0xf4108aa0_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ReplyMarkup::Keyboard {
                flags: _,
                ref resize,
                ref single_use,
                ref selective,
                ref rows,
            } => {
                let mut ser_flags: u32 = 0;

                if resize {
                    ser_flags |= 1 << 0;
                }

                if single_use {
                    ser_flags |= 1 << 1;
                }

                if selective {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(rows)?;
            },

            ReplyMarkup::KeyboardHide {
                flags: _,
                ref selective,
            } => {
                let mut ser_flags: u32 = 0;

                if selective {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
            },

            ReplyMarkup::Inline {
                ref rows,
            } => {
                buf.serialize(rows)?;
            },

            ReplyMarkup::KeyboardForceReply {
                flags: _,
                ref single_use,
                ref selective,
            } => {
                let mut ser_flags: u32 = 0;

                if single_use {
                    ser_flags |= 1 << 1;
                }

                if selective {
                    ser_flags |= 1 << 2;
                }

                buf.serialize(&ser_flags)?;
            },
        }

        Ok(())
    }
}

impl Serializable for BotInfo {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x98e81d3a_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.user_id)?;
        buf.serialize(&self.description)?;
        buf.serialize(&self.commands)?;

        Ok(())
    }
 }

impl Serializable for InputPeer {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             InputPeer::Self_ { .. } => buf.serialize(&0x7da07ec9_u32)?,
             InputPeer::Empty { .. } => buf.serialize(&0x7f3b18ea_u32)?,
             InputPeer::Chat { .. } => buf.serialize(&0x179be863_u32)?,
             InputPeer::User { .. } => buf.serialize(&0x7b8e7de6_u32)?,
             InputPeer::Channel { .. } => buf.serialize(&0x20adaef8_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            InputPeer::Self_ {
            } => {
            },

            InputPeer::Empty {
            } => {
            },

            InputPeer::Chat {
                ref chat_id,
            } => {
                buf.serialize(chat_id)?;
            },

            InputPeer::User {
                ref user_id,
                ref access_hash,
            } => {
                buf.serialize(user_id)?;
                buf.serialize(access_hash)?;
            },

            InputPeer::Channel {
                ref channel_id,
                ref access_hash,
            } => {
                buf.serialize(channel_id)?;
                buf.serialize(access_hash)?;
            },
        }

        Ok(())
    }
}

impl Serializable for ChatPhoto {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ChatPhoto::Empty { .. } => buf.serialize(&0x37c1011c_u32)?,
             ChatPhoto::Photo { .. } => buf.serialize(&0x6153276a_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ChatPhoto::Empty {
            } => {
            },

            ChatPhoto::Photo {
                ref photo_small,
                ref photo_big,
            } => {
                buf.serialize(photo_small.as_ref())?;
                buf.serialize(photo_big.as_ref())?;
            },
        }

        Ok(())
    }
}

impl Serializable for GeoPoint {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             GeoPoint::Point { .. } => buf.serialize(&0x2049d70c_u32)?,
             GeoPoint::Empty { .. } => buf.serialize(&0x1117dd5f_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            GeoPoint::Point {
                ref long,
                ref lat,
            } => {
                buf.serialize(long)?;
                buf.serialize(lat)?;
            },

            GeoPoint::Empty {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for WebPage {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             WebPage::NotModified { .. } => buf.serialize(&0x85849473_u32)?,
             WebPage::Pending { .. } => buf.serialize(&0xc586da1c_u32)?,
             WebPage::Page { .. } => buf.serialize(&0x5f07b4bc_u32)?,
             WebPage::Empty { .. } => buf.serialize(&0xeb1477e8_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            WebPage::NotModified {
            } => {
            },

            WebPage::Pending {
                ref id,
                ref date,
            } => {
                buf.serialize(id)?;
                buf.serialize(date)?;
            },

            WebPage::Page {
                flags: _,
                ref id,
                ref url,
                ref display_url,
                ref hash,
                ref type_,
                ref site_name,
                ref title,
                ref description,
                ref photo,
                ref embed_url,
                ref embed_type,
                ref embed_width,
                ref embed_height,
                ref duration,
                ref author,
                ref document,
                ref cached_page,
            } => {
                let mut ser_flags: u32 = 0;

                if type_.is_some() {
                    ser_flags |= 1 << 0;
                }

                if site_name.is_some() {
                    ser_flags |= 1 << 1;
                }

                if title.is_some() {
                    ser_flags |= 1 << 2;
                }

                if description.is_some() {
                    ser_flags |= 1 << 3;
                }

                if photo.is_some() {
                    ser_flags |= 1 << 4;
                }

                if embed_url.is_some() {
                    ser_flags |= 1 << 5;
                }

                if embed_type.is_some() {
                    ser_flags |= 1 << 5;
                }

                if embed_width.is_some() {
                    ser_flags |= 1 << 6;
                }

                if embed_height.is_some() {
                    ser_flags |= 1 << 6;
                }

                if duration.is_some() {
                    ser_flags |= 1 << 7;
                }

                if author.is_some() {
                    ser_flags |= 1 << 8;
                }

                if document.is_some() {
                    ser_flags |= 1 << 9;
                }

                if cached_page.is_some() {
                    ser_flags |= 1 << 10;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(id)?;
                buf.serialize(url)?;
                buf.serialize(display_url)?;
                buf.serialize(hash)?;

                if let Some(ref value) = type_ {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = site_name {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = title {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = description {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = photo {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = embed_url {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = embed_type {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = embed_width {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = embed_height {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = duration {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = author {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = document {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = cached_page {
                    buf.serialize(value.as_ref())?;
                }

            },

            WebPage::Empty {
                ref id,
            } => {
                buf.serialize(id)?;
            },
        }

        Ok(())
    }
}

impl Serializable for AuthCodeType {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             AuthCodeType::Call { .. } => buf.serialize(&0x741cd3e3_u32)?,
             AuthCodeType::FlashCall { .. } => buf.serialize(&0x226ccefb_u32)?,
             AuthCodeType::Sms { .. } => buf.serialize(&0x72a3158c_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            AuthCodeType::Call {
            } => {
            },

            AuthCodeType::FlashCall {
            } => {
            },

            AuthCodeType::Sms {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for InputWebDocument {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x9bed434d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.url)?;
        buf.serialize(&self.size)?;
        buf.serialize(&self.mime_type)?;
        buf.serialize(&self.attributes)?;

        Ok(())
    }
 }

impl Serializable for MsgDetailedInfo {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MsgDetailedInfo::New { .. } => buf.serialize(&0x809db6df_u32)?,
             MsgDetailedInfo::Info { .. } => buf.serialize(&0x276d3ec6_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MsgDetailedInfo::New {
                ref answer_msg_id,
                ref bytes,
                ref status,
            } => {
                buf.serialize(answer_msg_id)?;
                buf.serialize(bytes)?;
                buf.serialize(status)?;
            },

            MsgDetailedInfo::Info {
                ref msg_id,
                ref answer_msg_id,
                ref bytes,
                ref status,
            } => {
                buf.serialize(msg_id)?;
                buf.serialize(answer_msg_id)?;
                buf.serialize(bytes)?;
                buf.serialize(status)?;
            },
        }

        Ok(())
    }
}

impl Serializable for CdnFileHash {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x77eec38f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.offset)?;
        buf.serialize(&self.limit)?;
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for Error {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc4b9f9bb_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.code)?;
        buf.serialize(&self.text)?;

        Ok(())
    }
 }

impl Serializable for ServerDhInnerData {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xb5890dba_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.nonce)?;
        buf.serialize(&self.server_nonce)?;
        buf.serialize(&self.g)?;
        buf.serialize(&self.dh_prime)?;
        buf.serialize(&self.g_a)?;
        buf.serialize(&self.server_time)?;

        Ok(())
    }
 }

impl Serializable for MessagesSavedGifs {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessagesSavedGifs::Gifs { .. } => buf.serialize(&0x2e0709a5_u32)?,
             MessagesSavedGifs::NotModified { .. } => buf.serialize(&0xe8025ca2_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessagesSavedGifs::Gifs {
                ref hash,
                ref gifs,
            } => {
                buf.serialize(hash)?;
                buf.serialize(gifs)?;
            },

            MessagesSavedGifs::NotModified {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for PaymentCharge {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xea02c27e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;
        buf.serialize(&self.provider_charge_id)?;

        Ok(())
    }
 }

impl Serializable for Update {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             Update::PinnedDialogs { .. } => buf.serialize(&0xea4cb65b_u32)?,
             Update::ContactRegistered { .. } => buf.serialize(&0x2575bbb9_u32)?,
             Update::UserTyping { .. } => buf.serialize(&0x5c486927_u32)?,
             Update::ContactLink { .. } => buf.serialize(&0x9d2e67c5_u32)?,
             Update::DeleteChannelMessages { .. } => buf.serialize(&0xc37521c9_u32)?,
             Update::SavedGifs { .. } => buf.serialize(&0x9375341e_u32)?,
             Update::EditMessage { .. } => buf.serialize(&0xe40370a3_u32)?,
             Update::DeleteMessages { .. } => buf.serialize(&0xa20db0e5_u32)?,
             Update::ChannelMessageViews { .. } => buf.serialize(&0x98a12b4b_u32)?,
             Update::BotInlineQuery { .. } => buf.serialize(&0x54826690_u32)?,
             Update::Privacy { .. } => buf.serialize(&0xee3b272a_u32)?,
             Update::BotWebhookJson { .. } => buf.serialize(&0x8317c0c3_u32)?,
             Update::ReadFeed { .. } => buf.serialize(&0x994852a9_u32)?,
             Update::ContactsReset { .. } => buf.serialize(&0x7084a7be_u32)?,
             Update::BotInlineSend { .. } => buf.serialize(&0x0e48f964_u32)?,
             Update::ChannelWebPage { .. } => buf.serialize(&0x40771900_u32)?,
             Update::ChannelPinnedMessage { .. } => buf.serialize(&0x98592475_u32)?,
             Update::DialogPinned { .. } => buf.serialize(&0x19d27f3c_u32)?,
             Update::BotShippingQuery { .. } => buf.serialize(&0xe0cdc940_u32)?,
             Update::ChatUserTyping { .. } => buf.serialize(&0x9a65ea1f_u32)?,
             Update::BotWebhookJsonquery { .. } => buf.serialize(&0x9b9240a6_u32)?,
             Update::FavedStickers { .. } => buf.serialize(&0xe511996d_u32)?,
             Update::UserStatus { .. } => buf.serialize(&0x1bfbd823_u32)?,
             Update::DraftMessage { .. } => buf.serialize(&0xee2bb969_u32)?,
             Update::NewChannelMessage { .. } => buf.serialize(&0x62ba04d9_u32)?,
             Update::UserBlocked { .. } => buf.serialize(&0x80ece81a_u32)?,
             Update::ReadChannelOutbox { .. } => buf.serialize(&0x25d6c9c7_u32)?,
             Update::PhoneCall { .. } => buf.serialize(&0xab0f6b1e_u32)?,
             Update::ReadFeaturedStickers { .. } => buf.serialize(&0x571d2742_u32)?,
             Update::ReadMessagesContents { .. } => buf.serialize(&0x68c13933_u32)?,
             Update::StickerSetsOrder { .. } => buf.serialize(&0x0bb2d201_u32)?,
             Update::LangPackTooLong { .. } => buf.serialize(&0x10c2404b_u32)?,
             Update::ChannelTooLong { .. } => buf.serialize(&0xeb0467fb_u32)?,
             Update::RecentStickers { .. } => buf.serialize(&0x9a422c20_u32)?,
             Update::NotifySettings { .. } => buf.serialize(&0xbec268ef_u32)?,
             Update::ReadHistoryOutbox { .. } => buf.serialize(&0x2f2f21bf_u32)?,
             Update::Config { .. } => buf.serialize(&0xa229dd06_u32)?,
             Update::WebPage { .. } => buf.serialize(&0x7f891213_u32)?,
             Update::NewMessage { .. } => buf.serialize(&0x1f2b0afd_u32)?,
             Update::EncryptedMessagesRead { .. } => buf.serialize(&0x38fe25b7_u32)?,
             Update::DcOptions { .. } => buf.serialize(&0x8e5e9873_u32)?,
             Update::ServiceNotification { .. } => buf.serialize(&0xebe46819_u32)?,
             Update::EncryptedChatTyping { .. } => buf.serialize(&0x1710f156_u32)?,
             Update::ReadChannelInbox { .. } => buf.serialize(&0x4214f37f_u32)?,
             Update::MessageId { .. } => buf.serialize(&0x4e90bfd6_u32)?,
             Update::LangPack { .. } => buf.serialize(&0x56022f4d_u32)?,
             Update::ChatParticipantAdmin { .. } => buf.serialize(&0xb6901959_u32)?,
             Update::UserName { .. } => buf.serialize(&0xa7332b73_u32)?,
             Update::ChatParticipantAdd { .. } => buf.serialize(&0xea4b0e5c_u32)?,
             Update::Encryption { .. } => buf.serialize(&0xb4a2e88d_u32)?,
             Update::BotPrecheckoutQuery { .. } => buf.serialize(&0x5d2f3aa9_u32)?,
             Update::NewEncryptedMessage { .. } => buf.serialize(&0x12bcbd9a_u32)?,
             Update::Channel { .. } => buf.serialize(&0xb6d45656_u32)?,
             Update::NewStickerSet { .. } => buf.serialize(&0x688a30aa_u32)?,
             Update::ReadHistoryInbox { .. } => buf.serialize(&0x9961fd5c_u32)?,
             Update::ChannelAvailableMessages { .. } => buf.serialize(&0x70db6837_u32)?,
             Update::ChatAdmins { .. } => buf.serialize(&0x6e947941_u32)?,
             Update::UserPhone { .. } => buf.serialize(&0x12b9417b_u32)?,
             Update::BotCallbackQuery { .. } => buf.serialize(&0xe73547e1_u32)?,
             Update::ChatParticipantDelete { .. } => buf.serialize(&0x6e5f8c22_u32)?,
             Update::InlineBotCallbackQuery { .. } => buf.serialize(&0xf9d27a5a_u32)?,
             Update::ChatParticipants { .. } => buf.serialize(&0x07761198_u32)?,
             Update::StickerSets { .. } => buf.serialize(&0x43ae3dec_u32)?,
             Update::PtsChanged { .. } => buf.serialize(&0x3354678f_u32)?,
             Update::ChannelReadMessagesContents { .. } => buf.serialize(&0x89893b45_u32)?,
             Update::UserPhoto { .. } => buf.serialize(&0x95313b0c_u32)?,
             Update::EditChannelMessage { .. } => buf.serialize(&0x1b3f4df7_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            Update::PinnedDialogs {
                flags: _,
                ref order,
            } => {
                let mut ser_flags: u32 = 0;

                if order.is_some() {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;

                if let Some(ref value) = order {
                    buf.serialize(value)?;
                }

            },

            Update::ContactRegistered {
                ref user_id,
                ref date,
            } => {
                buf.serialize(user_id)?;
                buf.serialize(date)?;
            },

            Update::UserTyping {
                ref user_id,
                ref action,
            } => {
                buf.serialize(user_id)?;
                buf.serialize(action.as_ref())?;
            },

            Update::ContactLink {
                ref user_id,
                ref my_link,
                ref foreign_link,
            } => {
                buf.serialize(user_id)?;
                buf.serialize(my_link.as_ref())?;
                buf.serialize(foreign_link.as_ref())?;
            },

            Update::DeleteChannelMessages {
                ref channel_id,
                ref messages,
                ref pts,
                ref pts_count,
            } => {
                buf.serialize(channel_id)?;
                buf.serialize(messages)?;
                buf.serialize(pts)?;
                buf.serialize(pts_count)?;
            },

            Update::SavedGifs {
            } => {
            },

            Update::EditMessage {
                ref message,
                ref pts,
                ref pts_count,
            } => {
                buf.serialize(message.as_ref())?;
                buf.serialize(pts)?;
                buf.serialize(pts_count)?;
            },

            Update::DeleteMessages {
                ref messages,
                ref pts,
                ref pts_count,
            } => {
                buf.serialize(messages)?;
                buf.serialize(pts)?;
                buf.serialize(pts_count)?;
            },

            Update::ChannelMessageViews {
                ref channel_id,
                ref id,
                ref views,
            } => {
                buf.serialize(channel_id)?;
                buf.serialize(id)?;
                buf.serialize(views)?;
            },

            Update::BotInlineQuery {
                flags: _,
                ref query_id,
                ref user_id,
                ref query,
                ref geo,
                ref offset,
            } => {
                let mut ser_flags: u32 = 0;

                if geo.is_some() {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(query_id)?;
                buf.serialize(user_id)?;
                buf.serialize(query)?;

                if let Some(ref value) = geo {
                    buf.serialize(value.as_ref())?;
                }

                buf.serialize(offset)?;
            },

            Update::Privacy {
                ref key,
                ref rules,
            } => {
                buf.serialize(key.as_ref())?;
                buf.serialize(rules)?;
            },

            Update::BotWebhookJson {
                ref data,
            } => {
                buf.serialize(data.as_ref())?;
            },

            Update::ReadFeed {
                ref feed_id,
                ref max_position,
            } => {
                buf.serialize(feed_id)?;
                buf.serialize(max_position.as_ref())?;
            },

            Update::ContactsReset {
            } => {
            },

            Update::BotInlineSend {
                flags: _,
                ref user_id,
                ref query,
                ref geo,
                ref id,
                ref msg_id,
            } => {
                let mut ser_flags: u32 = 0;

                if geo.is_some() {
                    ser_flags |= 1 << 0;
                }

                if msg_id.is_some() {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(user_id)?;
                buf.serialize(query)?;

                if let Some(ref value) = geo {
                    buf.serialize(value.as_ref())?;
                }

                buf.serialize(id)?;

                if let Some(ref value) = msg_id {
                    buf.serialize(value.as_ref())?;
                }

            },

            Update::ChannelWebPage {
                ref channel_id,
                ref webpage,
                ref pts,
                ref pts_count,
            } => {
                buf.serialize(channel_id)?;
                buf.serialize(webpage.as_ref())?;
                buf.serialize(pts)?;
                buf.serialize(pts_count)?;
            },

            Update::ChannelPinnedMessage {
                ref channel_id,
                ref id,
            } => {
                buf.serialize(channel_id)?;
                buf.serialize(id)?;
            },

            Update::DialogPinned {
                flags: _,
                ref pinned,
                ref peer,
            } => {
                let mut ser_flags: u32 = 0;

                if pinned {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(peer.as_ref())?;
            },

            Update::BotShippingQuery {
                ref query_id,
                ref user_id,
                ref payload,
                ref shipping_address,
            } => {
                buf.serialize(query_id)?;
                buf.serialize(user_id)?;
                buf.serialize(payload)?;
                buf.serialize(shipping_address.as_ref())?;
            },

            Update::ChatUserTyping {
                ref chat_id,
                ref user_id,
                ref action,
            } => {
                buf.serialize(chat_id)?;
                buf.serialize(user_id)?;
                buf.serialize(action.as_ref())?;
            },

            Update::BotWebhookJsonquery {
                ref query_id,
                ref data,
                ref timeout,
            } => {
                buf.serialize(query_id)?;
                buf.serialize(data.as_ref())?;
                buf.serialize(timeout)?;
            },

            Update::FavedStickers {
            } => {
            },

            Update::UserStatus {
                ref user_id,
                ref status,
            } => {
                buf.serialize(user_id)?;
                buf.serialize(status.as_ref())?;
            },

            Update::DraftMessage {
                ref peer,
                ref draft,
            } => {
                buf.serialize(peer.as_ref())?;
                buf.serialize(draft.as_ref())?;
            },

            Update::NewChannelMessage {
                ref message,
                ref pts,
                ref pts_count,
            } => {
                buf.serialize(message.as_ref())?;
                buf.serialize(pts)?;
                buf.serialize(pts_count)?;
            },

            Update::UserBlocked {
                ref user_id,
                ref blocked,
            } => {
                buf.serialize(user_id)?;
                buf.serialize(blocked)?;
            },

            Update::ReadChannelOutbox {
                ref channel_id,
                ref max_id,
            } => {
                buf.serialize(channel_id)?;
                buf.serialize(max_id)?;
            },

            Update::PhoneCall {
                ref phone_call,
            } => {
                buf.serialize(phone_call.as_ref())?;
            },

            Update::ReadFeaturedStickers {
            } => {
            },

            Update::ReadMessagesContents {
                ref messages,
                ref pts,
                ref pts_count,
            } => {
                buf.serialize(messages)?;
                buf.serialize(pts)?;
                buf.serialize(pts_count)?;
            },

            Update::StickerSetsOrder {
                flags: _,
                ref masks,
                ref order,
            } => {
                let mut ser_flags: u32 = 0;

                if masks {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(order)?;
            },

            Update::LangPackTooLong {
            } => {
            },

            Update::ChannelTooLong {
                flags: _,
                ref channel_id,
                ref pts,
            } => {
                let mut ser_flags: u32 = 0;

                if pts.is_some() {
                    ser_flags |= 1 << 0;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(channel_id)?;

                if let Some(ref value) = pts {
                    buf.serialize(value)?;
                }

            },

            Update::RecentStickers {
            } => {
            },

            Update::NotifySettings {
                ref peer,
                ref notify_settings,
            } => {
                buf.serialize(peer.as_ref())?;
                buf.serialize(notify_settings.as_ref())?;
            },

            Update::ReadHistoryOutbox {
                ref peer,
                ref max_id,
                ref pts,
                ref pts_count,
            } => {
                buf.serialize(peer.as_ref())?;
                buf.serialize(max_id)?;
                buf.serialize(pts)?;
                buf.serialize(pts_count)?;
            },

            Update::Config {
            } => {
            },

            Update::WebPage {
                ref webpage,
                ref pts,
                ref pts_count,
            } => {
                buf.serialize(webpage.as_ref())?;
                buf.serialize(pts)?;
                buf.serialize(pts_count)?;
            },

            Update::NewMessage {
                ref message,
                ref pts,
                ref pts_count,
            } => {
                buf.serialize(message.as_ref())?;
                buf.serialize(pts)?;
                buf.serialize(pts_count)?;
            },

            Update::EncryptedMessagesRead {
                ref chat_id,
                ref max_date,
                ref date,
            } => {
                buf.serialize(chat_id)?;
                buf.serialize(max_date)?;
                buf.serialize(date)?;
            },

            Update::DcOptions {
                ref dc_options,
            } => {
                buf.serialize(dc_options)?;
            },

            Update::ServiceNotification {
                flags: _,
                ref popup,
                ref inbox_date,
                ref type_,
                ref message,
                ref media,
                ref entities,
            } => {
                let mut ser_flags: u32 = 0;

                if popup {
                    ser_flags |= 1 << 0;
                }

                if inbox_date.is_some() {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;

                if let Some(ref value) = inbox_date {
                    buf.serialize(value)?;
                }

                buf.serialize(type_)?;
                buf.serialize(message)?;
                buf.serialize(media.as_ref())?;
                buf.serialize(entities)?;
            },

            Update::EncryptedChatTyping {
                ref chat_id,
            } => {
                buf.serialize(chat_id)?;
            },

            Update::ReadChannelInbox {
                ref channel_id,
                ref max_id,
            } => {
                buf.serialize(channel_id)?;
                buf.serialize(max_id)?;
            },

            Update::MessageId {
                ref id,
                ref random_id,
            } => {
                buf.serialize(id)?;
                buf.serialize(random_id)?;
            },

            Update::LangPack {
                ref difference,
            } => {
                buf.serialize(difference.as_ref())?;
            },

            Update::ChatParticipantAdmin {
                ref chat_id,
                ref user_id,
                ref is_admin,
                ref version,
            } => {
                buf.serialize(chat_id)?;
                buf.serialize(user_id)?;
                buf.serialize(is_admin)?;
                buf.serialize(version)?;
            },

            Update::UserName {
                ref user_id,
                ref first_name,
                ref last_name,
                ref username,
            } => {
                buf.serialize(user_id)?;
                buf.serialize(first_name)?;
                buf.serialize(last_name)?;
                buf.serialize(username)?;
            },

            Update::ChatParticipantAdd {
                ref chat_id,
                ref user_id,
                ref inviter_id,
                ref date,
                ref version,
            } => {
                buf.serialize(chat_id)?;
                buf.serialize(user_id)?;
                buf.serialize(inviter_id)?;
                buf.serialize(date)?;
                buf.serialize(version)?;
            },

            Update::Encryption {
                ref chat,
                ref date,
            } => {
                buf.serialize(chat.as_ref())?;
                buf.serialize(date)?;
            },

            Update::BotPrecheckoutQuery {
                flags: _,
                ref query_id,
                ref user_id,
                ref payload,
                ref info,
                ref shipping_option_id,
                ref currency,
                ref total_amount,
            } => {
                let mut ser_flags: u32 = 0;

                if info.is_some() {
                    ser_flags |= 1 << 0;
                }

                if shipping_option_id.is_some() {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(query_id)?;
                buf.serialize(user_id)?;
                buf.serialize(payload)?;

                if let Some(ref value) = info {
                    buf.serialize(value.as_ref())?;
                }


                if let Some(ref value) = shipping_option_id {
                    buf.serialize(value)?;
                }

                buf.serialize(currency)?;
                buf.serialize(total_amount)?;
            },

            Update::NewEncryptedMessage {
                ref message,
                ref qts,
            } => {
                buf.serialize(message.as_ref())?;
                buf.serialize(qts)?;
            },

            Update::Channel {
                ref channel_id,
            } => {
                buf.serialize(channel_id)?;
            },

            Update::NewStickerSet {
                ref stickerset,
            } => {
                buf.serialize(stickerset.as_ref())?;
            },

            Update::ReadHistoryInbox {
                ref peer,
                ref max_id,
                ref pts,
                ref pts_count,
            } => {
                buf.serialize(peer.as_ref())?;
                buf.serialize(max_id)?;
                buf.serialize(pts)?;
                buf.serialize(pts_count)?;
            },

            Update::ChannelAvailableMessages {
                ref channel_id,
                ref available_min_id,
            } => {
                buf.serialize(channel_id)?;
                buf.serialize(available_min_id)?;
            },

            Update::ChatAdmins {
                ref chat_id,
                ref enabled,
                ref version,
            } => {
                buf.serialize(chat_id)?;
                buf.serialize(enabled)?;
                buf.serialize(version)?;
            },

            Update::UserPhone {
                ref user_id,
                ref phone,
            } => {
                buf.serialize(user_id)?;
                buf.serialize(phone)?;
            },

            Update::BotCallbackQuery {
                flags: _,
                ref query_id,
                ref user_id,
                ref peer,
                ref msg_id,
                ref chat_instance,
                ref data,
                ref game_short_name,
            } => {
                let mut ser_flags: u32 = 0;

                if data.is_some() {
                    ser_flags |= 1 << 0;
                }

                if game_short_name.is_some() {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(query_id)?;
                buf.serialize(user_id)?;
                buf.serialize(peer.as_ref())?;
                buf.serialize(msg_id)?;
                buf.serialize(chat_instance)?;

                if let Some(ref value) = data {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = game_short_name {
                    buf.serialize(value)?;
                }

            },

            Update::ChatParticipantDelete {
                ref chat_id,
                ref user_id,
                ref version,
            } => {
                buf.serialize(chat_id)?;
                buf.serialize(user_id)?;
                buf.serialize(version)?;
            },

            Update::InlineBotCallbackQuery {
                flags: _,
                ref query_id,
                ref user_id,
                ref msg_id,
                ref chat_instance,
                ref data,
                ref game_short_name,
            } => {
                let mut ser_flags: u32 = 0;

                if data.is_some() {
                    ser_flags |= 1 << 0;
                }

                if game_short_name.is_some() {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(query_id)?;
                buf.serialize(user_id)?;
                buf.serialize(msg_id.as_ref())?;
                buf.serialize(chat_instance)?;

                if let Some(ref value) = data {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = game_short_name {
                    buf.serialize(value)?;
                }

            },

            Update::ChatParticipants {
                ref participants,
            } => {
                buf.serialize(participants.as_ref())?;
            },

            Update::StickerSets {
            } => {
            },

            Update::PtsChanged {
            } => {
            },

            Update::ChannelReadMessagesContents {
                ref channel_id,
                ref messages,
            } => {
                buf.serialize(channel_id)?;
                buf.serialize(messages)?;
            },

            Update::UserPhoto {
                ref user_id,
                ref date,
                ref photo,
                ref previous,
            } => {
                buf.serialize(user_id)?;
                buf.serialize(date)?;
                buf.serialize(photo.as_ref())?;
                buf.serialize(previous)?;
            },

            Update::EditChannelMessage {
                ref message,
                ref pts,
                ref pts_count,
            } => {
                buf.serialize(message.as_ref())?;
                buf.serialize(pts)?;
                buf.serialize(pts_count)?;
            },
        }

        Ok(())
    }
}

impl Serializable for PaymentsSavedInfo {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xfb8fe43c_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.has_saved_credentials {
            ser_flags |= 1 << 1;
        }

        if self.saved_info.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;

        if let Some(ref value) = self.saved_info {
            buf.serialize(value.as_ref())?;
        }


        Ok(())
    }
 }

impl Serializable for MessagesDhConfig {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             MessagesDhConfig::NotModified { .. } => buf.serialize(&0xc0e24635_u32)?,
             MessagesDhConfig::Config { .. } => buf.serialize(&0x2c221edd_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            MessagesDhConfig::NotModified {
                ref random,
            } => {
                buf.serialize(random)?;
            },

            MessagesDhConfig::Config {
                ref g,
                ref p,
                ref version,
                ref random,
            } => {
                buf.serialize(g)?;
                buf.serialize(p)?;
                buf.serialize(version)?;
                buf.serialize(random)?;
            },
        }

        Ok(())
    }
}

impl Serializable for MessageFwdHeader {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x559ebe6d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.from_id.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.channel_id.is_some() {
            ser_flags |= 1 << 1;
        }

        if self.channel_post.is_some() {
            ser_flags |= 1 << 2;
        }

        if self.post_author.is_some() {
            ser_flags |= 1 << 3;
        }

        if self.saved_from_peer.is_some() {
            ser_flags |= 1 << 4;
        }

        if self.saved_from_msg_id.is_some() {
            ser_flags |= 1 << 4;
        }

        buf.serialize(&ser_flags)?;

        if let Some(ref value) = self.from_id {
            buf.serialize(value)?;
        }

        buf.serialize(&self.date)?;

        if let Some(ref value) = self.channel_id {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.channel_post {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.post_author {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.saved_from_peer {
            buf.serialize(value.as_ref())?;
        }


        if let Some(ref value) = self.saved_from_msg_id {
            buf.serialize(value)?;
        }


        Ok(())
    }
 }

impl Serializable for ContactsContacts {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ContactsContacts::Contacts { .. } => buf.serialize(&0xeae87e42_u32)?,
             ContactsContacts::ContactsNotModified { .. } => buf.serialize(&0xb74ba9d2_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ContactsContacts::Contacts {
                ref contacts,
                ref saved_count,
                ref users,
            } => {
                buf.serialize(contacts)?;
                buf.serialize(saved_count)?;
                buf.serialize(users)?;
            },

            ContactsContacts::ContactsNotModified {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for ExportedMessageLink {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x5dab1af4_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.link)?;
        buf.serialize(&self.html)?;

        Ok(())
    }
 }

impl Serializable for Dialog {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             Dialog::Feed { .. } => buf.serialize(&0x36086d42_u32)?,
             Dialog::Dialog { .. } => buf.serialize(&0xe4def5db_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            Dialog::Feed {
                flags: _,
                ref pinned,
                ref peer,
                ref top_message,
                ref feed_id,
                ref feed_other_channels,
                ref read_max_position,
                ref unread_count,
                ref unread_muted_count,
            } => {
                let mut ser_flags: u32 = 0;

                if pinned {
                    ser_flags |= 1 << 2;
                }

                if read_max_position.is_some() {
                    ser_flags |= 1 << 3;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(peer.as_ref())?;
                buf.serialize(top_message)?;
                buf.serialize(feed_id)?;
                buf.serialize(feed_other_channels)?;

                if let Some(ref value) = read_max_position {
                    buf.serialize(value.as_ref())?;
                }

                buf.serialize(unread_count)?;
                buf.serialize(unread_muted_count)?;
            },

            Dialog::Dialog {
                flags: _,
                ref pinned,
                ref peer,
                ref top_message,
                ref read_inbox_max_id,
                ref read_outbox_max_id,
                ref unread_count,
                ref unread_mentions_count,
                ref notify_settings,
                ref pts,
                ref draft,
            } => {
                let mut ser_flags: u32 = 0;

                if pinned {
                    ser_flags |= 1 << 2;
                }

                if pts.is_some() {
                    ser_flags |= 1 << 0;
                }

                if draft.is_some() {
                    ser_flags |= 1 << 1;
                }

                buf.serialize(&ser_flags)?;
                buf.serialize(peer.as_ref())?;
                buf.serialize(top_message)?;
                buf.serialize(read_inbox_max_id)?;
                buf.serialize(read_outbox_max_id)?;
                buf.serialize(unread_count)?;
                buf.serialize(unread_mentions_count)?;
                buf.serialize(notify_settings.as_ref())?;

                if let Some(ref value) = pts {
                    buf.serialize(value)?;
                }


                if let Some(ref value) = draft {
                    buf.serialize(value.as_ref())?;
                }

            },
        }

        Ok(())
    }
}

impl Serializable for PQInnerData {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x83c95aec_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.pq)?;
        buf.serialize(&self.p)?;
        buf.serialize(&self.q)?;
        buf.serialize(&self.nonce)?;
        buf.serialize(&self.server_nonce)?;
        buf.serialize(&self.new_nonce)?;

        Ok(())
    }
 }

impl Serializable for PeerNotifyEvents {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             PeerNotifyEvents::All { .. } => buf.serialize(&0x6d1ded88_u32)?,
             PeerNotifyEvents::Empty { .. } => buf.serialize(&0xadd53cb3_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            PeerNotifyEvents::All {
            } => {
            },

            PeerNotifyEvents::Empty {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for ContactLink {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             ContactLink::HasPhone { .. } => buf.serialize(&0x268f3f59_u32)?,
             ContactLink::Contact { .. } => buf.serialize(&0xd502c2d0_u32)?,
             ContactLink::Unknown { .. } => buf.serialize(&0x5f4f9247_u32)?,
             ContactLink::None { .. } => buf.serialize(&0xfeedd3ad_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            ContactLink::HasPhone {
            } => {
            },

            ContactLink::Contact {
            } => {
            },

            ContactLink::Unknown {
            } => {
            },

            ContactLink::None {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for DestroyAuthKeyRes {
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {
             DestroyAuthKeyRes::Ok { .. } => buf.serialize(&0xf660e1d4_u32)?,
             DestroyAuthKeyRes::None { .. } => buf.serialize(&0x0a9f2259_u32)?,
             DestroyAuthKeyRes::Fail { .. } => buf.serialize(&0xea109b13_u32)?,
        };

        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        match self {

            DestroyAuthKeyRes::Ok {
            } => {
            },

            DestroyAuthKeyRes::None {
            } => {
            },

            DestroyAuthKeyRes::Fail {
            } => {
            },
        }

        Ok(())
    }
}

impl Serializable for InlineBotSwitchPm {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3c20629f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.text)?;
        buf.serialize(&self.start_param)?;

        Ok(())
    }
 }

impl Serializable for Contact {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf911c994_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.user_id)?;
        buf.serialize(&self.mutual)?;

        Ok(())
    }
 }

