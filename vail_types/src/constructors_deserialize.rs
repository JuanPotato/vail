impl Deserializable for ChannelAdminRights {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x5d7ceba5);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChannelAdminRights> {
                let flags = buf.deserialize()?;

        let change_info = flags & 1 << 0 != 0;

        let post_messages = flags & 1 << 1 != 0;

        let edit_messages = flags & 1 << 2 != 0;

        let delete_messages = flags & 1 << 3 != 0;

        let ban_users = flags & 1 << 4 != 0;

        let invite_users = flags & 1 << 5 != 0;

        let invite_link = flags & 1 << 6 != 0;

        let pin_messages = flags & 1 << 7 != 0;

        let add_admins = flags & 1 << 9 != 0;

        let manage_call = flags & 1 << 10 != 0;

        Ok(ChannelAdminRights {
            flags,
            change_info,
            post_messages,
            edit_messages,
            delete_messages,
            ban_users,
            invite_users,
            invite_link,
            pin_messages,
            add_admins,
            manage_call,
        })
    }
}

impl Deserializable for FeedPosition {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x5059dc73);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<FeedPosition> {
                let date = buf.deserialize()?;
        let peer = Box::new(buf.deserialize()?);
        let id = buf.deserialize()?;

        Ok(FeedPosition {
            date,
            peer,
            id,
        })
    }
}

impl Deserializable for ContactsResolvedPeer {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x7f077ad9);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ContactsResolvedPeer> {
                let peer = Box::new(buf.deserialize()?);
        let chats = buf.deserialize()?;
        let users = buf.deserialize()?;

        Ok(ContactsResolvedPeer {
            peer,
            chats,
            users,
        })
    }
}

impl Deserializable for PrivacyKey {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PrivacyKey> {
        match id {
            0x500e6dfa_u32 => {
            
                Ok(PrivacyKey::ChatInvite {
                })
            },

            0x3d662b7b_u32 => {
            
                Ok(PrivacyKey::PhoneCall {
                })
            },

            0xbc2eab30_u32 => {
            
                Ok(PrivacyKey::StatusTimestamp {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputPrivacyKey {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputPrivacyKey> {
        match id {
            0xbdfb0426_u32 => {
            
                Ok(InputPrivacyKey::ChatInvite {
                })
            },

            0x4f96cb18_u32 => {
            
                Ok(InputPrivacyKey::StatusTimestamp {
                })
            },

            0xfabadc5f_u32 => {
            
                Ok(InputPrivacyKey::PhoneCall {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputWebFileLocation {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xc239d686);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputWebFileLocation> {
                let url = buf.deserialize()?;
        let access_hash = buf.deserialize()?;

        Ok(InputWebFileLocation {
            url,
            access_hash,
        })
    }
}

impl Deserializable for MessagesFilter {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesFilter> {
        match id {
            0x7ef0dd87_u32 => {
            
                Ok(MessagesFilter::InputUrl {
                })
            },

            0x7a7c17a4_u32 => {
            
                Ok(MessagesFilter::InputRoundVoice {
                })
            },

            0x57e2f66c_u32 => {
            
                Ok(MessagesFilter::InputEmpty {
                })
            },

            0xc1f8e69a_u32 => {
            
                Ok(MessagesFilter::InputMyMentions {
                })
            },

            0x3a20ecb8_u32 => {
            
                Ok(MessagesFilter::InputChatPhotos {
                })
            },

            0xe062db83_u32 => {
            
                Ok(MessagesFilter::InputContacts {
                })
            },

            0x9609a51c_u32 => {
            
                Ok(MessagesFilter::InputPhotos {
                })
            },

            0x56e9f0e4_u32 => {
            
                Ok(MessagesFilter::InputPhotoVideo {
                })
            },

            0x9eddf188_u32 => {
            
                Ok(MessagesFilter::InputDocument {
                })
            },

            0x80c99768_u32 => {
                let flags = buf.deserialize()?;

                let missed = flags & 1 << 0 != 0;
            
                Ok(MessagesFilter::InputPhoneCalls {
                    flags,
                    missed,
                })
            },

            0x50f5c392_u32 => {
            
                Ok(MessagesFilter::InputVoice {
                })
            },

            0xb549da53_u32 => {
            
                Ok(MessagesFilter::InputRoundVideo {
                })
            },

            0x9fc00e65_u32 => {
            
                Ok(MessagesFilter::InputVideo {
                })
            },

            0xffc86587_u32 => {
            
                Ok(MessagesFilter::InputGif {
                })
            },

            0xe7026d0d_u32 => {
            
                Ok(MessagesFilter::InputGeo {
                })
            },

            0x3751b49e_u32 => {
            
                Ok(MessagesFilter::InputMusic {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputChatPhoto {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputChatPhoto> {
        match id {
            0x927c55b4_u32 => {
                let file = Box::new(buf.deserialize()?);
            
                Ok(InputChatPhoto::Uploaded {
                    file,
                })
            },

            0x8953ad37_u32 => {
                let id = Box::new(buf.deserialize()?);
            
                Ok(InputChatPhoto::Photo {
                    id,
                })
            },

            0x1ca48f57_u32 => {
            
                Ok(InputChatPhoto::Empty {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for Dialog {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Dialog> {
        match id {
            0x36086d42_u32 => {
                let flags = buf.deserialize()?;

                let pinned = flags & 1 << 2 != 0;
                let peer = Box::new(buf.deserialize()?);
                let top_message = buf.deserialize()?;
                let feed_id = buf.deserialize()?;
                let feed_other_channels = buf.deserialize()?;

                let read_max_position = if flags & 1 << 3 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
                let unread_count = buf.deserialize()?;
                let unread_muted_count = buf.deserialize()?;
            
                Ok(Dialog::Feed {
                    flags,
                    pinned,
                    peer,
                    top_message,
                    feed_id,
                    feed_other_channels,
                    read_max_position,
                    unread_count,
                    unread_muted_count,
                })
            },

            0xe4def5db_u32 => {
                let flags = buf.deserialize()?;

                let pinned = flags & 1 << 2 != 0;
                let peer = Box::new(buf.deserialize()?);
                let top_message = buf.deserialize()?;
                let read_inbox_max_id = buf.deserialize()?;
                let read_outbox_max_id = buf.deserialize()?;
                let unread_count = buf.deserialize()?;
                let unread_mentions_count = buf.deserialize()?;
                let notify_settings = Box::new(buf.deserialize()?);

                let pts = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let draft = if flags & 1 << 1 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(Dialog::Dialog {
                    flags,
                    pinned,
                    peer,
                    top_message,
                    read_inbox_max_id,
                    read_outbox_max_id,
                    unread_count,
                    unread_mentions_count,
                    notify_settings,
                    pts,
                    draft,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for AuthSentCodeType {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<AuthSentCodeType> {
        match id {
            0xc000bba2_u32 => {
                let length = buf.deserialize()?;
            
                Ok(AuthSentCodeType::Sms {
                    length,
                })
            },

            0x5353e5a7_u32 => {
                let length = buf.deserialize()?;
            
                Ok(AuthSentCodeType::Call {
                    length,
                })
            },

            0x3dbb5986_u32 => {
                let length = buf.deserialize()?;
            
                Ok(AuthSentCodeType::App {
                    length,
                })
            },

            0xab03c6d9_u32 => {
                let pattern = buf.deserialize()?;
            
                Ok(AuthSentCodeType::FlashCall {
                    pattern,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputMedia {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputMedia> {
        match id {
            0x9664f57f_u32 => {
            
                Ok(InputMedia::Empty {
                })
            },

            0x7b1a118f_u32 => {
                let geo_point = Box::new(buf.deserialize()?);
                let period = buf.deserialize()?;
            
                Ok(InputMedia::GeoLive {
                    geo_point,
                    period,
                })
            },

            0x0922aec1_u32 => {
                let flags = buf.deserialize()?;
                let url = buf.deserialize()?;
                let caption = buf.deserialize()?;

                let ttl_seconds = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(InputMedia::PhotoExternal {
                    flags,
                    url,
                    caption,
                    ttl_seconds,
                })
            },

            0xd33f43f3_u32 => {
                let id = Box::new(buf.deserialize()?);
            
                Ok(InputMedia::Game {
                    id,
                })
            },

            0x4843b0fd_u32 => {
                let url = buf.deserialize()?;
                let q = buf.deserialize()?;
            
                Ok(InputMedia::GifExternal {
                    url,
                    q,
                })
            },

            0x2f37e231_u32 => {
                let flags = buf.deserialize()?;
                let file = Box::new(buf.deserialize()?);
                let caption = buf.deserialize()?;

                let stickers = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let ttl_seconds = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(InputMedia::UploadedPhoto {
                    flags,
                    file,
                    caption,
                    stickers,
                    ttl_seconds,
                })
            },

            0xf9c44144_u32 => {
                let geo_point = Box::new(buf.deserialize()?);
            
                Ok(InputMedia::GeoPoint {
                    geo_point,
                })
            },

            0xa6e45987_u32 => {
                let phone_number = buf.deserialize()?;
                let first_name = buf.deserialize()?;
                let last_name = buf.deserialize()?;
            
                Ok(InputMedia::Contact {
                    phone_number,
                    first_name,
                    last_name,
                })
            },

            0x5acb668e_u32 => {
                let flags = buf.deserialize()?;
                let id = Box::new(buf.deserialize()?);
                let caption = buf.deserialize()?;

                let ttl_seconds = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(InputMedia::Document {
                    flags,
                    id,
                    caption,
                    ttl_seconds,
                })
            },

            0xb6f74335_u32 => {
                let flags = buf.deserialize()?;
                let url = buf.deserialize()?;
                let caption = buf.deserialize()?;

                let ttl_seconds = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(InputMedia::DocumentExternal {
                    flags,
                    url,
                    caption,
                    ttl_seconds,
                })
            },

            0xf4e096c3_u32 => {
                let flags = buf.deserialize()?;
                let title = buf.deserialize()?;
                let description = buf.deserialize()?;

                let photo = if flags & 1 << 0 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
                let invoice = Box::new(buf.deserialize()?);
                let payload = buf.deserialize()?;
                let provider = buf.deserialize()?;
                let provider_data = Box::new(buf.deserialize()?);
                let start_param = buf.deserialize()?;
            
                Ok(InputMedia::Invoice {
                    flags,
                    title,
                    description,
                    photo,
                    invoice,
                    payload,
                    provider,
                    provider_data,
                    start_param,
                })
            },

            0xc13d1c11_u32 => {
                let geo_point = Box::new(buf.deserialize()?);
                let title = buf.deserialize()?;
                let address = buf.deserialize()?;
                let provider = buf.deserialize()?;
                let venue_id = buf.deserialize()?;
                let venue_type = buf.deserialize()?;
            
                Ok(InputMedia::Venue {
                    geo_point,
                    title,
                    address,
                    provider,
                    venue_id,
                    venue_type,
                })
            },

            0x81fa373a_u32 => {
                let flags = buf.deserialize()?;
                let id = Box::new(buf.deserialize()?);
                let caption = buf.deserialize()?;

                let ttl_seconds = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(InputMedia::Photo {
                    flags,
                    id,
                    caption,
                    ttl_seconds,
                })
            },

            0xe39621fd_u32 => {
                let flags = buf.deserialize()?;

                let nosound_video = flags & 1 << 3 != 0;
                let file = Box::new(buf.deserialize()?);

                let thumb = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
                let mime_type = buf.deserialize()?;
                let attributes = buf.deserialize()?;
                let caption = buf.deserialize()?;

                let stickers = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let ttl_seconds = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(InputMedia::UploadedDocument {
                    flags,
                    nosound_video,
                    file,
                    thumb,
                    mime_type,
                    attributes,
                    caption,
                    stickers,
                    ttl_seconds,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ServerDhParams {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ServerDhParams> {
        match id {
            0xd0e8075c_u32 => {
                let nonce = buf.deserialize()?;
                let server_nonce = buf.deserialize()?;
                let encrypted_answer = buf.deserialize()?;
            
                Ok(ServerDhParams::Ok {
                    nonce,
                    server_nonce,
                    encrypted_answer,
                })
            },

            0x79cb045d_u32 => {
                let nonce = buf.deserialize()?;
                let server_nonce = buf.deserialize()?;
                let new_nonce_hash = buf.deserialize()?;
            
                Ok(ServerDhParams::Fail {
                    nonce,
                    server_nonce,
                    new_nonce_hash,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for Chat {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Chat> {
        match id {
            0xc88974ac_u32 => {
                let flags = buf.deserialize()?;

                let creator = flags & 1 << 0 != 0;

                let left = flags & 1 << 2 != 0;

                let editor = flags & 1 << 3 != 0;

                let broadcast = flags & 1 << 5 != 0;

                let verified = flags & 1 << 7 != 0;

                let megagroup = flags & 1 << 8 != 0;

                let restricted = flags & 1 << 9 != 0;

                let democracy = flags & 1 << 10 != 0;

                let signatures = flags & 1 << 11 != 0;

                let min = flags & 1 << 12 != 0;
                let id = buf.deserialize()?;

                let access_hash = if flags & 1 << 13 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let title = buf.deserialize()?;

                let username = if flags & 1 << 6 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let photo = Box::new(buf.deserialize()?);
                let date = buf.deserialize()?;
                let version = buf.deserialize()?;

                let restriction_reason = if flags & 1 << 9 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let admin_rights = if flags & 1 << 14 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let banned_rights = if flags & 1 << 15 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let participants_count = if flags & 1 << 17 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let feed_id = if flags & 1 << 18 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(Chat::Channel {
                    flags,
                    creator,
                    left,
                    editor,
                    broadcast,
                    verified,
                    megagroup,
                    restricted,
                    democracy,
                    signatures,
                    min,
                    id,
                    access_hash,
                    title,
                    username,
                    photo,
                    date,
                    version,
                    restriction_reason,
                    admin_rights,
                    banned_rights,
                    participants_count,
                    feed_id,
                })
            },

            0x289da732_u32 => {
                let flags = buf.deserialize()?;

                let broadcast = flags & 1 << 5 != 0;

                let megagroup = flags & 1 << 8 != 0;
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
                let title = buf.deserialize()?;

                let until_date = if flags & 1 << 16 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(Chat::ChannelForbidden {
                    flags,
                    broadcast,
                    megagroup,
                    id,
                    access_hash,
                    title,
                    until_date,
                })
            },

            0xd91cdd54_u32 => {
                let flags = buf.deserialize()?;

                let creator = flags & 1 << 0 != 0;

                let kicked = flags & 1 << 1 != 0;

                let left = flags & 1 << 2 != 0;

                let admins_enabled = flags & 1 << 3 != 0;

                let admin = flags & 1 << 4 != 0;

                let deactivated = flags & 1 << 5 != 0;
                let id = buf.deserialize()?;
                let title = buf.deserialize()?;
                let photo = Box::new(buf.deserialize()?);
                let participants_count = buf.deserialize()?;
                let date = buf.deserialize()?;
                let version = buf.deserialize()?;

                let migrated_to = if flags & 1 << 6 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(Chat::Chat {
                    flags,
                    creator,
                    kicked,
                    left,
                    admins_enabled,
                    admin,
                    deactivated,
                    id,
                    title,
                    photo,
                    participants_count,
                    date,
                    version,
                    migrated_to,
                })
            },

            0x07328bdb_u32 => {
                let id = buf.deserialize()?;
                let title = buf.deserialize()?;
            
                Ok(Chat::Forbidden {
                    id,
                    title,
                })
            },

            0x9ba2d800_u32 => {
                let id = buf.deserialize()?;
            
                Ok(Chat::Empty {
                    id,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for Error {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xc4b9f9bb);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Error> {
                let code = buf.deserialize()?;
        let text = buf.deserialize()?;

        Ok(Error {
            code,
            text,
        })
    }
}

impl Deserializable for Config {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x9c840964);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Config> {
                let flags = buf.deserialize()?;

        let phonecalls_enabled = flags & 1 << 1 != 0;

        let default_p2p_contacts = flags & 1 << 3 != 0;
        let date = buf.deserialize()?;
        let expires = buf.deserialize()?;
        let test_mode = buf.deserialize()?;
        let this_dc = buf.deserialize()?;
        let dc_options = buf.deserialize()?;
        let chat_size_max = buf.deserialize()?;
        let megagroup_size_max = buf.deserialize()?;
        let forwarded_count_max = buf.deserialize()?;
        let online_update_period_ms = buf.deserialize()?;
        let offline_blur_timeout_ms = buf.deserialize()?;
        let offline_idle_timeout_ms = buf.deserialize()?;
        let online_cloud_timeout_ms = buf.deserialize()?;
        let notify_cloud_delay_ms = buf.deserialize()?;
        let notify_default_delay_ms = buf.deserialize()?;
        let chat_big_size = buf.deserialize()?;
        let push_chat_period_ms = buf.deserialize()?;
        let push_chat_limit = buf.deserialize()?;
        let saved_gifs_limit = buf.deserialize()?;
        let edit_time_limit = buf.deserialize()?;
        let rating_e_decay = buf.deserialize()?;
        let stickers_recent_limit = buf.deserialize()?;
        let stickers_faved_limit = buf.deserialize()?;
        let channels_read_media_period = buf.deserialize()?;

        let tmp_sessions = if flags & 1 << 0 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };
        let pinned_dialogs_count_max = buf.deserialize()?;
        let call_receive_timeout_ms = buf.deserialize()?;
        let call_ring_timeout_ms = buf.deserialize()?;
        let call_connect_timeout_ms = buf.deserialize()?;
        let call_packet_timeout_ms = buf.deserialize()?;
        let me_url_prefix = buf.deserialize()?;

        let suggested_lang_code = if flags & 1 << 2 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        let lang_pack_version = if flags & 1 << 2 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };
        let disabled_features = buf.deserialize()?;

        Ok(Config {
            flags,
            phonecalls_enabled,
            default_p2p_contacts,
            date,
            expires,
            test_mode,
            this_dc,
            dc_options,
            chat_size_max,
            megagroup_size_max,
            forwarded_count_max,
            online_update_period_ms,
            offline_blur_timeout_ms,
            offline_idle_timeout_ms,
            online_cloud_timeout_ms,
            notify_cloud_delay_ms,
            notify_default_delay_ms,
            chat_big_size,
            push_chat_period_ms,
            push_chat_limit,
            saved_gifs_limit,
            edit_time_limit,
            rating_e_decay,
            stickers_recent_limit,
            stickers_faved_limit,
            channels_read_media_period,
            tmp_sessions,
            pinned_dialogs_count_max,
            call_receive_timeout_ms,
            call_ring_timeout_ms,
            call_connect_timeout_ms,
            call_packet_timeout_ms,
            me_url_prefix,
            suggested_lang_code,
            lang_pack_version,
            disabled_features,
        })
    }
}

impl Deserializable for ChannelAdminLogEventsFilter {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xea107ae4);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChannelAdminLogEventsFilter> {
                let flags = buf.deserialize()?;

        let join = flags & 1 << 0 != 0;

        let leave = flags & 1 << 1 != 0;

        let invite = flags & 1 << 2 != 0;

        let ban = flags & 1 << 3 != 0;

        let unban = flags & 1 << 4 != 0;

        let kick = flags & 1 << 5 != 0;

        let unkick = flags & 1 << 6 != 0;

        let promote = flags & 1 << 7 != 0;

        let demote = flags & 1 << 8 != 0;

        let info = flags & 1 << 9 != 0;

        let settings = flags & 1 << 10 != 0;

        let pinned = flags & 1 << 11 != 0;

        let edit = flags & 1 << 12 != 0;

        let delete = flags & 1 << 13 != 0;

        Ok(ChannelAdminLogEventsFilter {
            flags,
            join,
            leave,
            invite,
            ban,
            unban,
            kick,
            unkick,
            promote,
            demote,
            info,
            settings,
            pinned,
            edit,
            delete,
        })
    }
}

impl Deserializable for DocumentAttribute {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<DocumentAttribute> {
        match id {
            0x9852f9c6_u32 => {
                let flags = buf.deserialize()?;

                let voice = flags & 1 << 10 != 0;
                let duration = buf.deserialize()?;

                let title = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let performer = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let waveform = if flags & 1 << 2 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(DocumentAttribute::Audio {
                    flags,
                    voice,
                    duration,
                    title,
                    performer,
                    waveform,
                })
            },

            0x6c37c15c_u32 => {
                let w = buf.deserialize()?;
                let h = buf.deserialize()?;
            
                Ok(DocumentAttribute::ImageSize {
                    w,
                    h,
                })
            },

            0x11b58939_u32 => {
            
                Ok(DocumentAttribute::Animated {
                })
            },

            0x9801d2f7_u32 => {
            
                Ok(DocumentAttribute::HasStickers {
                })
            },

            0x0ef02ce6_u32 => {
                let flags = buf.deserialize()?;

                let round_message = flags & 1 << 0 != 0;
                let duration = buf.deserialize()?;
                let w = buf.deserialize()?;
                let h = buf.deserialize()?;
            
                Ok(DocumentAttribute::Video {
                    flags,
                    round_message,
                    duration,
                    w,
                    h,
                })
            },

            0x6319d612_u32 => {
                let flags = buf.deserialize()?;

                let mask = flags & 1 << 1 != 0;
                let alt = buf.deserialize()?;
                let stickerset = Box::new(buf.deserialize()?);

                let mask_coords = if flags & 1 << 0 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(DocumentAttribute::Sticker {
                    flags,
                    mask,
                    alt,
                    stickerset,
                    mask_coords,
                })
            },

            0x15590068_u32 => {
                let file_name = buf.deserialize()?;
            
                Ok(DocumentAttribute::Filename {
                    file_name,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ImportedContact {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xd0028438);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ImportedContact> {
                let user_id = buf.deserialize()?;
        let client_id = buf.deserialize()?;

        Ok(ImportedContact {
            user_id,
            client_id,
        })
    }
}

impl Deserializable for ContactsImportedContacts {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x77d01c3b);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ContactsImportedContacts> {
                let imported = buf.deserialize()?;
        let popular_invites = buf.deserialize()?;
        let retry_contacts = buf.deserialize()?;
        let users = buf.deserialize()?;

        Ok(ContactsImportedContacts {
            imported,
            popular_invites,
            retry_contacts,
            users,
        })
    }
}

impl Deserializable for EncryptedFile {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<EncryptedFile> {
        match id {
            0x4a70994c_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
                let size = buf.deserialize()?;
                let dc_id = buf.deserialize()?;
                let key_fingerprint = buf.deserialize()?;
            
                Ok(EncryptedFile::File {
                    id,
                    access_hash,
                    size,
                    dc_id,
                    key_fingerprint,
                })
            },

            0xc21f497e_u32 => {
            
                Ok(EncryptedFile::Empty {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MessagesAffectedMessages {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x84d19185);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesAffectedMessages> {
                let pts = buf.deserialize()?;
        let pts_count = buf.deserialize()?;

        Ok(MessagesAffectedMessages {
            pts,
            pts_count,
        })
    }
}

impl Deserializable for ServerDhInnerData {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xb5890dba);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ServerDhInnerData> {
                let nonce = buf.deserialize()?;
        let server_nonce = buf.deserialize()?;
        let g = buf.deserialize()?;
        let dh_prime = buf.deserialize()?;
        let g_a = buf.deserialize()?;
        let server_time = buf.deserialize()?;

        Ok(ServerDhInnerData {
            nonce,
            server_nonce,
            g,
            dh_prime,
            g_a,
            server_time,
        })
    }
}

impl Deserializable for DraftMessage {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<DraftMessage> {
        match id {
            0xfd8e711f_u32 => {
                let flags = buf.deserialize()?;

                let no_webpage = flags & 1 << 1 != 0;

                let reply_to_msg_id = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let message = buf.deserialize()?;

                let entities = if flags & 1 << 3 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let date = buf.deserialize()?;
            
                Ok(DraftMessage::Message {
                    flags,
                    no_webpage,
                    reply_to_msg_id,
                    message,
                    entities,
                    date,
                })
            },

            0xba4baec5_u32 => {
            
                Ok(DraftMessage::Empty {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for HelpTermsOfService {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xf1ee3e90);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<HelpTermsOfService> {
                let text = buf.deserialize()?;

        Ok(HelpTermsOfService {
            text,
        })
    }
}

impl Deserializable for PaymentsValidatedRequestedInfo {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xd1451883);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PaymentsValidatedRequestedInfo> {
                let flags = buf.deserialize()?;

        let id = if flags & 1 << 0 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        let shipping_options = if flags & 1 << 1 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        Ok(PaymentsValidatedRequestedInfo {
            flags,
            id,
            shipping_options,
        })
    }
}

impl Deserializable for IpPort {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xd433ad73);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<IpPort> {
                let ipv4 = buf.deserialize()?;
        let port = buf.deserialize()?;

        Ok(IpPort {
            ipv4,
            port,
        })
    }
}

impl Deserializable for InputChannel {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputChannel> {
        match id {
            0xee8c1e86_u32 => {
            
                Ok(InputChannel::Empty {
                })
            },

            0xafeb712e_u32 => {
                let channel_id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
            
                Ok(InputChannel::Channel {
                    channel_id,
                    access_hash,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for PaymentCharge {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xea02c27e);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PaymentCharge> {
                let id = buf.deserialize()?;
        let provider_charge_id = buf.deserialize()?;

        Ok(PaymentCharge {
            id,
            provider_charge_id,
        })
    }
}

impl Deserializable for MessageMedia {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessageMedia> {
        match id {
            0x2ec0533f_u32 => {
                let geo = Box::new(buf.deserialize()?);
                let title = buf.deserialize()?;
                let address = buf.deserialize()?;
                let provider = buf.deserialize()?;
                let venue_id = buf.deserialize()?;
                let venue_type = buf.deserialize()?;
            
                Ok(MessageMedia::Venue {
                    geo,
                    title,
                    address,
                    provider,
                    venue_id,
                    venue_type,
                })
            },

            0xa32dd600_u32 => {
                let webpage = Box::new(buf.deserialize()?);
            
                Ok(MessageMedia::WebPage {
                    webpage,
                })
            },

            0x7c4414d3_u32 => {
                let flags = buf.deserialize()?;

                let document = if flags & 1 << 0 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let caption = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let ttl_seconds = if flags & 1 << 2 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(MessageMedia::Document {
                    flags,
                    document,
                    caption,
                    ttl_seconds,
                })
            },

            0x9f84f49e_u32 => {
            
                Ok(MessageMedia::Unsupported {
                })
            },

            0x7c3c2609_u32 => {
                let geo = Box::new(buf.deserialize()?);
                let period = buf.deserialize()?;
            
                Ok(MessageMedia::GeoLive {
                    geo,
                    period,
                })
            },

            0x56e0d474_u32 => {
                let geo = Box::new(buf.deserialize()?);
            
                Ok(MessageMedia::Geo {
                    geo,
                })
            },

            0x5e7d2f39_u32 => {
                let phone_number = buf.deserialize()?;
                let first_name = buf.deserialize()?;
                let last_name = buf.deserialize()?;
                let user_id = buf.deserialize()?;
            
                Ok(MessageMedia::Contact {
                    phone_number,
                    first_name,
                    last_name,
                    user_id,
                })
            },

            0xb5223b0f_u32 => {
                let flags = buf.deserialize()?;

                let photo = if flags & 1 << 0 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let caption = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let ttl_seconds = if flags & 1 << 2 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(MessageMedia::Photo {
                    flags,
                    photo,
                    caption,
                    ttl_seconds,
                })
            },

            0x84551347_u32 => {
                let flags = buf.deserialize()?;

                let shipping_address_requested = flags & 1 << 1 != 0;

                let test = flags & 1 << 3 != 0;
                let title = buf.deserialize()?;
                let description = buf.deserialize()?;

                let photo = if flags & 1 << 0 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let receipt_msg_id = if flags & 1 << 2 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let currency = buf.deserialize()?;
                let total_amount = buf.deserialize()?;
                let start_param = buf.deserialize()?;
            
                Ok(MessageMedia::Invoice {
                    flags,
                    shipping_address_requested,
                    test,
                    title,
                    description,
                    photo,
                    receipt_msg_id,
                    currency,
                    total_amount,
                    start_param,
                })
            },

            0x3ded6320_u32 => {
            
                Ok(MessageMedia::Empty {
                })
            },

            0xfdb19008_u32 => {
                let game = Box::new(buf.deserialize()?);
            
                Ok(MessageMedia::Game {
                    game,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputPeerNotifyEvents {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputPeerNotifyEvents> {
        match id {
            0xf03064d8_u32 => {
            
                Ok(InputPeerNotifyEvents::Empty {
                })
            },

            0xe86a2c74_u32 => {
            
                Ok(InputPeerNotifyEvents::All {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for AuthCheckedPhone {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x811ea28e);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<AuthCheckedPhone> {
                let phone_registered = buf.deserialize()?;

        Ok(AuthCheckedPhone {
            phone_registered,
        })
    }
}

impl Deserializable for User {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<User> {
        match id {
            0x200250ba_u32 => {
                let id = buf.deserialize()?;
            
                Ok(User::Empty {
                    id,
                })
            },

            0x2e13f4c3_u32 => {
                let flags = buf.deserialize()?;

                let self_ = flags & 1 << 10 != 0;

                let contact = flags & 1 << 11 != 0;

                let mutual_contact = flags & 1 << 12 != 0;

                let deleted = flags & 1 << 13 != 0;

                let bot = flags & 1 << 14 != 0;

                let bot_chat_history = flags & 1 << 15 != 0;

                let bot_nochats = flags & 1 << 16 != 0;

                let verified = flags & 1 << 17 != 0;

                let restricted = flags & 1 << 18 != 0;

                let min = flags & 1 << 20 != 0;

                let bot_inline_geo = flags & 1 << 21 != 0;
                let id = buf.deserialize()?;

                let access_hash = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let first_name = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let last_name = if flags & 1 << 2 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let username = if flags & 1 << 3 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let phone = if flags & 1 << 4 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let photo = if flags & 1 << 5 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let status = if flags & 1 << 6 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let bot_info_version = if flags & 1 << 14 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let restriction_reason = if flags & 1 << 18 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let bot_inline_placeholder = if flags & 1 << 19 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let lang_code = if flags & 1 << 22 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(User::User {
                    flags,
                    self_,
                    contact,
                    mutual_contact,
                    deleted,
                    bot,
                    bot_chat_history,
                    bot_nochats,
                    verified,
                    restricted,
                    min,
                    bot_inline_geo,
                    id,
                    access_hash,
                    first_name,
                    last_name,
                    username,
                    phone,
                    photo,
                    status,
                    bot_info_version,
                    restriction_reason,
                    bot_inline_placeholder,
                    lang_code,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for HttpWait {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x9299359f);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<HttpWait> {
                let max_delay = buf.deserialize()?;
        let wait_after = buf.deserialize()?;
        let max_wait = buf.deserialize()?;

        Ok(HttpWait {
            max_delay,
            wait_after,
            max_wait,
        })
    }
}

impl Deserializable for UserStatus {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<UserStatus> {
        match id {
            0x07bf09fc_u32 => {
            
                Ok(UserStatus::LastWeek {
                })
            },

            0x008c703f_u32 => {
                let was_online = buf.deserialize()?;
            
                Ok(UserStatus::Offline {
                    was_online,
                })
            },

            0xedb93949_u32 => {
                let expires = buf.deserialize()?;
            
                Ok(UserStatus::Online {
                    expires,
                })
            },

            0x77ebc742_u32 => {
            
                Ok(UserStatus::LastMonth {
                })
            },

            0x09d05049_u32 => {
            
                Ok(UserStatus::Empty {
                })
            },

            0xe26f42f1_u32 => {
            
                Ok(UserStatus::Recently {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MessagesSavedGifs {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesSavedGifs> {
        match id {
            0xe8025ca2_u32 => {
            
                Ok(MessagesSavedGifs::NotModified {
                })
            },

            0x2e0709a5_u32 => {
                let hash = buf.deserialize()?;
                let gifs = buf.deserialize()?;
            
                Ok(MessagesSavedGifs::Gifs {
                    hash,
                    gifs,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for EncryptedChat {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<EncryptedChat> {
        match id {
            0xc878527e_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
                let date = buf.deserialize()?;
                let admin_id = buf.deserialize()?;
                let participant_id = buf.deserialize()?;
                let g_a = buf.deserialize()?;
            
                Ok(EncryptedChat::Requested {
                    id,
                    access_hash,
                    date,
                    admin_id,
                    participant_id,
                    g_a,
                })
            },

            0xab7ec0a0_u32 => {
                let id = buf.deserialize()?;
            
                Ok(EncryptedChat::Empty {
                    id,
                })
            },

            0x3bf703dc_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
                let date = buf.deserialize()?;
                let admin_id = buf.deserialize()?;
                let participant_id = buf.deserialize()?;
            
                Ok(EncryptedChat::Waiting {
                    id,
                    access_hash,
                    date,
                    admin_id,
                    participant_id,
                })
            },

            0xfa56ce36_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
                let date = buf.deserialize()?;
                let admin_id = buf.deserialize()?;
                let participant_id = buf.deserialize()?;
                let g_a_or_b = buf.deserialize()?;
                let key_fingerprint = buf.deserialize()?;
            
                Ok(EncryptedChat::Chat {
                    id,
                    access_hash,
                    date,
                    admin_id,
                    participant_id,
                    g_a_or_b,
                    key_fingerprint,
                })
            },

            0x13d6dd27_u32 => {
                let id = buf.deserialize()?;
            
                Ok(EncryptedChat::Discarded {
                    id,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for UploadCdnFile {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<UploadCdnFile> {
        match id {
            0xa99fca4f_u32 => {
                let bytes = buf.deserialize()?;
            
                Ok(UploadCdnFile::File {
                    bytes,
                })
            },

            0xeea8e46e_u32 => {
                let request_token = buf.deserialize()?;
            
                Ok(UploadCdnFile::ReuploadNeeded {
                    request_token,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputEncryptedFile {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputEncryptedFile> {
        match id {
            0x5a17b5e5_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
            
                Ok(InputEncryptedFile::File {
                    id,
                    access_hash,
                })
            },

            0x2dc173c8_u32 => {
                let id = buf.deserialize()?;
                let parts = buf.deserialize()?;
                let key_fingerprint = buf.deserialize()?;
            
                Ok(InputEncryptedFile::BigUploaded {
                    id,
                    parts,
                    key_fingerprint,
                })
            },

            0x1837c364_u32 => {
            
                Ok(InputEncryptedFile::Empty {
                })
            },

            0x64bd0306_u32 => {
                let id = buf.deserialize()?;
                let parts = buf.deserialize()?;
                let md5_checksum = buf.deserialize()?;
                let key_fingerprint = buf.deserialize()?;
            
                Ok(InputEncryptedFile::Uploaded {
                    id,
                    parts,
                    md5_checksum,
                    key_fingerprint,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for PeerNotifySettings {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PeerNotifySettings> {
        match id {
            0x70a68512_u32 => {
            
                Ok(PeerNotifySettings::Empty {
                })
            },

            0x9acda4c0_u32 => {
                let flags = buf.deserialize()?;

                let show_previews = flags & 1 << 0 != 0;

                let silent = flags & 1 << 1 != 0;
                let mute_until = buf.deserialize()?;
                let sound = buf.deserialize()?;
            
                Ok(PeerNotifySettings::Settings {
                    flags,
                    show_previews,
                    silent,
                    mute_until,
                    sound,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for RpcError {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x2144ca19);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<RpcError> {
                let error_code = buf.deserialize()?;
        let error_message = buf.deserialize()?;

        Ok(RpcError {
            error_code,
            error_message,
        })
    }
}

impl Deserializable for DialogPeer {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<DialogPeer> {
        match id {
            0xda429411_u32 => {
                let feed_id = buf.deserialize()?;
            
                Ok(DialogPeer::Feed {
                    feed_id,
                })
            },

            0xe56dbf05_u32 => {
                let peer = Box::new(buf.deserialize()?);
            
                Ok(DialogPeer::Peer {
                    peer,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for StickerSetCovered {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<StickerSetCovered> {
        match id {
            0x3407e51b_u32 => {
                let set = Box::new(buf.deserialize()?);
                let covers = buf.deserialize()?;
            
                Ok(StickerSetCovered::Multi {
                    set,
                    covers,
                })
            },

            0x6410a5d2_u32 => {
                let set = Box::new(buf.deserialize()?);
                let cover = Box::new(buf.deserialize()?);
            
                Ok(StickerSetCovered::Covered {
                    set,
                    cover,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for EncryptedMessage {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<EncryptedMessage> {
        match id {
            0xed18c118_u32 => {
                let random_id = buf.deserialize()?;
                let chat_id = buf.deserialize()?;
                let date = buf.deserialize()?;
                let bytes = buf.deserialize()?;
                let file = Box::new(buf.deserialize()?);
            
                Ok(EncryptedMessage::Message {
                    random_id,
                    chat_id,
                    date,
                    bytes,
                    file,
                })
            },

            0x23734b06_u32 => {
                let random_id = buf.deserialize()?;
                let chat_id = buf.deserialize()?;
                let date = buf.deserialize()?;
                let bytes = buf.deserialize()?;
            
                Ok(EncryptedMessage::Service {
                    random_id,
                    chat_id,
                    date,
                    bytes,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for RpcDropAnswer {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<RpcDropAnswer> {
        match id {
            0x5e2ad36e_u32 => {
            
                Ok(RpcDropAnswer::AnswerUnknown {
                })
            },

            0xcd78e586_u32 => {
            
                Ok(RpcDropAnswer::AnswerDroppedRunning {
                })
            },

            0xa43ad8b7_u32 => {
                let msg_id = buf.deserialize()?;
                let seq_no = buf.deserialize()?;
                let bytes = buf.deserialize()?;
            
                Ok(RpcDropAnswer::AnswerDropped {
                    msg_id,
                    seq_no,
                    bytes,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputBotInlineResult {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputBotInlineResult> {
        match id {
            0x2cbbe15a_u32 => {
                let flags = buf.deserialize()?;
                let id = buf.deserialize()?;
                let type_ = buf.deserialize()?;

                let title = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let description = if flags & 1 << 2 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let url = if flags & 1 << 3 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let thumb_url = if flags & 1 << 4 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let content_url = if flags & 1 << 5 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let content_type = if flags & 1 << 5 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let w = if flags & 1 << 6 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let h = if flags & 1 << 6 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let duration = if flags & 1 << 7 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let send_message = Box::new(buf.deserialize()?);
            
                Ok(InputBotInlineResult::Result {
                    flags,
                    id,
                    type_,
                    title,
                    description,
                    url,
                    thumb_url,
                    content_url,
                    content_type,
                    w,
                    h,
                    duration,
                    send_message,
                })
            },

            0x4fa417f2_u32 => {
                let id = buf.deserialize()?;
                let short_name = buf.deserialize()?;
                let send_message = Box::new(buf.deserialize()?);
            
                Ok(InputBotInlineResult::Game {
                    id,
                    short_name,
                    send_message,
                })
            },

            0xa8d864a7_u32 => {
                let id = buf.deserialize()?;
                let type_ = buf.deserialize()?;
                let photo = Box::new(buf.deserialize()?);
                let send_message = Box::new(buf.deserialize()?);
            
                Ok(InputBotInlineResult::Photo {
                    id,
                    type_,
                    photo,
                    send_message,
                })
            },

            0xfff8fdc4_u32 => {
                let flags = buf.deserialize()?;
                let id = buf.deserialize()?;
                let type_ = buf.deserialize()?;

                let title = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let description = if flags & 1 << 2 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let document = Box::new(buf.deserialize()?);
                let send_message = Box::new(buf.deserialize()?);
            
                Ok(InputBotInlineResult::Document {
                    flags,
                    id,
                    type_,
                    title,
                    description,
                    document,
                    send_message,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for PQInnerData {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x83c95aec);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PQInnerData> {
                let pq = buf.deserialize()?;
        let p = buf.deserialize()?;
        let q = buf.deserialize()?;
        let nonce = buf.deserialize()?;
        let server_nonce = buf.deserialize()?;
        let new_nonce = buf.deserialize()?;

        Ok(PQInnerData {
            pq,
            p,
            q,
            nonce,
            server_nonce,
            new_nonce,
        })
    }
}

impl Deserializable for ChannelBannedRights {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x58cf4249);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChannelBannedRights> {
                let flags = buf.deserialize()?;

        let view_messages = flags & 1 << 0 != 0;

        let send_messages = flags & 1 << 1 != 0;

        let send_media = flags & 1 << 2 != 0;

        let send_stickers = flags & 1 << 3 != 0;

        let send_gifs = flags & 1 << 4 != 0;

        let send_games = flags & 1 << 5 != 0;

        let send_inline = flags & 1 << 6 != 0;

        let embed_links = flags & 1 << 7 != 0;
        let until_date = buf.deserialize()?;

        Ok(ChannelBannedRights {
            flags,
            view_messages,
            send_messages,
            send_media,
            send_stickers,
            send_gifs,
            send_games,
            send_inline,
            embed_links,
            until_date,
        })
    }
}

impl Deserializable for MsgsStateReq {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xda69fb52);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MsgsStateReq> {
                let msg_ids = buf.deserialize()?;

        Ok(MsgsStateReq {
            msg_ids,
        })
    }
}

impl Deserializable for TopPeerCategoryPeers {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xfb834291);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<TopPeerCategoryPeers> {
                let category = Box::new(buf.deserialize()?);
        let count = buf.deserialize()?;
        let peers = buf.deserialize()?;

        Ok(TopPeerCategoryPeers {
            category,
            count,
            peers,
        })
    }
}

impl Deserializable for PageBlock {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PageBlock> {
        match id {
            0x8ffa9a1f_u32 => {
                let text = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Subtitle {
                    text,
                })
            },

            0xef1751b5_u32 => {
                let channel = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Channel {
                    channel,
                })
            },

            0xf12bb6e1_u32 => {
                let text = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Subheader {
                    text,
                })
            },

            0x48870999_u32 => {
                let text = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Footer {
                    text,
                })
            },

            0xdb20b188_u32 => {
            
                Ok(PageBlock::Divider {
                })
            },

            0xcde200d1_u32 => {
                let flags = buf.deserialize()?;

                let full_width = flags & 1 << 0 != 0;

                let allow_scrolling = flags & 1 << 3 != 0;

                let url = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let html = if flags & 1 << 2 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let poster_photo_id = if flags & 1 << 4 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let w = buf.deserialize()?;
                let h = buf.deserialize()?;
                let caption = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Embed {
                    flags,
                    full_width,
                    allow_scrolling,
                    url,
                    html,
                    poster_photo_id,
                    w,
                    h,
                    caption,
                })
            },

            0x130c8963_u32 => {
                let items = buf.deserialize()?;
                let caption = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Slideshow {
                    items,
                    caption,
                })
            },

            0xd9d71866_u32 => {
                let flags = buf.deserialize()?;

                let autoplay = flags & 1 << 0 != 0;

                let loop_ = flags & 1 << 1 != 0;
                let video_id = buf.deserialize()?;
                let caption = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Video {
                    flags,
                    autoplay,
                    loop_,
                    video_id,
                    caption,
                })
            },

            0xc070d93e_u32 => {
                let text = Box::new(buf.deserialize()?);
                let language = buf.deserialize()?;
            
                Ok(PageBlock::Preformatted {
                    text,
                    language,
                })
            },

            0xce0d37b0_u32 => {
                let name = buf.deserialize()?;
            
                Ok(PageBlock::Anchor {
                    name,
                })
            },

            0x3a58c7f4_u32 => {
                let ordered = buf.deserialize()?;
                let items = buf.deserialize()?;
            
                Ok(PageBlock::List {
                    ordered,
                    items,
                })
            },

            0x292c7be9_u32 => {
                let url = buf.deserialize()?;
                let webpage_id = buf.deserialize()?;
                let author_photo_id = buf.deserialize()?;
                let author = buf.deserialize()?;
                let date = buf.deserialize()?;
                let blocks = buf.deserialize()?;
                let caption = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::EmbedPost {
                    url,
                    webpage_id,
                    author_photo_id,
                    author,
                    date,
                    blocks,
                    caption,
                })
            },

            0xe9c69982_u32 => {
                let photo_id = buf.deserialize()?;
                let caption = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Photo {
                    photo_id,
                    caption,
                })
            },

            0x263d7c26_u32 => {
                let text = Box::new(buf.deserialize()?);
                let caption = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Blockquote {
                    text,
                    caption,
                })
            },

            0x08b31c4f_u32 => {
                let items = buf.deserialize()?;
                let caption = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Collage {
                    items,
                    caption,
                })
            },

            0x70abc3fd_u32 => {
                let text = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Title {
                    text,
                })
            },

            0x4f4456d3_u32 => {
                let text = Box::new(buf.deserialize()?);
                let caption = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Pullquote {
                    text,
                    caption,
                })
            },

            0xbaafe5e0_u32 => {
                let author = Box::new(buf.deserialize()?);
                let published_date = buf.deserialize()?;
            
                Ok(PageBlock::AuthorDate {
                    author,
                    published_date,
                })
            },

            0x467a0766_u32 => {
                let text = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Paragraph {
                    text,
                })
            },

            0xbfd064ec_u32 => {
                let text = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Header {
                    text,
                })
            },

            0x13567e8a_u32 => {
            
                Ok(PageBlock::Unsupported {
                })
            },

            0x31b81a7f_u32 => {
                let audio_id = buf.deserialize()?;
                let caption = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Audio {
                    audio_id,
                    caption,
                })
            },

            0x39f23300_u32 => {
                let cover = Box::new(buf.deserialize()?);
            
                Ok(PageBlock::Cover {
                    cover,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ChannelMessagesFilter {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChannelMessagesFilter> {
        match id {
            0xcd77d957_u32 => {
                let flags = buf.deserialize()?;

                let exclude_new_messages = flags & 1 << 1 != 0;
                let ranges = buf.deserialize()?;
            
                Ok(ChannelMessagesFilter::Filter {
                    flags,
                    exclude_new_messages,
                    ranges,
                })
            },

            0x94d42ee7_u32 => {
            
                Ok(ChannelMessagesFilter::Empty {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MessagesHighScores {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x9a3bfd99);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesHighScores> {
                let scores = buf.deserialize()?;
        let users = buf.deserialize()?;

        Ok(MessagesHighScores {
            scores,
            users,
        })
    }
}

impl Deserializable for InputBotInlineMessage {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputBotInlineMessage> {
        match id {
            0xc1b15d65_u32 => {
                let flags = buf.deserialize()?;
                let geo_point = Box::new(buf.deserialize()?);
                let period = buf.deserialize()?;

                let reply_markup = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(InputBotInlineMessage::MediaGeo {
                    flags,
                    geo_point,
                    period,
                    reply_markup,
                })
            },

            0x3dcd7a87_u32 => {
                let flags = buf.deserialize()?;

                let no_webpage = flags & 1 << 0 != 0;
                let message = buf.deserialize()?;

                let entities = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let reply_markup = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(InputBotInlineMessage::Text {
                    flags,
                    no_webpage,
                    message,
                    entities,
                    reply_markup,
                })
            },

            0x2daf01a7_u32 => {
                let flags = buf.deserialize()?;
                let phone_number = buf.deserialize()?;
                let first_name = buf.deserialize()?;
                let last_name = buf.deserialize()?;

                let reply_markup = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(InputBotInlineMessage::MediaContact {
                    flags,
                    phone_number,
                    first_name,
                    last_name,
                    reply_markup,
                })
            },

            0x292fed13_u32 => {
                let flags = buf.deserialize()?;
                let caption = buf.deserialize()?;

                let reply_markup = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(InputBotInlineMessage::MediaAuto {
                    flags,
                    caption,
                    reply_markup,
                })
            },

            0xaaafadc8_u32 => {
                let flags = buf.deserialize()?;
                let geo_point = Box::new(buf.deserialize()?);
                let title = buf.deserialize()?;
                let address = buf.deserialize()?;
                let provider = buf.deserialize()?;
                let venue_id = buf.deserialize()?;

                let reply_markup = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(InputBotInlineMessage::MediaVenue {
                    flags,
                    geo_point,
                    title,
                    address,
                    provider,
                    venue_id,
                    reply_markup,
                })
            },

            0x4b425864_u32 => {
                let flags = buf.deserialize()?;

                let reply_markup = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(InputBotInlineMessage::Game {
                    flags,
                    reply_markup,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for Updates {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Updates> {
        match id {
            0xe317af7e_u32 => {
            
                Ok(Updates::TooLong {
                })
            },

            0x16812688_u32 => {
                let flags = buf.deserialize()?;

                let out = flags & 1 << 1 != 0;

                let mentioned = flags & 1 << 4 != 0;

                let media_unread = flags & 1 << 5 != 0;

                let silent = flags & 1 << 13 != 0;
                let id = buf.deserialize()?;
                let from_id = buf.deserialize()?;
                let chat_id = buf.deserialize()?;
                let message = buf.deserialize()?;
                let pts = buf.deserialize()?;
                let pts_count = buf.deserialize()?;
                let date = buf.deserialize()?;

                let fwd_from = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let via_bot_id = if flags & 1 << 11 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let reply_to_msg_id = if flags & 1 << 3 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let entities = if flags & 1 << 7 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(Updates::UpdateShortChatMessage {
                    flags,
                    out,
                    mentioned,
                    media_unread,
                    silent,
                    id,
                    from_id,
                    chat_id,
                    message,
                    pts,
                    pts_count,
                    date,
                    fwd_from,
                    via_bot_id,
                    reply_to_msg_id,
                    entities,
                })
            },

            0x914fbf11_u32 => {
                let flags = buf.deserialize()?;

                let out = flags & 1 << 1 != 0;

                let mentioned = flags & 1 << 4 != 0;

                let media_unread = flags & 1 << 5 != 0;

                let silent = flags & 1 << 13 != 0;
                let id = buf.deserialize()?;
                let user_id = buf.deserialize()?;
                let message = buf.deserialize()?;
                let pts = buf.deserialize()?;
                let pts_count = buf.deserialize()?;
                let date = buf.deserialize()?;

                let fwd_from = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let via_bot_id = if flags & 1 << 11 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let reply_to_msg_id = if flags & 1 << 3 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let entities = if flags & 1 << 7 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(Updates::UpdateShortMessage {
                    flags,
                    out,
                    mentioned,
                    media_unread,
                    silent,
                    id,
                    user_id,
                    message,
                    pts,
                    pts_count,
                    date,
                    fwd_from,
                    via_bot_id,
                    reply_to_msg_id,
                    entities,
                })
            },

            0x78d4dec1_u32 => {
                let update = Box::new(buf.deserialize()?);
                let date = buf.deserialize()?;
            
                Ok(Updates::UpdateShort {
                    update,
                    date,
                })
            },

            0x11f1331c_u32 => {
                let flags = buf.deserialize()?;

                let out = flags & 1 << 1 != 0;
                let id = buf.deserialize()?;
                let pts = buf.deserialize()?;
                let pts_count = buf.deserialize()?;
                let date = buf.deserialize()?;

                let media = if flags & 1 << 9 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let entities = if flags & 1 << 7 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(Updates::UpdateShortSentMessage {
                    flags,
                    out,
                    id,
                    pts,
                    pts_count,
                    date,
                    media,
                    entities,
                })
            },

            0x74ae4240_u32 => {
                let updates = buf.deserialize()?;
                let users = buf.deserialize()?;
                let chats = buf.deserialize()?;
                let date = buf.deserialize()?;
                let seq = buf.deserialize()?;
            
                Ok(Updates::Updates {
                    updates,
                    users,
                    chats,
                    date,
                    seq,
                })
            },

            0x725b04c3_u32 => {
                let updates = buf.deserialize()?;
                let users = buf.deserialize()?;
                let chats = buf.deserialize()?;
                let date = buf.deserialize()?;
                let seq_start = buf.deserialize()?;
                let seq = buf.deserialize()?;
            
                Ok(Updates::Combined {
                    updates,
                    users,
                    chats,
                    date,
                    seq_start,
                    seq,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for PhoneConnection {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x9d4c17c0);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PhoneConnection> {
                let id = buf.deserialize()?;
        let ip = buf.deserialize()?;
        let ipv6 = buf.deserialize()?;
        let port = buf.deserialize()?;
        let peer_tag = buf.deserialize()?;

        Ok(PhoneConnection {
            id,
            ip,
            ipv6,
            port,
            peer_tag,
        })
    }
}

impl Deserializable for HelpInviteText {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x18cb9f78);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<HelpInviteText> {
                let message = buf.deserialize()?;

        Ok(HelpInviteText {
            message,
        })
    }
}

impl Deserializable for MessageEntity {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessageEntity> {
        match id {
            0xbb92ba95_u32 => {
                let offset = buf.deserialize()?;
                let length = buf.deserialize()?;
            
                Ok(MessageEntity::Unknown {
                    offset,
                    length,
                })
            },

            0x73924be0_u32 => {
                let offset = buf.deserialize()?;
                let length = buf.deserialize()?;
                let language = buf.deserialize()?;
            
                Ok(MessageEntity::Pre {
                    offset,
                    length,
                    language,
                })
            },

            0x76a6d327_u32 => {
                let offset = buf.deserialize()?;
                let length = buf.deserialize()?;
                let url = buf.deserialize()?;
            
                Ok(MessageEntity::TextUrl {
                    offset,
                    length,
                    url,
                })
            },

            0xfa04579d_u32 => {
                let offset = buf.deserialize()?;
                let length = buf.deserialize()?;
            
                Ok(MessageEntity::Mention {
                    offset,
                    length,
                })
            },

            0x208e68c9_u32 => {
                let offset = buf.deserialize()?;
                let length = buf.deserialize()?;
                let user_id = Box::new(buf.deserialize()?);
            
                Ok(MessageEntity::InputMentionName {
                    offset,
                    length,
                    user_id,
                })
            },

            0xbd610bc9_u32 => {
                let offset = buf.deserialize()?;
                let length = buf.deserialize()?;
            
                Ok(MessageEntity::Bold {
                    offset,
                    length,
                })
            },

            0x352dca58_u32 => {
                let offset = buf.deserialize()?;
                let length = buf.deserialize()?;
                let user_id = buf.deserialize()?;
            
                Ok(MessageEntity::MentionName {
                    offset,
                    length,
                    user_id,
                })
            },

            0x28a20571_u32 => {
                let offset = buf.deserialize()?;
                let length = buf.deserialize()?;
            
                Ok(MessageEntity::Code {
                    offset,
                    length,
                })
            },

            0x826f8b60_u32 => {
                let offset = buf.deserialize()?;
                let length = buf.deserialize()?;
            
                Ok(MessageEntity::Italic {
                    offset,
                    length,
                })
            },

            0x6f635b0d_u32 => {
                let offset = buf.deserialize()?;
                let length = buf.deserialize()?;
            
                Ok(MessageEntity::Hashtag {
                    offset,
                    length,
                })
            },

            0x6cef8ac7_u32 => {
                let offset = buf.deserialize()?;
                let length = buf.deserialize()?;
            
                Ok(MessageEntity::BotCommand {
                    offset,
                    length,
                })
            },

            0x6ed02538_u32 => {
                let offset = buf.deserialize()?;
                let length = buf.deserialize()?;
            
                Ok(MessageEntity::Url {
                    offset,
                    length,
                })
            },

            0x64e475c2_u32 => {
                let offset = buf.deserialize()?;
                let length = buf.deserialize()?;
            
                Ok(MessageEntity::Email {
                    offset,
                    length,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ChatParticipant {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChatParticipant> {
        match id {
            0xda13538a_u32 => {
                let user_id = buf.deserialize()?;
            
                Ok(ChatParticipant::Creator {
                    user_id,
                })
            },

            0xe2d6e436_u32 => {
                let user_id = buf.deserialize()?;
                let inviter_id = buf.deserialize()?;
                let date = buf.deserialize()?;
            
                Ok(ChatParticipant::Admin {
                    user_id,
                    inviter_id,
                    date,
                })
            },

            0xc8d7493e_u32 => {
                let user_id = buf.deserialize()?;
                let inviter_id = buf.deserialize()?;
                let date = buf.deserialize()?;
            
                Ok(ChatParticipant::Participant {
                    user_id,
                    inviter_id,
                    date,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ChannelAdminLogEventAction {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChannelAdminLogEventAction> {
        match id {
            0x5f5c95f1_u32 => {
                let new_value = buf.deserialize()?;
            
                Ok(ChannelAdminLogEventAction::TogglePreHistoryHidden {
                    new_value,
                })
            },

            0xb82f55c3_u32 => {
                let prev_photo = Box::new(buf.deserialize()?);
                let new_photo = Box::new(buf.deserialize()?);
            
                Ok(ChannelAdminLogEventAction::ChangePhoto {
                    prev_photo,
                    new_photo,
                })
            },

            0x709b2405_u32 => {
                let prev_message = Box::new(buf.deserialize()?);
                let new_message = Box::new(buf.deserialize()?);
            
                Ok(ChannelAdminLogEventAction::EditMessage {
                    prev_message,
                    new_message,
                })
            },

            0x6a4afc38_u32 => {
                let prev_value = buf.deserialize()?;
                let new_value = buf.deserialize()?;
            
                Ok(ChannelAdminLogEventAction::ChangeUsername {
                    prev_value,
                    new_value,
                })
            },

            0xd5676710_u32 => {
                let prev_participant = Box::new(buf.deserialize()?);
                let new_participant = Box::new(buf.deserialize()?);
            
                Ok(ChannelAdminLogEventAction::ParticipantToggleAdmin {
                    prev_participant,
                    new_participant,
                })
            },

            0x183040d3_u32 => {
            
                Ok(ChannelAdminLogEventAction::ParticipantJoin {
                })
            },

            0xf89777f2_u32 => {
            
                Ok(ChannelAdminLogEventAction::ParticipantLeave {
                })
            },

            0x26ae0971_u32 => {
                let new_value = buf.deserialize()?;
            
                Ok(ChannelAdminLogEventAction::ToggleSignatures {
                    new_value,
                })
            },

            0xe6dfb825_u32 => {
                let prev_value = buf.deserialize()?;
                let new_value = buf.deserialize()?;
            
                Ok(ChannelAdminLogEventAction::ChangeTitle {
                    prev_value,
                    new_value,
                })
            },

            0xe31c34d8_u32 => {
                let participant = Box::new(buf.deserialize()?);
            
                Ok(ChannelAdminLogEventAction::ParticipantInvite {
                    participant,
                })
            },

            0xe6d83d7e_u32 => {
                let prev_participant = Box::new(buf.deserialize()?);
                let new_participant = Box::new(buf.deserialize()?);
            
                Ok(ChannelAdminLogEventAction::ParticipantToggleBan {
                    prev_participant,
                    new_participant,
                })
            },

            0x1b7907ae_u32 => {
                let new_value = buf.deserialize()?;
            
                Ok(ChannelAdminLogEventAction::ToggleInvites {
                    new_value,
                })
            },

            0x55188a2e_u32 => {
                let prev_value = buf.deserialize()?;
                let new_value = buf.deserialize()?;
            
                Ok(ChannelAdminLogEventAction::ChangeAbout {
                    prev_value,
                    new_value,
                })
            },

            0x42e047bb_u32 => {
                let message = Box::new(buf.deserialize()?);
            
                Ok(ChannelAdminLogEventAction::DeleteMessage {
                    message,
                })
            },

            0xb1c3caa7_u32 => {
                let prev_stickerset = Box::new(buf.deserialize()?);
                let new_stickerset = Box::new(buf.deserialize()?);
            
                Ok(ChannelAdminLogEventAction::ChangeStickerSet {
                    prev_stickerset,
                    new_stickerset,
                })
            },

            0xe9e82c18_u32 => {
                let message = Box::new(buf.deserialize()?);
            
                Ok(ChannelAdminLogEventAction::UpdatePinned {
                    message,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ContactStatus {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xd3680c61);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ContactStatus> {
                let user_id = buf.deserialize()?;
        let status = Box::new(buf.deserialize()?);

        Ok(ContactStatus {
            user_id,
            status,
        })
    }
}

impl Deserializable for MessageAction {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessageAction> {
        match id {
            0x4792929b_u32 => {
            
                Ok(MessageAction::ScreenshotTaken {
                })
            },

            0x40699cd0_u32 => {
                let currency = buf.deserialize()?;
                let total_amount = buf.deserialize()?;
            
                Ok(MessageAction::PaymentSent {
                    currency,
                    total_amount,
                })
            },

            0x51bdb021_u32 => {
                let channel_id = buf.deserialize()?;
            
                Ok(MessageAction::ChatMigrateTo {
                    channel_id,
                })
            },

            0xb5a1ce5a_u32 => {
                let title = buf.deserialize()?;
            
                Ok(MessageAction::ChatEditTitle {
                    title,
                })
            },

            0x9fbab604_u32 => {
            
                Ok(MessageAction::HistoryClear {
                })
            },

            0x95d2ac92_u32 => {
                let title = buf.deserialize()?;
            
                Ok(MessageAction::ChannelCreate {
                    title,
                })
            },

            0xa6638b9a_u32 => {
                let title = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(MessageAction::ChatCreate {
                    title,
                    users,
                })
            },

            0x95e3fbef_u32 => {
            
                Ok(MessageAction::ChatDeletePhoto {
                })
            },

            0xb6aef7b0_u32 => {
            
                Ok(MessageAction::Empty {
                })
            },

            0xfae69f56_u32 => {
                let message = buf.deserialize()?;
            
                Ok(MessageAction::CustomAction {
                    message,
                })
            },

            0x488a7337_u32 => {
                let users = buf.deserialize()?;
            
                Ok(MessageAction::ChatAddUser {
                    users,
                })
            },

            0xb055eaee_u32 => {
                let title = buf.deserialize()?;
                let chat_id = buf.deserialize()?;
            
                Ok(MessageAction::ChannelMigrateFrom {
                    title,
                    chat_id,
                })
            },

            0x8f31b327_u32 => {
                let flags = buf.deserialize()?;
                let currency = buf.deserialize()?;
                let total_amount = buf.deserialize()?;
                let payload = buf.deserialize()?;

                let info = if flags & 1 << 0 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let shipping_option_id = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let charge = Box::new(buf.deserialize()?);
            
                Ok(MessageAction::PaymentSentMe {
                    flags,
                    currency,
                    total_amount,
                    payload,
                    info,
                    shipping_option_id,
                    charge,
                })
            },

            0xb2ae9b0c_u32 => {
                let user_id = buf.deserialize()?;
            
                Ok(MessageAction::ChatDeleteUser {
                    user_id,
                })
            },

            0x94bd38ed_u32 => {
            
                Ok(MessageAction::PinMessage {
                })
            },

            0x7fcb13a8_u32 => {
                let photo = Box::new(buf.deserialize()?);
            
                Ok(MessageAction::ChatEditPhoto {
                    photo,
                })
            },

            0x80e11a7f_u32 => {
                let flags = buf.deserialize()?;
                let call_id = buf.deserialize()?;

                let reason = if flags & 1 << 0 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let duration = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(MessageAction::PhoneCall {
                    flags,
                    call_id,
                    reason,
                    duration,
                })
            },

            0xf89cf5e8_u32 => {
                let inviter_id = buf.deserialize()?;
            
                Ok(MessageAction::ChatJoinedByLink {
                    inviter_id,
                })
            },

            0x92a72876_u32 => {
                let game_id = buf.deserialize()?;
                let score = buf.deserialize()?;
            
                Ok(MessageAction::GameScore {
                    game_id,
                    score,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputStickeredMedia {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputStickeredMedia> {
        match id {
            0x0438865b_u32 => {
                let id = Box::new(buf.deserialize()?);
            
                Ok(InputStickeredMedia::Document {
                    id,
                })
            },

            0x4a992157_u32 => {
                let id = Box::new(buf.deserialize()?);
            
                Ok(InputStickeredMedia::Photo {
                    id,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for Invoice {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xc30aa358);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Invoice> {
                let flags = buf.deserialize()?;

        let test = flags & 1 << 0 != 0;

        let name_requested = flags & 1 << 1 != 0;

        let phone_requested = flags & 1 << 2 != 0;

        let email_requested = flags & 1 << 3 != 0;

        let shipping_address_requested = flags & 1 << 4 != 0;

        let flexible = flags & 1 << 5 != 0;

        let phone_to_provider = flags & 1 << 6 != 0;

        let email_to_provider = flags & 1 << 7 != 0;
        let currency = buf.deserialize()?;
        let prices = buf.deserialize()?;

        Ok(Invoice {
            flags,
            test,
            name_requested,
            phone_requested,
            email_requested,
            shipping_address_requested,
            flexible,
            phone_to_provider,
            email_to_provider,
            currency,
            prices,
        })
    }
}

impl Deserializable for MessagesStickerSetInstallResult {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesStickerSetInstallResult> {
        match id {
            0x38641628_u32 => {
            
                Ok(MessagesStickerSetInstallResult::Success {
                })
            },

            0x35e410a8_u32 => {
                let sets = buf.deserialize()?;
            
                Ok(MessagesStickerSetInstallResult::Archive {
                    sets,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for NewSession {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<NewSession> {
        match id {
            0x9ec20908_u32 => {
                let first_msg_id = buf.deserialize()?;
                let unique_id = buf.deserialize()?;
                let server_salt = buf.deserialize()?;
            
                Ok(NewSession::Created {
                    first_msg_id,
                    unique_id,
                    server_salt,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputPeerNotifySettings {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x38935eb2);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputPeerNotifySettings> {
                let flags = buf.deserialize()?;

        let show_previews = flags & 1 << 0 != 0;

        let silent = flags & 1 << 1 != 0;
        let mute_until = buf.deserialize()?;
        let sound = buf.deserialize()?;

        Ok(InputPeerNotifySettings {
            flags,
            show_previews,
            silent,
            mute_until,
            sound,
        })
    }
}

impl Deserializable for MsgsAllInfo {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x8cc0d131);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MsgsAllInfo> {
                let msg_ids = buf.deserialize()?;
        let info = buf.deserialize()?;

        Ok(MsgsAllInfo {
            msg_ids,
            info,
        })
    }
}

impl Deserializable for MessagesChatFull {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xe5d7d19c);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesChatFull> {
                let full_chat = Box::new(buf.deserialize()?);
        let chats = buf.deserialize()?;
        let users = buf.deserialize()?;

        Ok(MessagesChatFull {
            full_chat,
            chats,
            users,
        })
    }
}

impl Deserializable for MessagesSentEncryptedMessage {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesSentEncryptedMessage> {
        match id {
            0x9493ff32_u32 => {
                let date = buf.deserialize()?;
                let file = Box::new(buf.deserialize()?);
            
                Ok(MessagesSentEncryptedMessage::File {
                    date,
                    file,
                })
            },

            0x560f8935_u32 => {
                let date = buf.deserialize()?;
            
                Ok(MessagesSentEncryptedMessage::Message {
                    date,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for UserProfilePhoto {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<UserProfilePhoto> {
        match id {
            0x4f11bae1_u32 => {
            
                Ok(UserProfilePhoto::Empty {
                })
            },

            0xd559d8c8_u32 => {
                let photo_id = buf.deserialize()?;
                let photo_small = Box::new(buf.deserialize()?);
                let photo_big = Box::new(buf.deserialize()?);
            
                Ok(UserProfilePhoto::Photo {
                    photo_id,
                    photo_small,
                    photo_big,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputPeer {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputPeer> {
        match id {
            0x7b8e7de6_u32 => {
                let user_id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
            
                Ok(InputPeer::User {
                    user_id,
                    access_hash,
                })
            },

            0x7f3b18ea_u32 => {
            
                Ok(InputPeer::Empty {
                })
            },

            0x179be863_u32 => {
                let chat_id = buf.deserialize()?;
            
                Ok(InputPeer::Chat {
                    chat_id,
                })
            },

            0x7da07ec9_u32 => {
            
                Ok(InputPeer::Self_ {
                })
            },

            0x20adaef8_u32 => {
                let channel_id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
            
                Ok(InputPeer::Channel {
                    channel_id,
                    access_hash,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for Update {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Update> {
        match id {
            0x10c2404b_u32 => {
            
                Ok(Update::LangPackTooLong {
                })
            },

            0x9a422c20_u32 => {
            
                Ok(Update::RecentStickers {
                })
            },

            0x6e5f8c22_u32 => {
                let chat_id = buf.deserialize()?;
                let user_id = buf.deserialize()?;
                let version = buf.deserialize()?;
            
                Ok(Update::ChatParticipantDelete {
                    chat_id,
                    user_id,
                    version,
                })
            },

            0x5d2f3aa9_u32 => {
                let flags = buf.deserialize()?;
                let query_id = buf.deserialize()?;
                let user_id = buf.deserialize()?;
                let payload = buf.deserialize()?;

                let info = if flags & 1 << 0 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let shipping_option_id = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let currency = buf.deserialize()?;
                let total_amount = buf.deserialize()?;
            
                Ok(Update::BotPrecheckoutQuery {
                    flags,
                    query_id,
                    user_id,
                    payload,
                    info,
                    shipping_option_id,
                    currency,
                    total_amount,
                })
            },

            0x7084a7be_u32 => {
            
                Ok(Update::ContactsReset {
                })
            },

            0x9b9240a6_u32 => {
                let query_id = buf.deserialize()?;
                let data = Box::new(buf.deserialize()?);
                let timeout = buf.deserialize()?;
            
                Ok(Update::BotWebhookJsonquery {
                    query_id,
                    data,
                    timeout,
                })
            },

            0x38fe25b7_u32 => {
                let chat_id = buf.deserialize()?;
                let max_date = buf.deserialize()?;
                let date = buf.deserialize()?;
            
                Ok(Update::EncryptedMessagesRead {
                    chat_id,
                    max_date,
                    date,
                })
            },

            0x1f2b0afd_u32 => {
                let message = Box::new(buf.deserialize()?);
                let pts = buf.deserialize()?;
                let pts_count = buf.deserialize()?;
            
                Ok(Update::NewMessage {
                    message,
                    pts,
                    pts_count,
                })
            },

            0x25d6c9c7_u32 => {
                let channel_id = buf.deserialize()?;
                let max_id = buf.deserialize()?;
            
                Ok(Update::ReadChannelOutbox {
                    channel_id,
                    max_id,
                })
            },

            0x5c486927_u32 => {
                let user_id = buf.deserialize()?;
                let action = Box::new(buf.deserialize()?);
            
                Ok(Update::UserTyping {
                    user_id,
                    action,
                })
            },

            0x70db6837_u32 => {
                let channel_id = buf.deserialize()?;
                let available_min_id = buf.deserialize()?;
            
                Ok(Update::ChannelAvailableMessages {
                    channel_id,
                    available_min_id,
                })
            },

            0x9d2e67c5_u32 => {
                let user_id = buf.deserialize()?;
                let my_link = Box::new(buf.deserialize()?);
                let foreign_link = Box::new(buf.deserialize()?);
            
                Ok(Update::ContactLink {
                    user_id,
                    my_link,
                    foreign_link,
                })
            },

            0x7f891213_u32 => {
                let webpage = Box::new(buf.deserialize()?);
                let pts = buf.deserialize()?;
                let pts_count = buf.deserialize()?;
            
                Ok(Update::WebPage {
                    webpage,
                    pts,
                    pts_count,
                })
            },

            0x6e947941_u32 => {
                let chat_id = buf.deserialize()?;
                let enabled = buf.deserialize()?;
                let version = buf.deserialize()?;
            
                Ok(Update::ChatAdmins {
                    chat_id,
                    enabled,
                    version,
                })
            },

            0x68c13933_u32 => {
                let messages = buf.deserialize()?;
                let pts = buf.deserialize()?;
                let pts_count = buf.deserialize()?;
            
                Ok(Update::ReadMessagesContents {
                    messages,
                    pts,
                    pts_count,
                })
            },

            0x0bb2d201_u32 => {
                let flags = buf.deserialize()?;

                let masks = flags & 1 << 0 != 0;
                let order = buf.deserialize()?;
            
                Ok(Update::StickerSetsOrder {
                    flags,
                    masks,
                    order,
                })
            },

            0x12bcbd9a_u32 => {
                let message = Box::new(buf.deserialize()?);
                let qts = buf.deserialize()?;
            
                Ok(Update::NewEncryptedMessage {
                    message,
                    qts,
                })
            },

            0x688a30aa_u32 => {
                let stickerset = Box::new(buf.deserialize()?);
            
                Ok(Update::NewStickerSet {
                    stickerset,
                })
            },

            0xe73547e1_u32 => {
                let flags = buf.deserialize()?;
                let query_id = buf.deserialize()?;
                let user_id = buf.deserialize()?;
                let peer = Box::new(buf.deserialize()?);
                let msg_id = buf.deserialize()?;
                let chat_instance = buf.deserialize()?;

                let data = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let game_short_name = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(Update::BotCallbackQuery {
                    flags,
                    query_id,
                    user_id,
                    peer,
                    msg_id,
                    chat_instance,
                    data,
                    game_short_name,
                })
            },

            0x4214f37f_u32 => {
                let channel_id = buf.deserialize()?;
                let max_id = buf.deserialize()?;
            
                Ok(Update::ReadChannelInbox {
                    channel_id,
                    max_id,
                })
            },

            0x1bfbd823_u32 => {
                let user_id = buf.deserialize()?;
                let status = Box::new(buf.deserialize()?);
            
                Ok(Update::UserStatus {
                    user_id,
                    status,
                })
            },

            0x19d27f3c_u32 => {
                let flags = buf.deserialize()?;

                let pinned = flags & 1 << 0 != 0;
                let peer = Box::new(buf.deserialize()?);
            
                Ok(Update::DialogPinned {
                    flags,
                    pinned,
                    peer,
                })
            },

            0xe40370a3_u32 => {
                let message = Box::new(buf.deserialize()?);
                let pts = buf.deserialize()?;
                let pts_count = buf.deserialize()?;
            
                Ok(Update::EditMessage {
                    message,
                    pts,
                    pts_count,
                })
            },

            0x571d2742_u32 => {
            
                Ok(Update::ReadFeaturedStickers {
                })
            },

            0x12b9417b_u32 => {
                let user_id = buf.deserialize()?;
                let phone = buf.deserialize()?;
            
                Ok(Update::UserPhone {
                    user_id,
                    phone,
                })
            },

            0x1b3f4df7_u32 => {
                let message = Box::new(buf.deserialize()?);
                let pts = buf.deserialize()?;
                let pts_count = buf.deserialize()?;
            
                Ok(Update::EditChannelMessage {
                    message,
                    pts,
                    pts_count,
                })
            },

            0x0e48f964_u32 => {
                let flags = buf.deserialize()?;
                let user_id = buf.deserialize()?;
                let query = buf.deserialize()?;

                let geo = if flags & 1 << 0 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
                let id = buf.deserialize()?;

                let msg_id = if flags & 1 << 1 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(Update::BotInlineSend {
                    flags,
                    user_id,
                    query,
                    geo,
                    id,
                    msg_id,
                })
            },

            0xeb0467fb_u32 => {
                let flags = buf.deserialize()?;
                let channel_id = buf.deserialize()?;

                let pts = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(Update::ChannelTooLong {
                    flags,
                    channel_id,
                    pts,
                })
            },

            0xe511996d_u32 => {
            
                Ok(Update::FavedStickers {
                })
            },

            0xea4b0e5c_u32 => {
                let chat_id = buf.deserialize()?;
                let user_id = buf.deserialize()?;
                let inviter_id = buf.deserialize()?;
                let date = buf.deserialize()?;
                let version = buf.deserialize()?;
            
                Ok(Update::ChatParticipantAdd {
                    chat_id,
                    user_id,
                    inviter_id,
                    date,
                    version,
                })
            },

            0xab0f6b1e_u32 => {
                let phone_call = Box::new(buf.deserialize()?);
            
                Ok(Update::PhoneCall {
                    phone_call,
                })
            },

            0x43ae3dec_u32 => {
            
                Ok(Update::StickerSets {
                })
            },

            0xbec268ef_u32 => {
                let peer = Box::new(buf.deserialize()?);
                let notify_settings = Box::new(buf.deserialize()?);
            
                Ok(Update::NotifySettings {
                    peer,
                    notify_settings,
                })
            },

            0xc37521c9_u32 => {
                let channel_id = buf.deserialize()?;
                let messages = buf.deserialize()?;
                let pts = buf.deserialize()?;
                let pts_count = buf.deserialize()?;
            
                Ok(Update::DeleteChannelMessages {
                    channel_id,
                    messages,
                    pts,
                    pts_count,
                })
            },

            0xea4cb65b_u32 => {
                let flags = buf.deserialize()?;

                let order = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(Update::PinnedDialogs {
                    flags,
                    order,
                })
            },

            0x80ece81a_u32 => {
                let user_id = buf.deserialize()?;
                let blocked = buf.deserialize()?;
            
                Ok(Update::UserBlocked {
                    user_id,
                    blocked,
                })
            },

            0x62ba04d9_u32 => {
                let message = Box::new(buf.deserialize()?);
                let pts = buf.deserialize()?;
                let pts_count = buf.deserialize()?;
            
                Ok(Update::NewChannelMessage {
                    message,
                    pts,
                    pts_count,
                })
            },

            0x54826690_u32 => {
                let flags = buf.deserialize()?;
                let query_id = buf.deserialize()?;
                let user_id = buf.deserialize()?;
                let query = buf.deserialize()?;

                let geo = if flags & 1 << 0 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
                let offset = buf.deserialize()?;
            
                Ok(Update::BotInlineQuery {
                    flags,
                    query_id,
                    user_id,
                    query,
                    geo,
                    offset,
                })
            },

            0x9a65ea1f_u32 => {
                let chat_id = buf.deserialize()?;
                let user_id = buf.deserialize()?;
                let action = Box::new(buf.deserialize()?);
            
                Ok(Update::ChatUserTyping {
                    chat_id,
                    user_id,
                    action,
                })
            },

            0x40771900_u32 => {
                let channel_id = buf.deserialize()?;
                let webpage = Box::new(buf.deserialize()?);
                let pts = buf.deserialize()?;
                let pts_count = buf.deserialize()?;
            
                Ok(Update::ChannelWebPage {
                    channel_id,
                    webpage,
                    pts,
                    pts_count,
                })
            },

            0x2f2f21bf_u32 => {
                let peer = Box::new(buf.deserialize()?);
                let max_id = buf.deserialize()?;
                let pts = buf.deserialize()?;
                let pts_count = buf.deserialize()?;
            
                Ok(Update::ReadHistoryOutbox {
                    peer,
                    max_id,
                    pts,
                    pts_count,
                })
            },

            0x9961fd5c_u32 => {
                let peer = Box::new(buf.deserialize()?);
                let max_id = buf.deserialize()?;
                let pts = buf.deserialize()?;
                let pts_count = buf.deserialize()?;
            
                Ok(Update::ReadHistoryInbox {
                    peer,
                    max_id,
                    pts,
                    pts_count,
                })
            },

            0x2575bbb9_u32 => {
                let user_id = buf.deserialize()?;
                let date = buf.deserialize()?;
            
                Ok(Update::ContactRegistered {
                    user_id,
                    date,
                })
            },

            0x56022f4d_u32 => {
                let difference = Box::new(buf.deserialize()?);
            
                Ok(Update::LangPack {
                    difference,
                })
            },

            0x3354678f_u32 => {
            
                Ok(Update::PtsChanged {
                })
            },

            0xee2bb969_u32 => {
                let peer = Box::new(buf.deserialize()?);
                let draft = Box::new(buf.deserialize()?);
            
                Ok(Update::DraftMessage {
                    peer,
                    draft,
                })
            },

            0xe0cdc940_u32 => {
                let query_id = buf.deserialize()?;
                let user_id = buf.deserialize()?;
                let payload = buf.deserialize()?;
                let shipping_address = Box::new(buf.deserialize()?);
            
                Ok(Update::BotShippingQuery {
                    query_id,
                    user_id,
                    payload,
                    shipping_address,
                })
            },

            0x98592475_u32 => {
                let channel_id = buf.deserialize()?;
                let id = buf.deserialize()?;
            
                Ok(Update::ChannelPinnedMessage {
                    channel_id,
                    id,
                })
            },

            0x89893b45_u32 => {
                let channel_id = buf.deserialize()?;
                let messages = buf.deserialize()?;
            
                Ok(Update::ChannelReadMessagesContents {
                    channel_id,
                    messages,
                })
            },

            0x1710f156_u32 => {
                let chat_id = buf.deserialize()?;
            
                Ok(Update::EncryptedChatTyping {
                    chat_id,
                })
            },

            0x994852a9_u32 => {
                let feed_id = buf.deserialize()?;
                let max_position = Box::new(buf.deserialize()?);
            
                Ok(Update::ReadFeed {
                    feed_id,
                    max_position,
                })
            },

            0xb4a2e88d_u32 => {
                let chat = Box::new(buf.deserialize()?);
                let date = buf.deserialize()?;
            
                Ok(Update::Encryption {
                    chat,
                    date,
                })
            },

            0x8317c0c3_u32 => {
                let data = Box::new(buf.deserialize()?);
            
                Ok(Update::BotWebhookJson {
                    data,
                })
            },

            0x07761198_u32 => {
                let participants = Box::new(buf.deserialize()?);
            
                Ok(Update::ChatParticipants {
                    participants,
                })
            },

            0xb6901959_u32 => {
                let chat_id = buf.deserialize()?;
                let user_id = buf.deserialize()?;
                let is_admin = buf.deserialize()?;
                let version = buf.deserialize()?;
            
                Ok(Update::ChatParticipantAdmin {
                    chat_id,
                    user_id,
                    is_admin,
                    version,
                })
            },

            0x8e5e9873_u32 => {
                let dc_options = buf.deserialize()?;
            
                Ok(Update::DcOptions {
                    dc_options,
                })
            },

            0xee3b272a_u32 => {
                let key = Box::new(buf.deserialize()?);
                let rules = buf.deserialize()?;
            
                Ok(Update::Privacy {
                    key,
                    rules,
                })
            },

            0xa7332b73_u32 => {
                let user_id = buf.deserialize()?;
                let first_name = buf.deserialize()?;
                let last_name = buf.deserialize()?;
                let username = buf.deserialize()?;
            
                Ok(Update::UserName {
                    user_id,
                    first_name,
                    last_name,
                    username,
                })
            },

            0xf9d27a5a_u32 => {
                let flags = buf.deserialize()?;
                let query_id = buf.deserialize()?;
                let user_id = buf.deserialize()?;
                let msg_id = Box::new(buf.deserialize()?);
                let chat_instance = buf.deserialize()?;

                let data = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let game_short_name = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(Update::InlineBotCallbackQuery {
                    flags,
                    query_id,
                    user_id,
                    msg_id,
                    chat_instance,
                    data,
                    game_short_name,
                })
            },

            0x95313b0c_u32 => {
                let user_id = buf.deserialize()?;
                let date = buf.deserialize()?;
                let photo = Box::new(buf.deserialize()?);
                let previous = buf.deserialize()?;
            
                Ok(Update::UserPhoto {
                    user_id,
                    date,
                    photo,
                    previous,
                })
            },

            0xebe46819_u32 => {
                let flags = buf.deserialize()?;

                let popup = flags & 1 << 0 != 0;

                let inbox_date = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let type_ = buf.deserialize()?;
                let message = buf.deserialize()?;
                let media = Box::new(buf.deserialize()?);
                let entities = buf.deserialize()?;
            
                Ok(Update::ServiceNotification {
                    flags,
                    popup,
                    inbox_date,
                    type_,
                    message,
                    media,
                    entities,
                })
            },

            0x4e90bfd6_u32 => {
                let id = buf.deserialize()?;
                let random_id = buf.deserialize()?;
            
                Ok(Update::MessageId {
                    id,
                    random_id,
                })
            },

            0x9375341e_u32 => {
            
                Ok(Update::SavedGifs {
                })
            },

            0xa20db0e5_u32 => {
                let messages = buf.deserialize()?;
                let pts = buf.deserialize()?;
                let pts_count = buf.deserialize()?;
            
                Ok(Update::DeleteMessages {
                    messages,
                    pts,
                    pts_count,
                })
            },

            0xa229dd06_u32 => {
            
                Ok(Update::Config {
                })
            },

            0x98a12b4b_u32 => {
                let channel_id = buf.deserialize()?;
                let id = buf.deserialize()?;
                let views = buf.deserialize()?;
            
                Ok(Update::ChannelMessageViews {
                    channel_id,
                    id,
                    views,
                })
            },

            0xb6d45656_u32 => {
                let channel_id = buf.deserialize()?;
            
                Ok(Update::Channel {
                    channel_id,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MessagesFoundGifs {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x450a1c0a);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesFoundGifs> {
                let next_offset = buf.deserialize()?;
        let results = buf.deserialize()?;

        Ok(MessagesFoundGifs {
            next_offset,
            results,
        })
    }
}

impl Deserializable for NearestDc {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x8e1a1775);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<NearestDc> {
                let country = buf.deserialize()?;
        let this_dc = buf.deserialize()?;
        let nearest_dc = buf.deserialize()?;

        Ok(NearestDc {
            country,
            this_dc,
            nearest_dc,
        })
    }
}

impl Deserializable for HighScore {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x58fffcd0);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<HighScore> {
                let pos = buf.deserialize()?;
        let user_id = buf.deserialize()?;
        let score = buf.deserialize()?;

        Ok(HighScore {
            pos,
            user_id,
            score,
        })
    }
}

impl Deserializable for AuthSentCode {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x5e002502);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<AuthSentCode> {
                let flags = buf.deserialize()?;

        let phone_registered = flags & 1 << 0 != 0;
        let type_ = Box::new(buf.deserialize()?);
        let phone_code_hash = buf.deserialize()?;

        let next_type = if flags & 1 << 1 != 0 {
            Some(Box::new(buf.deserialize()?))
        } else {
            None
        };

        let timeout = if flags & 1 << 2 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        Ok(AuthSentCode {
            flags,
            phone_registered,
            type_,
            phone_code_hash,
            next_type,
            timeout,
        })
    }
}

impl Deserializable for MsgResendReq {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x7d861a08);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MsgResendReq> {
                let msg_ids = buf.deserialize()?;

        Ok(MsgResendReq {
            msg_ids,
        })
    }
}

impl Deserializable for HelpRecentMeUrls {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x0e0310d7);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<HelpRecentMeUrls> {
                let urls = buf.deserialize()?;
        let chats = buf.deserialize()?;
        let users = buf.deserialize()?;

        Ok(HelpRecentMeUrls {
            urls,
            chats,
            users,
        })
    }
}

impl Deserializable for MessageFwdHeader {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x559ebe6d);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessageFwdHeader> {
                let flags = buf.deserialize()?;

        let from_id = if flags & 1 << 0 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };
        let date = buf.deserialize()?;

        let channel_id = if flags & 1 << 1 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        let channel_post = if flags & 1 << 2 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        let post_author = if flags & 1 << 3 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        let saved_from_peer = if flags & 1 << 4 != 0 {
            Some(Box::new(buf.deserialize()?))
        } else {
            None
        };

        let saved_from_msg_id = if flags & 1 << 4 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        Ok(MessageFwdHeader {
            flags,
            from_id,
            date,
            channel_id,
            channel_post,
            post_author,
            saved_from_peer,
            saved_from_msg_id,
        })
    }
}

impl Deserializable for InputGame {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputGame> {
        match id {
            0x032c3e77_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
            
                Ok(InputGame::Id {
                    id,
                    access_hash,
                })
            },

            0xc331e80a_u32 => {
                let bot_id = Box::new(buf.deserialize()?);
                let short_name = buf.deserialize()?;
            
                Ok(InputGame::ShortName {
                    bot_id,
                    short_name,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for FutureSalt {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x0949d9dc);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<FutureSalt> {
                let valid_since = buf.deserialize()?;
        let valid_until = buf.deserialize()?;
        let salt = buf.deserialize()?;

        Ok(FutureSalt {
            valid_since,
            valid_until,
            salt,
        })
    }
}

impl Deserializable for StickerPack {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x12b299d4);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<StickerPack> {
                let emoticon = buf.deserialize()?;
        let documents = buf.deserialize()?;

        Ok(StickerPack {
            emoticon,
            documents,
        })
    }
}

impl Deserializable for InputPrivacyRule {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputPrivacyRule> {
        match id {
            0x0ba52007_u32 => {
            
                Ok(InputPrivacyRule::ValueDisallowContacts {
                })
            },

            0x90110467_u32 => {
                let users = buf.deserialize()?;
            
                Ok(InputPrivacyRule::ValueDisallowUsers {
                    users,
                })
            },

            0x131cc67f_u32 => {
                let users = buf.deserialize()?;
            
                Ok(InputPrivacyRule::ValueAllowUsers {
                    users,
                })
            },

            0x184b35ce_u32 => {
            
                Ok(InputPrivacyRule::ValueAllowAll {
                })
            },

            0xd66b66c9_u32 => {
            
                Ok(InputPrivacyRule::ValueDisallowAll {
                })
            },

            0x0d09e07b_u32 => {
            
                Ok(InputPrivacyRule::ValueAllowContacts {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for HelpSupport {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x17c6b5f6);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<HelpSupport> {
                let phone_number = buf.deserialize()?;
        let user = Box::new(buf.deserialize()?);

        Ok(HelpSupport {
            phone_number,
            user,
        })
    }
}

impl Deserializable for ChannelsChannelParticipant {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xd0d9b163);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChannelsChannelParticipant> {
                let participant = Box::new(buf.deserialize()?);
        let users = buf.deserialize()?;

        Ok(ChannelsChannelParticipant {
            participant,
            users,
        })
    }
}

impl Deserializable for ReplyMarkup {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ReplyMarkup> {
        match id {
            0x3502758c_u32 => {
                let flags = buf.deserialize()?;

                let resize = flags & 1 << 0 != 0;

                let single_use = flags & 1 << 1 != 0;

                let selective = flags & 1 << 2 != 0;
                let rows = buf.deserialize()?;
            
                Ok(ReplyMarkup::Keyboard {
                    flags,
                    resize,
                    single_use,
                    selective,
                    rows,
                })
            },

            0xf4108aa0_u32 => {
                let flags = buf.deserialize()?;

                let single_use = flags & 1 << 1 != 0;

                let selective = flags & 1 << 2 != 0;
            
                Ok(ReplyMarkup::KeyboardForceReply {
                    flags,
                    single_use,
                    selective,
                })
            },

            0xa03e5b85_u32 => {
                let flags = buf.deserialize()?;

                let selective = flags & 1 << 2 != 0;
            
                Ok(ReplyMarkup::KeyboardHide {
                    flags,
                    selective,
                })
            },

            0x48a30254_u32 => {
                let rows = buf.deserialize()?;
            
                Ok(ReplyMarkup::Inline {
                    rows,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MessagesArchivedStickers {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x4fcba9c8);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesArchivedStickers> {
                let count = buf.deserialize()?;
        let sets = buf.deserialize()?;

        Ok(MessagesArchivedStickers {
            count,
            sets,
        })
    }
}

impl Deserializable for MessagesFavedStickers {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesFavedStickers> {
        match id {
            0xf37f2f16_u32 => {
                let hash = buf.deserialize()?;
                let packs = buf.deserialize()?;
                let stickers = buf.deserialize()?;
            
                Ok(MessagesFavedStickers::Stickers {
                    hash,
                    packs,
                    stickers,
                })
            },

            0x9e8fa6d3_u32 => {
            
                Ok(MessagesFavedStickers::NotModified {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for SendMessageAction {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<SendMessageAction> {
        match id {
            0x176f8ba1_u32 => {
            
                Ok(SendMessageAction::GeoLocation {
                })
            },

            0xd1d34a26_u32 => {
                let progress = buf.deserialize()?;
            
                Ok(SendMessageAction::UploadPhoto {
                    progress,
                })
            },

            0xaa0cd9e4_u32 => {
                let progress = buf.deserialize()?;
            
                Ok(SendMessageAction::UploadDocument {
                    progress,
                })
            },

            0xfd5ec8f5_u32 => {
            
                Ok(SendMessageAction::Cancel {
                })
            },

            0x16bf744e_u32 => {
            
                Ok(SendMessageAction::Typing {
                })
            },

            0x243e1c66_u32 => {
                let progress = buf.deserialize()?;
            
                Ok(SendMessageAction::UploadRound {
                    progress,
                })
            },

            0xd52f73f7_u32 => {
            
                Ok(SendMessageAction::RecordAudio {
                })
            },

            0xf351d7ab_u32 => {
                let progress = buf.deserialize()?;
            
                Ok(SendMessageAction::UploadAudio {
                    progress,
                })
            },

            0x88f27fbc_u32 => {
            
                Ok(SendMessageAction::RecordRound {
                })
            },

            0xdd6a8f48_u32 => {
            
                Ok(SendMessageAction::GamePlay {
                })
            },

            0xe9763aec_u32 => {
                let progress = buf.deserialize()?;
            
                Ok(SendMessageAction::UploadVideo {
                    progress,
                })
            },

            0xa187d66f_u32 => {
            
                Ok(SendMessageAction::RecordVideo {
                })
            },

            0x628cbc6f_u32 => {
            
                Ok(SendMessageAction::ChooseContact {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputContact {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputContact> {
        match id {
            0xf392b7f4_u32 => {
                let client_id = buf.deserialize()?;
                let phone = buf.deserialize()?;
                let first_name = buf.deserialize()?;
                let last_name = buf.deserialize()?;
            
                Ok(InputContact::Phone {
                    client_id,
                    phone,
                    first_name,
                    last_name,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for PhoneCall {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PhoneCall> {
        match id {
            0xffe6ab67_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
                let date = buf.deserialize()?;
                let admin_id = buf.deserialize()?;
                let participant_id = buf.deserialize()?;
                let g_a_or_b = buf.deserialize()?;
                let key_fingerprint = buf.deserialize()?;
                let protocol = Box::new(buf.deserialize()?);
                let connection = Box::new(buf.deserialize()?);
                let alternative_connections = buf.deserialize()?;
                let start_date = buf.deserialize()?;
            
                Ok(PhoneCall::Call {
                    id,
                    access_hash,
                    date,
                    admin_id,
                    participant_id,
                    g_a_or_b,
                    key_fingerprint,
                    protocol,
                    connection,
                    alternative_connections,
                    start_date,
                })
            },

            0x6d003d3f_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
                let date = buf.deserialize()?;
                let admin_id = buf.deserialize()?;
                let participant_id = buf.deserialize()?;
                let g_b = buf.deserialize()?;
                let protocol = Box::new(buf.deserialize()?);
            
                Ok(PhoneCall::Accepted {
                    id,
                    access_hash,
                    date,
                    admin_id,
                    participant_id,
                    g_b,
                    protocol,
                })
            },

            0x5366c915_u32 => {
                let id = buf.deserialize()?;
            
                Ok(PhoneCall::Empty {
                    id,
                })
            },

            0x1b8f4ad1_u32 => {
                let flags = buf.deserialize()?;
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
                let date = buf.deserialize()?;
                let admin_id = buf.deserialize()?;
                let participant_id = buf.deserialize()?;
                let protocol = Box::new(buf.deserialize()?);

                let receive_date = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(PhoneCall::Waiting {
                    flags,
                    id,
                    access_hash,
                    date,
                    admin_id,
                    participant_id,
                    protocol,
                    receive_date,
                })
            },

            0x50ca4de1_u32 => {
                let flags = buf.deserialize()?;

                let need_rating = flags & 1 << 2 != 0;

                let need_debug = flags & 1 << 3 != 0;
                let id = buf.deserialize()?;

                let reason = if flags & 1 << 0 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let duration = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(PhoneCall::Discarded {
                    flags,
                    need_rating,
                    need_debug,
                    id,
                    reason,
                    duration,
                })
            },

            0x83761ce4_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
                let date = buf.deserialize()?;
                let admin_id = buf.deserialize()?;
                let participant_id = buf.deserialize()?;
                let g_a_hash = buf.deserialize()?;
                let protocol = Box::new(buf.deserialize()?);
            
                Ok(PhoneCall::Requested {
                    id,
                    access_hash,
                    date,
                    admin_id,
                    participant_id,
                    g_a_hash,
                    protocol,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for KeyboardButtonRow {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x77608b83);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<KeyboardButtonRow> {
                let buttons = buf.deserialize()?;

        Ok(KeyboardButtonRow {
            buttons,
        })
    }
}

impl Deserializable for ChatPhoto {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChatPhoto> {
        match id {
            0x6153276a_u32 => {
                let photo_small = Box::new(buf.deserialize()?);
                let photo_big = Box::new(buf.deserialize()?);
            
                Ok(ChatPhoto::Photo {
                    photo_small,
                    photo_big,
                })
            },

            0x37c1011c_u32 => {
            
                Ok(ChatPhoto::Empty {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for BotCommand {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xc27ac8c7);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<BotCommand> {
                let command = buf.deserialize()?;
        let description = buf.deserialize()?;

        Ok(BotCommand {
            command,
            description,
        })
    }
}

impl Deserializable for ChannelsAdminLogResults {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xed8af74d);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChannelsAdminLogResults> {
                let events = buf.deserialize()?;
        let chats = buf.deserialize()?;
        let users = buf.deserialize()?;

        Ok(ChannelsAdminLogResults {
            events,
            chats,
            users,
        })
    }
}

impl Deserializable for ChannelParticipantsFilter {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChannelParticipantsFilter> {
        match id {
            0xa3b54985_u32 => {
                let q = buf.deserialize()?;
            
                Ok(ChannelParticipantsFilter::Kicked {
                    q,
                })
            },

            0xb0d1865b_u32 => {
            
                Ok(ChannelParticipantsFilter::Bots {
                })
            },

            0xde3f3c79_u32 => {
            
                Ok(ChannelParticipantsFilter::Recent {
                })
            },

            0x1427a5e1_u32 => {
                let q = buf.deserialize()?;
            
                Ok(ChannelParticipantsFilter::Banned {
                    q,
                })
            },

            0x0656ac4b_u32 => {
                let q = buf.deserialize()?;
            
                Ok(ChannelParticipantsFilter::Search {
                    q,
                })
            },

            0xb4608969_u32 => {
            
                Ok(ChannelParticipantsFilter::Admins {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for Document {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Document> {
        match id {
            0x87232bc7_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
                let date = buf.deserialize()?;
                let mime_type = buf.deserialize()?;
                let size = buf.deserialize()?;
                let thumb = Box::new(buf.deserialize()?);
                let dc_id = buf.deserialize()?;
                let version = buf.deserialize()?;
                let attributes = buf.deserialize()?;
            
                Ok(Document::Document {
                    id,
                    access_hash,
                    date,
                    mime_type,
                    size,
                    thumb,
                    dc_id,
                    version,
                    attributes,
                })
            },

            0x36f8c871_u32 => {
                let id = buf.deserialize()?;
            
                Ok(Document::Empty {
                    id,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for Peer {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Peer> {
        match id {
            0xbddde532_u32 => {
                let channel_id = buf.deserialize()?;
            
                Ok(Peer::Channel {
                    channel_id,
                })
            },

            0xbad0e5bb_u32 => {
                let chat_id = buf.deserialize()?;
            
                Ok(Peer::Chat {
                    chat_id,
                })
            },

            0x9db1bc6d_u32 => {
                let user_id = buf.deserialize()?;
            
                Ok(Peer::User {
                    user_id,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputBotInlineMessageId {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x890c3d89);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputBotInlineMessageId> {
                let dc_id = buf.deserialize()?;
        let id = buf.deserialize()?;
        let access_hash = buf.deserialize()?;

        Ok(InputBotInlineMessageId {
            dc_id,
            id,
            access_hash,
        })
    }
}

impl Deserializable for InputFileLocation {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputFileLocation> {
        match id {
            0x430f0724_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
                let version = buf.deserialize()?;
            
                Ok(InputFileLocation::Document {
                    id,
                    access_hash,
                    version,
                })
            },

            0xf5235d55_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
            
                Ok(InputFileLocation::Encrypted {
                    id,
                    access_hash,
                })
            },

            0x14637196_u32 => {
                let volume_id = buf.deserialize()?;
                let local_id = buf.deserialize()?;
                let secret = buf.deserialize()?;
            
                Ok(InputFileLocation::Location {
                    volume_id,
                    local_id,
                    secret,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for UpdatesState {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xa56c2a3e);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<UpdatesState> {
                let pts = buf.deserialize()?;
        let qts = buf.deserialize()?;
        let date = buf.deserialize()?;
        let seq = buf.deserialize()?;
        let unread_count = buf.deserialize()?;

        Ok(UpdatesState {
            pts,
            qts,
            date,
            seq,
            unread_count,
        })
    }
}

impl Deserializable for BotInlineMessage {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<BotInlineMessage> {
        match id {
            0xb722de65_u32 => {
                let flags = buf.deserialize()?;
                let geo = Box::new(buf.deserialize()?);
                let period = buf.deserialize()?;

                let reply_markup = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(BotInlineMessage::MediaGeo {
                    flags,
                    geo,
                    period,
                    reply_markup,
                })
            },

            0x35edb4d4_u32 => {
                let flags = buf.deserialize()?;
                let phone_number = buf.deserialize()?;
                let first_name = buf.deserialize()?;
                let last_name = buf.deserialize()?;

                let reply_markup = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(BotInlineMessage::MediaContact {
                    flags,
                    phone_number,
                    first_name,
                    last_name,
                    reply_markup,
                })
            },

            0x0a74b15b_u32 => {
                let flags = buf.deserialize()?;
                let caption = buf.deserialize()?;

                let reply_markup = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(BotInlineMessage::MediaAuto {
                    flags,
                    caption,
                    reply_markup,
                })
            },

            0x8c7f65e2_u32 => {
                let flags = buf.deserialize()?;

                let no_webpage = flags & 1 << 0 != 0;
                let message = buf.deserialize()?;

                let entities = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let reply_markup = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(BotInlineMessage::Text {
                    flags,
                    no_webpage,
                    message,
                    entities,
                    reply_markup,
                })
            },

            0x4366232e_u32 => {
                let flags = buf.deserialize()?;
                let geo = Box::new(buf.deserialize()?);
                let title = buf.deserialize()?;
                let address = buf.deserialize()?;
                let provider = buf.deserialize()?;
                let venue_id = buf.deserialize()?;

                let reply_markup = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(BotInlineMessage::MediaVenue {
                    flags,
                    geo,
                    title,
                    address,
                    provider,
                    venue_id,
                    reply_markup,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputWebDocument {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x9bed434d);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputWebDocument> {
                let url = buf.deserialize()?;
        let size = buf.deserialize()?;
        let mime_type = buf.deserialize()?;
        let attributes = buf.deserialize()?;

        Ok(InputWebDocument {
            url,
            size,
            mime_type,
            attributes,
        })
    }
}

impl Deserializable for ContactsFound {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xb3134d9d);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ContactsFound> {
                let my_results = buf.deserialize()?;
        let results = buf.deserialize()?;
        let chats = buf.deserialize()?;
        let users = buf.deserialize()?;

        Ok(ContactsFound {
            my_results,
            results,
            chats,
            users,
        })
    }
}

impl Deserializable for PaymentsPaymentResult {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PaymentsPaymentResult> {
        match id {
            0x6b56b921_u32 => {
                let url = buf.deserialize()?;
            
                Ok(PaymentsPaymentResult::VerficationNeeded {
                    url,
                })
            },

            0x4e5f810d_u32 => {
                let updates = Box::new(buf.deserialize()?);
            
                Ok(PaymentsPaymentResult::Result {
                    updates,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for AccountPasswordSettings {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xb7b72ab3);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<AccountPasswordSettings> {
                let email = buf.deserialize()?;

        Ok(AccountPasswordSettings {
            email,
        })
    }
}

impl Deserializable for Photo {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Photo> {
        match id {
            0x2331b22d_u32 => {
                let id = buf.deserialize()?;
            
                Ok(Photo::Empty {
                    id,
                })
            },

            0x9288dd29_u32 => {
                let flags = buf.deserialize()?;

                let has_stickers = flags & 1 << 0 != 0;
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
                let date = buf.deserialize()?;
                let sizes = buf.deserialize()?;
            
                Ok(Photo::Photo {
                    flags,
                    has_stickers,
                    id,
                    access_hash,
                    date,
                    sizes,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputPaymentCredentials {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputPaymentCredentials> {
        match id {
            0xc10eb2cf_u32 => {
                let id = buf.deserialize()?;
                let tmp_password = buf.deserialize()?;
            
                Ok(InputPaymentCredentials::Saved {
                    id,
                    tmp_password,
                })
            },

            0xca05d50e_u32 => {
                let payment_token = Box::new(buf.deserialize()?);
                let google_transaction_id = buf.deserialize()?;
            
                Ok(InputPaymentCredentials::AndroidPay {
                    payment_token,
                    google_transaction_id,
                })
            },

            0x3417d728_u32 => {
                let flags = buf.deserialize()?;

                let save = flags & 1 << 0 != 0;
                let data = Box::new(buf.deserialize()?);
            
                Ok(InputPaymentCredentials::Credentials {
                    flags,
                    save,
                    data,
                })
            },

            0x0aa1c39f_u32 => {
                let payment_data = Box::new(buf.deserialize()?);
            
                Ok(InputPaymentCredentials::ApplePay {
                    payment_data,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ContactsLink {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x3ace484c);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ContactsLink> {
                let my_link = Box::new(buf.deserialize()?);
        let foreign_link = Box::new(buf.deserialize()?);
        let user = Box::new(buf.deserialize()?);

        Ok(ContactsLink {
            my_link,
            foreign_link,
            user,
        })
    }
}

impl Deserializable for LangPackLanguage {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x117698f1);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<LangPackLanguage> {
                let name = buf.deserialize()?;
        let native_name = buf.deserialize()?;
        let lang_code = buf.deserialize()?;

        Ok(LangPackLanguage {
            name,
            native_name,
            lang_code,
        })
    }
}

impl Deserializable for DestroyAuthKeyRes {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<DestroyAuthKeyRes> {
        match id {
            0x0a9f2259_u32 => {
            
                Ok(DestroyAuthKeyRes::None {
                })
            },

            0xf660e1d4_u32 => {
            
                Ok(DestroyAuthKeyRes::Ok {
                })
            },

            0xea109b13_u32 => {
            
                Ok(DestroyAuthKeyRes::Fail {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ChannelsFeedSources {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChannelsFeedSources> {
        match id {
            0x8e8bca3d_u32 => {
                let flags = buf.deserialize()?;

                let newly_joined_feed = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let feeds = buf.deserialize()?;
                let chats = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(ChannelsFeedSources::Sources {
                    flags,
                    newly_joined_feed,
                    feeds,
                    chats,
                    users,
                })
            },

            0x88b12a17_u32 => {
            
                Ok(ChannelsFeedSources::NotModified {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for PhonePhoneCall {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xec82e140);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PhonePhoneCall> {
                let phone_call = Box::new(buf.deserialize()?);
        let users = buf.deserialize()?;

        Ok(PhonePhoneCall {
            phone_call,
            users,
        })
    }
}

impl Deserializable for AccountAuthorizations {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x1250abde);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<AccountAuthorizations> {
                let authorizations = buf.deserialize()?;

        Ok(AccountAuthorizations {
            authorizations,
        })
    }
}

impl Deserializable for NotifyPeer {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<NotifyPeer> {
        match id {
            0xb4c83b4c_u32 => {
            
                Ok(NotifyPeer::Users {
                })
            },

            0x9fd40bd8_u32 => {
                let peer = Box::new(buf.deserialize()?);
            
                Ok(NotifyPeer::Peer {
                    peer,
                })
            },

            0x74d07c60_u32 => {
            
                Ok(NotifyPeer::All {
                })
            },

            0xc007cec3_u32 => {
            
                Ok(NotifyPeer::Chats {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for BotInfo {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x98e81d3a);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<BotInfo> {
                let user_id = buf.deserialize()?;
        let description = buf.deserialize()?;
        let commands = buf.deserialize()?;

        Ok(BotInfo {
            user_id,
            description,
            commands,
        })
    }
}

impl Deserializable for PhotosPhoto {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x20212ca8);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PhotosPhoto> {
                let photo = Box::new(buf.deserialize()?);
        let users = buf.deserialize()?;

        Ok(PhotosPhoto {
            photo,
            users,
        })
    }
}

impl Deserializable for Authorization {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x7bf2e6f6);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Authorization> {
                let hash = buf.deserialize()?;
        let flags = buf.deserialize()?;
        let device_model = buf.deserialize()?;
        let platform = buf.deserialize()?;
        let system_version = buf.deserialize()?;
        let api_id = buf.deserialize()?;
        let app_name = buf.deserialize()?;
        let app_version = buf.deserialize()?;
        let date_created = buf.deserialize()?;
        let date_active = buf.deserialize()?;
        let ip = buf.deserialize()?;
        let country = buf.deserialize()?;
        let region = buf.deserialize()?;

        Ok(Authorization {
            hash,
            flags,
            device_model,
            platform,
            system_version,
            api_id,
            app_name,
            app_version,
            date_created,
            date_active,
            ip,
            country,
            region,
        })
    }
}

impl Deserializable for MessagesDialogs {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesDialogs> {
        match id {
            0x15ba6c40_u32 => {
                let dialogs = buf.deserialize()?;
                let messages = buf.deserialize()?;
                let chats = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(MessagesDialogs::Dialogs {
                    dialogs,
                    messages,
                    chats,
                    users,
                })
            },

            0x71e094f3_u32 => {
                let count = buf.deserialize()?;
                let dialogs = buf.deserialize()?;
                let messages = buf.deserialize()?;
                let chats = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(MessagesDialogs::Slice {
                    count,
                    dialogs,
                    messages,
                    chats,
                    users,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for LangPackString {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<LangPackString> {
        match id {
            0xcad181f6_u32 => {
                let key = buf.deserialize()?;
                let value = buf.deserialize()?;
            
                Ok(LangPackString::String {
                    key,
                    value,
                })
            },

            0x2979eeb2_u32 => {
                let key = buf.deserialize()?;
            
                Ok(LangPackString::Deleted {
                    key,
                })
            },

            0x6c47ac9f_u32 => {
                let flags = buf.deserialize()?;
                let key = buf.deserialize()?;

                let zero_value = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let one_value = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let two_value = if flags & 1 << 2 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let few_value = if flags & 1 << 3 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let many_value = if flags & 1 << 4 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let other_value = buf.deserialize()?;
            
                Ok(LangPackString::Pluralized {
                    flags,
                    key,
                    zero_value,
                    one_value,
                    two_value,
                    few_value,
                    many_value,
                    other_value,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for AccountDaysTtl {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xb8d0afdf);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<AccountDaysTtl> {
                let days = buf.deserialize()?;

        Ok(AccountDaysTtl {
            days,
        })
    }
}

impl Deserializable for MessagesAllStickers {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesAllStickers> {
        match id {
            0xedfd405f_u32 => {
                let hash = buf.deserialize()?;
                let sets = buf.deserialize()?;
            
                Ok(MessagesAllStickers::Stickers {
                    hash,
                    sets,
                })
            },

            0xe86602c3_u32 => {
            
                Ok(MessagesAllStickers::NotModified {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ContactBlocked {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x561bc879);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ContactBlocked> {
                let user_id = buf.deserialize()?;
        let date = buf.deserialize()?;

        Ok(ContactBlocked {
            user_id,
            date,
        })
    }
}

impl Deserializable for PopularContact {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x5ce14175);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PopularContact> {
                let client_id = buf.deserialize()?;
        let importers = buf.deserialize()?;

        Ok(PopularContact {
            client_id,
            importers,
        })
    }
}

impl Deserializable for MessagesBotResults {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x947ca848);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesBotResults> {
                let flags = buf.deserialize()?;

        let gallery = flags & 1 << 0 != 0;
        let query_id = buf.deserialize()?;

        let next_offset = if flags & 1 << 1 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        let switch_pm = if flags & 1 << 2 != 0 {
            Some(Box::new(buf.deserialize()?))
        } else {
            None
        };
        let results = buf.deserialize()?;
        let cache_time = buf.deserialize()?;
        let users = buf.deserialize()?;

        Ok(MessagesBotResults {
            flags,
            gallery,
            query_id,
            next_offset,
            switch_pm,
            results,
            cache_time,
            users,
        })
    }
}

impl Deserializable for MessageRange {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x0ae30253);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessageRange> {
                let min_id = buf.deserialize()?;
        let max_id = buf.deserialize()?;

        Ok(MessageRange {
            min_id,
            max_id,
        })
    }
}

impl Deserializable for UpdatesDifference {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<UpdatesDifference> {
        match id {
            0x00f49ca0_u32 => {
                let new_messages = buf.deserialize()?;
                let new_encrypted_messages = buf.deserialize()?;
                let other_updates = buf.deserialize()?;
                let chats = buf.deserialize()?;
                let users = buf.deserialize()?;
                let state = Box::new(buf.deserialize()?);
            
                Ok(UpdatesDifference::Difference {
                    new_messages,
                    new_encrypted_messages,
                    other_updates,
                    chats,
                    users,
                    state,
                })
            },

            0xa8fb1981_u32 => {
                let new_messages = buf.deserialize()?;
                let new_encrypted_messages = buf.deserialize()?;
                let other_updates = buf.deserialize()?;
                let chats = buf.deserialize()?;
                let users = buf.deserialize()?;
                let intermediate_state = Box::new(buf.deserialize()?);
            
                Ok(UpdatesDifference::Slice {
                    new_messages,
                    new_encrypted_messages,
                    other_updates,
                    chats,
                    users,
                    intermediate_state,
                })
            },

            0x5d75a138_u32 => {
                let date = buf.deserialize()?;
                let seq = buf.deserialize()?;
            
                Ok(UpdatesDifference::Empty {
                    date,
                    seq,
                })
            },

            0x4afe8f6d_u32 => {
                let pts = buf.deserialize()?;
            
                Ok(UpdatesDifference::TooLong {
                    pts,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MessagesBotCallbackAnswer {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x36585ea4);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesBotCallbackAnswer> {
                let flags = buf.deserialize()?;

        let alert = flags & 1 << 1 != 0;

        let has_url = flags & 1 << 3 != 0;

        let native_ui = flags & 1 << 4 != 0;

        let message = if flags & 1 << 0 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        let url = if flags & 1 << 2 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };
        let cache_time = buf.deserialize()?;

        Ok(MessagesBotCallbackAnswer {
            flags,
            alert,
            has_url,
            native_ui,
            message,
            url,
            cache_time,
        })
    }
}

impl Deserializable for AuthAuthorization {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xcd050916);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<AuthAuthorization> {
                let flags = buf.deserialize()?;

        let tmp_sessions = if flags & 1 << 0 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };
        let user = Box::new(buf.deserialize()?);

        Ok(AuthAuthorization {
            flags,
            tmp_sessions,
            user,
        })
    }
}

impl Deserializable for StorageFileType {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<StorageFileType> {
        match id {
            0xaa963b05_u32 => {
            
                Ok(StorageFileType::Unknown {
                })
            },

            0x007efe0e_u32 => {
            
                Ok(StorageFileType::Jpeg {
                })
            },

            0xae1e508d_u32 => {
            
                Ok(StorageFileType::Pdf {
                })
            },

            0x4b09ebbc_u32 => {
            
                Ok(StorageFileType::Mov {
                })
            },

            0xcae1aadf_u32 => {
            
                Ok(StorageFileType::Gif {
                })
            },

            0x528a0677_u32 => {
            
                Ok(StorageFileType::Mp3 {
                })
            },

            0x0a4f63c0_u32 => {
            
                Ok(StorageFileType::Png {
                })
            },

            0xb3cea0e4_u32 => {
            
                Ok(StorageFileType::Mp4 {
                })
            },

            0x40bc6f52_u32 => {
            
                Ok(StorageFileType::Partial {
                })
            },

            0x1081464c_u32 => {
            
                Ok(StorageFileType::Webp {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for AuthExportedAuthorization {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xdf969c2d);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<AuthExportedAuthorization> {
                let id = buf.deserialize()?;
        let bytes = buf.deserialize()?;

        Ok(AuthExportedAuthorization {
            id,
            bytes,
        })
    }
}

impl Deserializable for InlineBotSwitchPm {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x3c20629f);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InlineBotSwitchPm> {
                let text = buf.deserialize()?;
        let start_param = buf.deserialize()?;

        Ok(InlineBotSwitchPm {
            text,
            start_param,
        })
    }
}

impl Deserializable for MessagesStickerSet {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xb60a24a6);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesStickerSet> {
                let set = Box::new(buf.deserialize()?);
        let packs = buf.deserialize()?;
        let documents = buf.deserialize()?;

        Ok(MessagesStickerSet {
            set,
            packs,
            documents,
        })
    }
}

impl Deserializable for StickerSet {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xcd303b41);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<StickerSet> {
                let flags = buf.deserialize()?;

        let installed = flags & 1 << 0 != 0;

        let archived = flags & 1 << 1 != 0;

        let official = flags & 1 << 2 != 0;

        let masks = flags & 1 << 3 != 0;
        let id = buf.deserialize()?;
        let access_hash = buf.deserialize()?;
        let title = buf.deserialize()?;
        let short_name = buf.deserialize()?;
        let count = buf.deserialize()?;
        let hash = buf.deserialize()?;

        Ok(StickerSet {
            flags,
            installed,
            archived,
            official,
            masks,
            id,
            access_hash,
            title,
            short_name,
            count,
            hash,
        })
    }
}

impl Deserializable for PhotoSize {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PhotoSize> {
        match id {
            0x77bfb61b_u32 => {
                let type_ = buf.deserialize()?;
                let location = Box::new(buf.deserialize()?);
                let w = buf.deserialize()?;
                let h = buf.deserialize()?;
                let size = buf.deserialize()?;
            
                Ok(PhotoSize::Size {
                    type_,
                    location,
                    w,
                    h,
                    size,
                })
            },

            0x0e17e23c_u32 => {
                let type_ = buf.deserialize()?;
            
                Ok(PhotoSize::Empty {
                    type_,
                })
            },

            0xe9a734fa_u32 => {
                let type_ = buf.deserialize()?;
                let location = Box::new(buf.deserialize()?);
                let w = buf.deserialize()?;
                let h = buf.deserialize()?;
                let bytes = buf.deserialize()?;
            
                Ok(PhotoSize::Cached {
                    type_,
                    location,
                    w,
                    h,
                    bytes,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputGeoPoint {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputGeoPoint> {
        match id {
            0xf3b7acc9_u32 => {
                let lat = buf.deserialize()?;
                let long = buf.deserialize()?;
            
                Ok(InputGeoPoint::Point {
                    lat,
                    long,
                })
            },

            0xe4c123d6_u32 => {
            
                Ok(InputGeoPoint::Empty {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for AuthPasswordRecovery {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x137948a5);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<AuthPasswordRecovery> {
                let email_pattern = buf.deserialize()?;

        Ok(AuthPasswordRecovery {
            email_pattern,
        })
    }
}

impl Deserializable for InputAppEvent {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x770656a8);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputAppEvent> {
                let time = buf.deserialize()?;
        let type_ = buf.deserialize()?;
        let peer = buf.deserialize()?;
        let data = buf.deserialize()?;

        Ok(InputAppEvent {
            time,
            type_,
            peer,
            data,
        })
    }
}

impl Deserializable for InputUser {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputUser> {
        match id {
            0xb98886cf_u32 => {
            
                Ok(InputUser::Empty {
                })
            },

            0xf7c1b13f_u32 => {
            
                Ok(InputUser::Self_ {
                })
            },

            0xd8292816_u32 => {
                let user_id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
            
                Ok(InputUser::User {
                    user_id,
                    access_hash,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for Pong {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x347773c5);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Pong> {
                let msg_id = buf.deserialize()?;
        let ping_id = buf.deserialize()?;

        Ok(Pong {
            msg_id,
            ping_id,
        })
    }
}

impl Deserializable for MessagesAffectedHistory {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xb45c69d1);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesAffectedHistory> {
                let pts = buf.deserialize()?;
        let pts_count = buf.deserialize()?;
        let offset = buf.deserialize()?;

        Ok(MessagesAffectedHistory {
            pts,
            pts_count,
            offset,
        })
    }
}

impl Deserializable for Page {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Page> {
        match id {
            0x556ec7aa_u32 => {
                let blocks = buf.deserialize()?;
                let photos = buf.deserialize()?;
                let documents = buf.deserialize()?;
            
                Ok(Page::Full {
                    blocks,
                    photos,
                    documents,
                })
            },

            0x8e3f9ebe_u32 => {
                let blocks = buf.deserialize()?;
                let photos = buf.deserialize()?;
                let documents = buf.deserialize()?;
            
                Ok(Page::Part {
                    blocks,
                    photos,
                    documents,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for BotInlineResult {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<BotInlineResult> {
        match id {
            0x17db940b_u32 => {
                let flags = buf.deserialize()?;
                let id = buf.deserialize()?;
                let type_ = buf.deserialize()?;

                let photo = if flags & 1 << 0 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let document = if flags & 1 << 1 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let title = if flags & 1 << 2 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let description = if flags & 1 << 3 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let send_message = Box::new(buf.deserialize()?);
            
                Ok(BotInlineResult::Media {
                    flags,
                    id,
                    type_,
                    photo,
                    document,
                    title,
                    description,
                    send_message,
                })
            },

            0x9bebaeb9_u32 => {
                let flags = buf.deserialize()?;
                let id = buf.deserialize()?;
                let type_ = buf.deserialize()?;

                let title = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let description = if flags & 1 << 2 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let url = if flags & 1 << 3 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let thumb_url = if flags & 1 << 4 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let content_url = if flags & 1 << 5 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let content_type = if flags & 1 << 5 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let w = if flags & 1 << 6 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let h = if flags & 1 << 6 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let duration = if flags & 1 << 7 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let send_message = Box::new(buf.deserialize()?);
            
                Ok(BotInlineResult::Result {
                    flags,
                    id,
                    type_,
                    title,
                    description,
                    url,
                    thumb_url,
                    content_url,
                    content_type,
                    w,
                    h,
                    duration,
                    send_message,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for DcOption {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x05d8c6cc);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<DcOption> {
                let flags = buf.deserialize()?;

        let ipv6 = flags & 1 << 0 != 0;

        let media_only = flags & 1 << 1 != 0;

        let tcpo_only = flags & 1 << 2 != 0;

        let cdn = flags & 1 << 3 != 0;

        let static_ = flags & 1 << 4 != 0;
        let id = buf.deserialize()?;
        let ip_address = buf.deserialize()?;
        let port = buf.deserialize()?;

        Ok(DcOption {
            flags,
            ipv6,
            media_only,
            tcpo_only,
            cdn,
            static_,
            id,
            ip_address,
            port,
        })
    }
}

impl Deserializable for LangPackDifference {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xf385c1f6);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<LangPackDifference> {
                let lang_code = buf.deserialize()?;
        let from_version = buf.deserialize()?;
        let version = buf.deserialize()?;
        let strings = buf.deserialize()?;

        Ok(LangPackDifference {
            lang_code,
            from_version,
            version,
            strings,
        })
    }
}

impl Deserializable for PhoneCallProtocol {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xa2bb35cb);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PhoneCallProtocol> {
                let flags = buf.deserialize()?;

        let udp_p2p = flags & 1 << 0 != 0;

        let udp_reflector = flags & 1 << 1 != 0;
        let min_layer = buf.deserialize()?;
        let max_layer = buf.deserialize()?;

        Ok(PhoneCallProtocol {
            flags,
            udp_p2p,
            udp_reflector,
            min_layer,
            max_layer,
        })
    }
}

impl Deserializable for SetClientDhParamsAnswer {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<SetClientDhParamsAnswer> {
        match id {
            0x3bcbf734_u32 => {
                let nonce = buf.deserialize()?;
                let server_nonce = buf.deserialize()?;
                let new_nonce_hash1 = buf.deserialize()?;
            
                Ok(SetClientDhParamsAnswer::DhGenOk {
                    nonce,
                    server_nonce,
                    new_nonce_hash1,
                })
            },

            0xa69dae02_u32 => {
                let nonce = buf.deserialize()?;
                let server_nonce = buf.deserialize()?;
                let new_nonce_hash3 = buf.deserialize()?;
            
                Ok(SetClientDhParamsAnswer::DhGenFail {
                    nonce,
                    server_nonce,
                    new_nonce_hash3,
                })
            },

            0x46dc1fb9_u32 => {
                let nonce = buf.deserialize()?;
                let server_nonce = buf.deserialize()?;
                let new_nonce_hash2 = buf.deserialize()?;
            
                Ok(SetClientDhParamsAnswer::DhGenRetry {
                    nonce,
                    server_nonce,
                    new_nonce_hash2,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for Null {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x56730bcc);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Null> {
        
        Ok(Null {
        })
    }
}

impl Deserializable for ContactLink {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ContactLink> {
        match id {
            0xfeedd3ad_u32 => {
            
                Ok(ContactLink::None {
                })
            },

            0xd502c2d0_u32 => {
            
                Ok(ContactLink::Contact {
                })
            },

            0x268f3f59_u32 => {
            
                Ok(ContactLink::HasPhone {
                })
            },

            0x5f4f9247_u32 => {
            
                Ok(ContactLink::Unknown {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for LabeledPrice {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xcb296bf8);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<LabeledPrice> {
                let label = buf.deserialize()?;
        let amount = buf.deserialize()?;

        Ok(LabeledPrice {
            label,
            amount,
        })
    }
}

impl Deserializable for UploadFile {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<UploadFile> {
        match id {
            0xea52fe5a_u32 => {
                let dc_id = buf.deserialize()?;
                let file_token = buf.deserialize()?;
                let encryption_key = buf.deserialize()?;
                let encryption_iv = buf.deserialize()?;
                let cdn_file_hashes = buf.deserialize()?;
            
                Ok(UploadFile::CdnRedirect {
                    dc_id,
                    file_token,
                    encryption_key,
                    encryption_iv,
                    cdn_file_hashes,
                })
            },

            0x096a18d5_u32 => {
                let type_ = Box::new(buf.deserialize()?);
                let mtime = buf.deserialize()?;
                let bytes = buf.deserialize()?;
            
                Ok(UploadFile::File {
                    type_,
                    mtime,
                    bytes,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ChannelParticipant {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChannelParticipant> {
        match id {
            0xa82fa898_u32 => {
                let flags = buf.deserialize()?;

                let can_edit = flags & 1 << 0 != 0;
                let user_id = buf.deserialize()?;
                let inviter_id = buf.deserialize()?;
                let promoted_by = buf.deserialize()?;
                let date = buf.deserialize()?;
                let admin_rights = Box::new(buf.deserialize()?);
            
                Ok(ChannelParticipant::Admin {
                    flags,
                    can_edit,
                    user_id,
                    inviter_id,
                    promoted_by,
                    date,
                    admin_rights,
                })
            },

            0x15ebac1d_u32 => {
                let user_id = buf.deserialize()?;
                let date = buf.deserialize()?;
            
                Ok(ChannelParticipant::Participant {
                    user_id,
                    date,
                })
            },

            0xa3289a6d_u32 => {
                let user_id = buf.deserialize()?;
                let inviter_id = buf.deserialize()?;
                let date = buf.deserialize()?;
            
                Ok(ChannelParticipant::Self_ {
                    user_id,
                    inviter_id,
                    date,
                })
            },

            0x222c1886_u32 => {
                let flags = buf.deserialize()?;

                let left = flags & 1 << 0 != 0;
                let user_id = buf.deserialize()?;
                let kicked_by = buf.deserialize()?;
                let date = buf.deserialize()?;
                let banned_rights = Box::new(buf.deserialize()?);
            
                Ok(ChannelParticipant::Banned {
                    flags,
                    left,
                    user_id,
                    kicked_by,
                    date,
                    banned_rights,
                })
            },

            0xe3e2e1f9_u32 => {
                let user_id = buf.deserialize()?;
            
                Ok(ChannelParticipant::Creator {
                    user_id,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MsgsStateInfo {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x04deb57d);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MsgsStateInfo> {
                let req_msg_id = buf.deserialize()?;
        let info = buf.deserialize()?;

        Ok(MsgsStateInfo {
            req_msg_id,
            info,
        })
    }
}

impl Deserializable for InputDocument {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputDocument> {
        match id {
            0x18798952_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
            
                Ok(InputDocument::Document {
                    id,
                    access_hash,
                })
            },

            0x72f0eaae_u32 => {
            
                Ok(InputDocument::Empty {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for UploadWebFile {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x21e753bc);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<UploadWebFile> {
                let size = buf.deserialize()?;
        let mime_type = buf.deserialize()?;
        let file_type = Box::new(buf.deserialize()?);
        let mtime = buf.deserialize()?;
        let bytes = buf.deserialize()?;

        Ok(UploadWebFile {
            size,
            mime_type,
            file_type,
            mtime,
            bytes,
        })
    }
}

impl Deserializable for PostAddress {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x1e8caaeb);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PostAddress> {
                let street_line1 = buf.deserialize()?;
        let street_line2 = buf.deserialize()?;
        let city = buf.deserialize()?;
        let state = buf.deserialize()?;
        let country_iso2 = buf.deserialize()?;
        let post_code = buf.deserialize()?;

        Ok(PostAddress {
            street_line1,
            street_line2,
            city,
            state,
            country_iso2,
            post_code,
        })
    }
}

impl Deserializable for FeedBroadcasts {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<FeedBroadcasts> {
        match id {
            0x4f4feaf1_u32 => {
                let feed_id = buf.deserialize()?;
                let channels = buf.deserialize()?;
            
                Ok(FeedBroadcasts::Broadcasts {
                    feed_id,
                    channels,
                })
            },

            0x9a687cba_u32 => {
                let channels = buf.deserialize()?;
            
                Ok(FeedBroadcasts::Ungrouped {
                    channels,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MessagesMessageEditData {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x26b5dde6);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesMessageEditData> {
                let flags = buf.deserialize()?;

        let caption = flags & 1 << 0 != 0;

        Ok(MessagesMessageEditData {
            flags,
            caption,
        })
    }
}

impl Deserializable for HelpAppUpdate {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<HelpAppUpdate> {
        match id {
            0x8987f311_u32 => {
                let id = buf.deserialize()?;
                let critical = buf.deserialize()?;
                let url = buf.deserialize()?;
                let text = buf.deserialize()?;
            
                Ok(HelpAppUpdate::Update {
                    id,
                    critical,
                    url,
                    text,
                })
            },

            0xc45a6536_u32 => {
            
                Ok(HelpAppUpdate::No {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for PaymentsPaymentReceipt {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x500911e1);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PaymentsPaymentReceipt> {
                let flags = buf.deserialize()?;
        let date = buf.deserialize()?;
        let bot_id = buf.deserialize()?;
        let invoice = Box::new(buf.deserialize()?);
        let provider_id = buf.deserialize()?;

        let info = if flags & 1 << 0 != 0 {
            Some(Box::new(buf.deserialize()?))
        } else {
            None
        };

        let shipping = if flags & 1 << 1 != 0 {
            Some(Box::new(buf.deserialize()?))
        } else {
            None
        };
        let currency = buf.deserialize()?;
        let total_amount = buf.deserialize()?;
        let credentials_title = buf.deserialize()?;
        let users = buf.deserialize()?;

        Ok(PaymentsPaymentReceipt {
            flags,
            date,
            bot_id,
            invoice,
            provider_id,
            info,
            shipping,
            currency,
            total_amount,
            credentials_title,
            users,
        })
    }
}

impl Deserializable for InputStickerSetItem {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xffa0a496);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputStickerSetItem> {
                let flags = buf.deserialize()?;
        let document = Box::new(buf.deserialize()?);
        let emoji = buf.deserialize()?;

        let mask_coords = if flags & 1 << 0 != 0 {
            Some(Box::new(buf.deserialize()?))
        } else {
            None
        };

        Ok(InputStickerSetItem {
            flags,
            document,
            emoji,
            mask_coords,
        })
    }
}

impl Deserializable for ChatParticipants {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChatParticipants> {
        match id {
            0x3f460fed_u32 => {
                let chat_id = buf.deserialize()?;
                let participants = buf.deserialize()?;
                let version = buf.deserialize()?;
            
                Ok(ChatParticipants::Participants {
                    chat_id,
                    participants,
                    version,
                })
            },

            0xfc900c2b_u32 => {
                let flags = buf.deserialize()?;
                let chat_id = buf.deserialize()?;

                let self_participant = if flags & 1 << 0 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(ChatParticipants::Forbidden {
                    flags,
                    chat_id,
                    self_participant,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ChannelAdminLogEvent {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x3b5a3e40);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChannelAdminLogEvent> {
                let id = buf.deserialize()?;
        let date = buf.deserialize()?;
        let user_id = buf.deserialize()?;
        let action = Box::new(buf.deserialize()?);

        Ok(ChannelAdminLogEvent {
            id,
            date,
            user_id,
            action,
        })
    }
}

impl Deserializable for FutureSalts {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xae500895);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<FutureSalts> {
                let req_msg_id = buf.deserialize()?;
        let now = buf.deserialize()?;
        let salts = buf.deserialize()?;

        Ok(FutureSalts {
            req_msg_id,
            now,
            salts,
        })
    }
}

impl Deserializable for RecentMeUrl {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<RecentMeUrl> {
        match id {
            0xbc0a57dc_u32 => {
                let url = buf.deserialize()?;
                let set = Box::new(buf.deserialize()?);
            
                Ok(RecentMeUrl::StickerSet {
                    url,
                    set,
                })
            },

            0xeb49081d_u32 => {
                let url = buf.deserialize()?;
                let chat_invite = Box::new(buf.deserialize()?);
            
                Ok(RecentMeUrl::ChatInvite {
                    url,
                    chat_invite,
                })
            },

            0x8dbc3336_u32 => {
                let url = buf.deserialize()?;
                let user_id = buf.deserialize()?;
            
                Ok(RecentMeUrl::User {
                    url,
                    user_id,
                })
            },

            0xa01b22f9_u32 => {
                let url = buf.deserialize()?;
                let chat_id = buf.deserialize()?;
            
                Ok(RecentMeUrl::Chat {
                    url,
                    chat_id,
                })
            },

            0x46e1d13d_u32 => {
                let url = buf.deserialize()?;
            
                Ok(RecentMeUrl::Unknown {
                    url,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for CdnConfig {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x5725e40a);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<CdnConfig> {
                let public_keys = buf.deserialize()?;

        Ok(CdnConfig {
            public_keys,
        })
    }
}

impl Deserializable for MessagesDhConfig {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesDhConfig> {
        match id {
            0xc0e24635_u32 => {
                let random = buf.deserialize()?;
            
                Ok(MessagesDhConfig::NotModified {
                    random,
                })
            },

            0x2c221edd_u32 => {
                let g = buf.deserialize()?;
                let p = buf.deserialize()?;
                let version = buf.deserialize()?;
                let random = buf.deserialize()?;
            
                Ok(MessagesDhConfig::Config {
                    g,
                    p,
                    version,
                    random,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ContactsBlocked {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ContactsBlocked> {
        match id {
            0x1c138d15_u32 => {
                let blocked = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(ContactsBlocked::Blocked {
                    blocked,
                    users,
                })
            },

            0x900802a1_u32 => {
                let count = buf.deserialize()?;
                let blocked = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(ContactsBlocked::Slice {
                    count,
                    blocked,
                    users,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for AccountPassword {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<AccountPassword> {
        match id {
            0x7c18141c_u32 => {
                let current_salt = buf.deserialize()?;
                let new_salt = buf.deserialize()?;
                let hint = buf.deserialize()?;
                let has_recovery = buf.deserialize()?;
                let email_unconfirmed_pattern = buf.deserialize()?;
            
                Ok(AccountPassword::Password {
                    current_salt,
                    new_salt,
                    hint,
                    has_recovery,
                    email_unconfirmed_pattern,
                })
            },

            0x96dabc18_u32 => {
                let new_salt = buf.deserialize()?;
                let email_unconfirmed_pattern = buf.deserialize()?;
            
                Ok(AccountPassword::No {
                    new_salt,
                    email_unconfirmed_pattern,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ReportReason {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ReportReason> {
        match id {
            0x2e59d922_u32 => {
            
                Ok(ReportReason::InputPornography {
                })
            },

            0x1e22c78d_u32 => {
            
                Ok(ReportReason::InputViolence {
                })
            },

            0x58dbcab8_u32 => {
            
                Ok(ReportReason::InputSpam {
                })
            },

            0xe1746d0a_u32 => {
                let text = buf.deserialize()?;
            
                Ok(ReportReason::InputOther {
                    text,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MaskCoords {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xaed6dbb2);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MaskCoords> {
                let n = buf.deserialize()?;
        let x = buf.deserialize()?;
        let y = buf.deserialize()?;
        let zoom = buf.deserialize()?;

        Ok(MaskCoords {
            n,
            x,
            y,
            zoom,
        })
    }
}

impl Deserializable for ResPq {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x05162463);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ResPq> {
                let nonce = buf.deserialize()?;
        let server_nonce = buf.deserialize()?;
        let pq = buf.deserialize()?;
        let server_public_key_fingerprints = buf.deserialize()?;

        Ok(ResPq {
            nonce,
            server_nonce,
            pq,
            server_public_key_fingerprints,
        })
    }
}

impl Deserializable for AuthCodeType {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<AuthCodeType> {
        match id {
            0x226ccefb_u32 => {
            
                Ok(AuthCodeType::FlashCall {
                })
            },

            0x741cd3e3_u32 => {
            
                Ok(AuthCodeType::Call {
                })
            },

            0x72a3158c_u32 => {
            
                Ok(AuthCodeType::Sms {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MessagesStickers {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesStickers> {
        match id {
            0x8a8ecd32_u32 => {
                let hash = buf.deserialize()?;
                let stickers = buf.deserialize()?;
            
                Ok(MessagesStickers::Stickers {
                    hash,
                    stickers,
                })
            },

            0xf1749a22_u32 => {
            
                Ok(MessagesStickers::NotModified {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for FoundGif {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<FoundGif> {
        match id {
            0x9c750409_u32 => {
                let url = buf.deserialize()?;
                let photo = Box::new(buf.deserialize()?);
                let document = Box::new(buf.deserialize()?);
            
                Ok(FoundGif::Cached {
                    url,
                    photo,
                    document,
                })
            },

            0x162ecc1f_u32 => {
                let url = buf.deserialize()?;
                let thumb_url = buf.deserialize()?;
                let content_url = buf.deserialize()?;
                let content_type = buf.deserialize()?;
                let w = buf.deserialize()?;
                let h = buf.deserialize()?;
            
                Ok(FoundGif::Gif {
                    url,
                    thumb_url,
                    content_url,
                    content_type,
                    w,
                    h,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MessagesMessages {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesMessages> {
        match id {
            0x74535f21_u32 => {
                let count = buf.deserialize()?;
            
                Ok(MessagesMessages::MessagesNotModified {
                    count,
                })
            },

            0x0b446ae3_u32 => {
                let count = buf.deserialize()?;
                let messages = buf.deserialize()?;
                let chats = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(MessagesMessages::MessagesSlice {
                    count,
                    messages,
                    chats,
                    users,
                })
            },

            0x8c718e87_u32 => {
                let messages = buf.deserialize()?;
                let chats = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(MessagesMessages::Messages {
                    messages,
                    chats,
                    users,
                })
            },

            0x99262e37_u32 => {
                let flags = buf.deserialize()?;
                let pts = buf.deserialize()?;
                let count = buf.deserialize()?;
                let messages = buf.deserialize()?;
                let chats = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(MessagesMessages::ChannelMessages {
                    flags,
                    pts,
                    count,
                    messages,
                    chats,
                    users,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ExportedMessageLink {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x5dab1af4);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ExportedMessageLink> {
                let link = buf.deserialize()?;
        let html = buf.deserialize()?;

        Ok(ExportedMessageLink {
            link,
            html,
        })
    }
}

impl Deserializable for AccountPasswordInputSettings {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x86916deb);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<AccountPasswordInputSettings> {
                let flags = buf.deserialize()?;

        let new_salt = if flags & 1 << 0 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        let new_password_hash = if flags & 1 << 0 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        let hint = if flags & 1 << 0 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        let email = if flags & 1 << 1 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        Ok(AccountPasswordInputSettings {
            flags,
            new_salt,
            new_password_hash,
            hint,
            email,
        })
    }
}

impl Deserializable for UserFull {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x0f220f3f);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<UserFull> {
                let flags = buf.deserialize()?;

        let blocked = flags & 1 << 0 != 0;

        let phone_calls_available = flags & 1 << 4 != 0;

        let phone_calls_private = flags & 1 << 5 != 0;
        let user = Box::new(buf.deserialize()?);

        let about = if flags & 1 << 1 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };
        let link = Box::new(buf.deserialize()?);

        let profile_photo = if flags & 1 << 2 != 0 {
            Some(Box::new(buf.deserialize()?))
        } else {
            None
        };
        let notify_settings = Box::new(buf.deserialize()?);

        let bot_info = if flags & 1 << 3 != 0 {
            Some(Box::new(buf.deserialize()?))
        } else {
            None
        };
        let common_chats_count = buf.deserialize()?;

        Ok(UserFull {
            flags,
            blocked,
            phone_calls_available,
            phone_calls_private,
            user,
            about,
            link,
            profile_photo,
            notify_settings,
            bot_info,
            common_chats_count,
        })
    }
}

impl Deserializable for DisabledFeature {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xae636f24);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<DisabledFeature> {
                let feature = buf.deserialize()?;
        let description = buf.deserialize()?;

        Ok(DisabledFeature {
            feature,
            description,
        })
    }
}

impl Deserializable for DataJson {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x7d748d04);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<DataJson> {
                let data = buf.deserialize()?;

        Ok(DataJson {
            data,
        })
    }
}

impl Deserializable for WebPage {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<WebPage> {
        match id {
            0x5f07b4bc_u32 => {
                let flags = buf.deserialize()?;
                let id = buf.deserialize()?;
                let url = buf.deserialize()?;
                let display_url = buf.deserialize()?;
                let hash = buf.deserialize()?;

                let type_ = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let site_name = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let title = if flags & 1 << 2 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let description = if flags & 1 << 3 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let photo = if flags & 1 << 4 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let embed_url = if flags & 1 << 5 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let embed_type = if flags & 1 << 5 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let embed_width = if flags & 1 << 6 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let embed_height = if flags & 1 << 6 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let duration = if flags & 1 << 7 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let author = if flags & 1 << 8 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let document = if flags & 1 << 9 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let cached_page = if flags & 1 << 10 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
            
                Ok(WebPage::Page {
                    flags,
                    id,
                    url,
                    display_url,
                    hash,
                    type_,
                    site_name,
                    title,
                    description,
                    photo,
                    embed_url,
                    embed_type,
                    embed_width,
                    embed_height,
                    duration,
                    author,
                    document,
                    cached_page,
                })
            },

            0xc586da1c_u32 => {
                let id = buf.deserialize()?;
                let date = buf.deserialize()?;
            
                Ok(WebPage::Pending {
                    id,
                    date,
                })
            },

            0xeb1477e8_u32 => {
                let id = buf.deserialize()?;
            
                Ok(WebPage::Empty {
                    id,
                })
            },

            0x85849473_u32 => {
            
                Ok(WebPage::NotModified {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputStickerSet {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputStickerSet> {
        match id {
            0x861cc8a0_u32 => {
                let short_name = buf.deserialize()?;
            
                Ok(InputStickerSet::ShortName {
                    short_name,
                })
            },

            0x9de7a269_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
            
                Ok(InputStickerSet::Id {
                    id,
                    access_hash,
                })
            },

            0xffb62b95_u32 => {
            
                Ok(InputStickerSet::Empty {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputDialogPeer {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputDialogPeer> {
        match id {
            0x2c38b8cf_u32 => {
                let feed_id = buf.deserialize()?;
            
                Ok(InputDialogPeer::Feed {
                    feed_id,
                })
            },

            0xfcaafeb7_u32 => {
                let peer = Box::new(buf.deserialize()?);
            
                Ok(InputDialogPeer::Peer {
                    peer,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MessagesFeedMessages {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesFeedMessages> {
        match id {
            0x55c3a1b1_u32 => {
                let flags = buf.deserialize()?;

                let max_position = if flags & 1 << 0 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let min_position = if flags & 1 << 1 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let read_max_position = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };
                let messages = buf.deserialize()?;
                let chats = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(MessagesFeedMessages::Messages {
                    flags,
                    max_position,
                    min_position,
                    read_max_position,
                    messages,
                    chats,
                    users,
                })
            },

            0x4678d0cf_u32 => {
            
                Ok(MessagesFeedMessages::NotModified {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for PhotosPhotos {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PhotosPhotos> {
        match id {
            0x15051f54_u32 => {
                let count = buf.deserialize()?;
                let photos = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(PhotosPhotos::PhotosSlice {
                    count,
                    photos,
                    users,
                })
            },

            0x8dca6aa5_u32 => {
                let photos = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(PhotosPhotos::Photos {
                    photos,
                    users,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for PeerNotifyEvents {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PeerNotifyEvents> {
        match id {
            0x6d1ded88_u32 => {
            
                Ok(PeerNotifyEvents::All {
                })
            },

            0xadd53cb3_u32 => {
            
                Ok(PeerNotifyEvents::Empty {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ContactsTopPeers {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ContactsTopPeers> {
        match id {
            0xde266ef5_u32 => {
            
                Ok(ContactsTopPeers::NotModified {
                })
            },

            0x70b772a8_u32 => {
                let categories = buf.deserialize()?;
                let chats = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(ContactsTopPeers::Peers {
                    categories,
                    chats,
                    users,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for AccountTmpPassword {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xdb64fd34);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<AccountTmpPassword> {
                let tmp_password = buf.deserialize()?;
        let valid_until = buf.deserialize()?;

        Ok(AccountTmpPassword {
            tmp_password,
            valid_until,
        })
    }
}

impl Deserializable for ShippingOption {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xb6213cdf);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ShippingOption> {
                let id = buf.deserialize()?;
        let title = buf.deserialize()?;
        let prices = buf.deserialize()?;

        Ok(ShippingOption {
            id,
            title,
            prices,
        })
    }
}

impl Deserializable for FileLocation {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<FileLocation> {
        match id {
            0x7c596b46_u32 => {
                let volume_id = buf.deserialize()?;
                let local_id = buf.deserialize()?;
                let secret = buf.deserialize()?;
            
                Ok(FileLocation::Unavailable {
                    volume_id,
                    local_id,
                    secret,
                })
            },

            0x53d69076_u32 => {
                let dc_id = buf.deserialize()?;
                let volume_id = buf.deserialize()?;
                let local_id = buf.deserialize()?;
                let secret = buf.deserialize()?;
            
                Ok(FileLocation::Location {
                    dc_id,
                    volume_id,
                    local_id,
                    secret,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputFile {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputFile> {
        match id {
            0xf52ff27f_u32 => {
                let id = buf.deserialize()?;
                let parts = buf.deserialize()?;
                let name = buf.deserialize()?;
                let md5_checksum = buf.deserialize()?;
            
                Ok(InputFile::File {
                    id,
                    parts,
                    name,
                    md5_checksum,
                })
            },

            0xfa4f0bb5_u32 => {
                let id = buf.deserialize()?;
                let parts = buf.deserialize()?;
                let name = buf.deserialize()?;
            
                Ok(InputFile::Big {
                    id,
                    parts,
                    name,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MsgDetailedInfo {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MsgDetailedInfo> {
        match id {
            0x809db6df_u32 => {
                let answer_msg_id = buf.deserialize()?;
                let bytes = buf.deserialize()?;
                let status = buf.deserialize()?;
            
                Ok(MsgDetailedInfo::New {
                    answer_msg_id,
                    bytes,
                    status,
                })
            },

            0x276d3ec6_u32 => {
                let msg_id = buf.deserialize()?;
                let answer_msg_id = buf.deserialize()?;
                let bytes = buf.deserialize()?;
                let status = buf.deserialize()?;
            
                Ok(MsgDetailedInfo::Info {
                    msg_id,
                    answer_msg_id,
                    bytes,
                    status,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ChannelsChannelParticipants {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChannelsChannelParticipants> {
        match id {
            0xf56ee2a8_u32 => {
                let count = buf.deserialize()?;
                let participants = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(ChannelsChannelParticipants::Participants {
                    count,
                    participants,
                    users,
                })
            },

            0xf0173fe9_u32 => {
            
                Ok(ChannelsChannelParticipants::NotModified {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MessagesFeaturedStickers {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesFeaturedStickers> {
        match id {
            0x04ede3cf_u32 => {
            
                Ok(MessagesFeaturedStickers::NotModified {
                })
            },

            0xf89d88e5_u32 => {
                let hash = buf.deserialize()?;
                let sets = buf.deserialize()?;
                let unread = buf.deserialize()?;
            
                Ok(MessagesFeaturedStickers::Stickers {
                    hash,
                    sets,
                    unread,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ClientDhInnerData {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x6643b654);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ClientDhInnerData> {
                let nonce = buf.deserialize()?;
        let server_nonce = buf.deserialize()?;
        let retry_id = buf.deserialize()?;
        let g_b = buf.deserialize()?;

        Ok(ClientDhInnerData {
            nonce,
            server_nonce,
            retry_id,
            g_b,
        })
    }
}

impl Deserializable for GeoPoint {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<GeoPoint> {
        match id {
            0x2049d70c_u32 => {
                let long = buf.deserialize()?;
                let lat = buf.deserialize()?;
            
                Ok(GeoPoint::Point {
                    long,
                    lat,
                })
            },

            0x1117dd5f_u32 => {
            
                Ok(GeoPoint::Empty {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MessagesPeerDialogs {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x3371c354);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesPeerDialogs> {
                let dialogs = buf.deserialize()?;
        let messages = buf.deserialize()?;
        let chats = buf.deserialize()?;
        let users = buf.deserialize()?;
        let state = Box::new(buf.deserialize()?);

        Ok(MessagesPeerDialogs {
            dialogs,
            messages,
            chats,
            users,
            state,
        })
    }
}

impl Deserializable for AccountPrivacyRules {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x554abb6f);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<AccountPrivacyRules> {
                let rules = buf.deserialize()?;
        let users = buf.deserialize()?;

        Ok(AccountPrivacyRules {
            rules,
            users,
        })
    }
}

impl Deserializable for MsgsAck {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x62d6b459);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MsgsAck> {
                let msg_ids = buf.deserialize()?;

        Ok(MsgsAck {
            msg_ids,
        })
    }
}

impl Deserializable for WebDocument {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xc61acbd8);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<WebDocument> {
                let url = buf.deserialize()?;
        let access_hash = buf.deserialize()?;
        let size = buf.deserialize()?;
        let mime_type = buf.deserialize()?;
        let attributes = buf.deserialize()?;
        let dc_id = buf.deserialize()?;

        Ok(WebDocument {
            url,
            access_hash,
            size,
            mime_type,
            attributes,
            dc_id,
        })
    }
}

impl Deserializable for RichText {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<RichText> {
        match id {
            0x9bf8bb95_u32 => {
                let text = Box::new(buf.deserialize()?);
            
                Ok(RichText::Strike {
                    text,
                })
            },

            0xdc3d824f_u32 => {
            
                Ok(RichText::Empty {
                })
            },

            0x6c3f19b9_u32 => {
                let text = Box::new(buf.deserialize()?);
            
                Ok(RichText::Fixed {
                    text,
                })
            },

            0xd912a59c_u32 => {
                let text = Box::new(buf.deserialize()?);
            
                Ok(RichText::Italic {
                    text,
                })
            },

            0x6724abc4_u32 => {
                let text = Box::new(buf.deserialize()?);
            
                Ok(RichText::Bold {
                    text,
                })
            },

            0xc12622c4_u32 => {
                let text = Box::new(buf.deserialize()?);
            
                Ok(RichText::Underline {
                    text,
                })
            },

            0x7e6260d7_u32 => {
                let texts = buf.deserialize()?;
            
                Ok(RichText::Concat {
                    texts,
                })
            },

            0x3c2884c1_u32 => {
                let text = Box::new(buf.deserialize()?);
                let url = buf.deserialize()?;
                let webpage_id = buf.deserialize()?;
            
                Ok(RichText::Url {
                    text,
                    url,
                    webpage_id,
                })
            },

            0x744694e0_u32 => {
                let text = buf.deserialize()?;
            
                Ok(RichText::Plain {
                    text,
                })
            },

            0xde5a0dd6_u32 => {
                let text = Box::new(buf.deserialize()?);
                let email = buf.deserialize()?;
            
                Ok(RichText::Email {
                    text,
                    email,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for BadMsgNotification {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<BadMsgNotification> {
        match id {
            0xedab447b_u32 => {
                let bad_msg_id = buf.deserialize()?;
                let bad_msg_seqno = buf.deserialize()?;
                let error_code = buf.deserialize()?;
                let new_server_salt = buf.deserialize()?;
            
                Ok(BadMsgNotification::ServerSalt {
                    bad_msg_id,
                    bad_msg_seqno,
                    error_code,
                    new_server_salt,
                })
            },

            0xa7eff811_u32 => {
                let bad_msg_id = buf.deserialize()?;
                let bad_msg_seqno = buf.deserialize()?;
                let error_code = buf.deserialize()?;
            
                Ok(BadMsgNotification::Notification {
                    bad_msg_id,
                    bad_msg_seqno,
                    error_code,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ContactsContacts {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ContactsContacts> {
        match id {
            0xb74ba9d2_u32 => {
            
                Ok(ContactsContacts::ContactsNotModified {
                })
            },

            0xeae87e42_u32 => {
                let contacts = buf.deserialize()?;
                let saved_count = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(ContactsContacts::Contacts {
                    contacts,
                    saved_count,
                    users,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for HelpConfigSimple {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xd997c3c5);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<HelpConfigSimple> {
                let date = buf.deserialize()?;
        let expires = buf.deserialize()?;
        let dc_id = buf.deserialize()?;
        let ip_port_list = buf.deserialize()?;

        Ok(HelpConfigSimple {
            date,
            expires,
            dc_id,
            ip_port_list,
        })
    }
}

impl Deserializable for DestroySessionRes {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<DestroySessionRes> {
        match id {
            0x62d350c9_u32 => {
                let session_id = buf.deserialize()?;
            
                Ok(DestroySessionRes::None {
                    session_id,
                })
            },

            0xe22045fc_u32 => {
                let session_id = buf.deserialize()?;
            
                Ok(DestroySessionRes::Ok {
                    session_id,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ChatInvite {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChatInvite> {
        match id {
            0x5a686d7c_u32 => {
                let chat = Box::new(buf.deserialize()?);
            
                Ok(ChatInvite::Already {
                    chat,
                })
            },

            0xdb74f558_u32 => {
                let flags = buf.deserialize()?;

                let channel = flags & 1 << 0 != 0;

                let broadcast = flags & 1 << 1 != 0;

                let public = flags & 1 << 2 != 0;

                let megagroup = flags & 1 << 3 != 0;
                let title = buf.deserialize()?;
                let photo = Box::new(buf.deserialize()?);
                let participants_count = buf.deserialize()?;

                let participants = if flags & 1 << 4 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(ChatInvite::Invite {
                    flags,
                    channel,
                    broadcast,
                    public,
                    megagroup,
                    title,
                    photo,
                    participants_count,
                    participants,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for Message {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Message> {
        match id {
            0x9e19a1f6_u32 => {
                let flags = buf.deserialize()?;

                let out = flags & 1 << 1 != 0;

                let mentioned = flags & 1 << 4 != 0;

                let media_unread = flags & 1 << 5 != 0;

                let silent = flags & 1 << 13 != 0;

                let post = flags & 1 << 14 != 0;
                let id = buf.deserialize()?;

                let from_id = if flags & 1 << 8 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let to_id = Box::new(buf.deserialize()?);

                let reply_to_msg_id = if flags & 1 << 3 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let date = buf.deserialize()?;
                let action = Box::new(buf.deserialize()?);
            
                Ok(Message::Service {
                    flags,
                    out,
                    mentioned,
                    media_unread,
                    silent,
                    post,
                    id,
                    from_id,
                    to_id,
                    reply_to_msg_id,
                    date,
                    action,
                })
            },

            0x83e5de54_u32 => {
                let id = buf.deserialize()?;
            
                Ok(Message::Empty {
                    id,
                })
            },

            0x44f9b43d_u32 => {
                let flags = buf.deserialize()?;

                let out = flags & 1 << 1 != 0;

                let mentioned = flags & 1 << 4 != 0;

                let media_unread = flags & 1 << 5 != 0;

                let silent = flags & 1 << 13 != 0;

                let post = flags & 1 << 14 != 0;
                let id = buf.deserialize()?;

                let from_id = if flags & 1 << 8 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let to_id = Box::new(buf.deserialize()?);

                let fwd_from = if flags & 1 << 2 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let via_bot_id = if flags & 1 << 11 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let reply_to_msg_id = if flags & 1 << 3 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let date = buf.deserialize()?;
                let message = buf.deserialize()?;

                let media = if flags & 1 << 9 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let reply_markup = if flags & 1 << 6 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let entities = if flags & 1 << 7 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let views = if flags & 1 << 10 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let edit_date = if flags & 1 << 15 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let post_author = if flags & 1 << 16 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let grouped_id = if flags & 1 << 17 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(Message::Message {
                    flags,
                    out,
                    mentioned,
                    media_unread,
                    silent,
                    post,
                    id,
                    from_id,
                    to_id,
                    fwd_from,
                    via_bot_id,
                    reply_to_msg_id,
                    date,
                    message,
                    media,
                    reply_markup,
                    entities,
                    views,
                    edit_date,
                    post_author,
                    grouped_id,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputEncryptedChat {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xf141b5e1);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputEncryptedChat> {
                let chat_id = buf.deserialize()?;
        let access_hash = buf.deserialize()?;

        Ok(InputEncryptedChat {
            chat_id,
            access_hash,
        })
    }
}

impl Deserializable for Contact {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xf911c994);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Contact> {
                let user_id = buf.deserialize()?;
        let mutual = buf.deserialize()?;

        Ok(Contact {
            user_id,
            mutual,
        })
    }
}

impl Deserializable for UpdatesChannelDifference {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<UpdatesChannelDifference> {
        match id {
            0x3e11affb_u32 => {
                let flags = buf.deserialize()?;

                let final_ = flags & 1 << 0 != 0;
                let pts = buf.deserialize()?;

                let timeout = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(UpdatesChannelDifference::Empty {
                    flags,
                    final_,
                    pts,
                    timeout,
                })
            },

            0x2064674e_u32 => {
                let flags = buf.deserialize()?;

                let final_ = flags & 1 << 0 != 0;
                let pts = buf.deserialize()?;

                let timeout = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let new_messages = buf.deserialize()?;
                let other_updates = buf.deserialize()?;
                let chats = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(UpdatesChannelDifference::Difference {
                    flags,
                    final_,
                    pts,
                    timeout,
                    new_messages,
                    other_updates,
                    chats,
                    users,
                })
            },

            0x6a9d7b35_u32 => {
                let flags = buf.deserialize()?;

                let final_ = flags & 1 << 0 != 0;
                let pts = buf.deserialize()?;

                let timeout = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let top_message = buf.deserialize()?;
                let read_inbox_max_id = buf.deserialize()?;
                let read_outbox_max_id = buf.deserialize()?;
                let unread_count = buf.deserialize()?;
                let unread_mentions_count = buf.deserialize()?;
                let messages = buf.deserialize()?;
                let chats = buf.deserialize()?;
                let users = buf.deserialize()?;
            
                Ok(UpdatesChannelDifference::TooLong {
                    flags,
                    final_,
                    pts,
                    timeout,
                    top_message,
                    read_inbox_max_id,
                    read_outbox_max_id,
                    unread_count,
                    unread_mentions_count,
                    messages,
                    chats,
                    users,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputNotifyPeer {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputNotifyPeer> {
        match id {
            0xb8bc5b0c_u32 => {
                let peer = Box::new(buf.deserialize()?);
            
                Ok(InputNotifyPeer::Peer {
                    peer,
                })
            },

            0x4a95e84e_u32 => {
            
                Ok(InputNotifyPeer::Chats {
                })
            },

            0xa429b886_u32 => {
            
                Ok(InputNotifyPeer::All {
                })
            },

            0x193b4417_u32 => {
            
                Ok(InputNotifyPeer::Users {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ChatFull {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ChatFull> {
        match id {
            0x76af5481_u32 => {
                let flags = buf.deserialize()?;

                let can_view_participants = flags & 1 << 3 != 0;

                let can_set_username = flags & 1 << 6 != 0;

                let can_set_stickers = flags & 1 << 7 != 0;

                let hidden_prehistory = flags & 1 << 10 != 0;
                let id = buf.deserialize()?;
                let about = buf.deserialize()?;

                let participants_count = if flags & 1 << 0 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let admins_count = if flags & 1 << 1 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let kicked_count = if flags & 1 << 2 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let banned_count = if flags & 1 << 2 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
                let read_inbox_max_id = buf.deserialize()?;
                let read_outbox_max_id = buf.deserialize()?;
                let unread_count = buf.deserialize()?;
                let chat_photo = Box::new(buf.deserialize()?);
                let notify_settings = Box::new(buf.deserialize()?);
                let exported_invite = Box::new(buf.deserialize()?);
                let bot_info = buf.deserialize()?;

                let migrated_from_chat_id = if flags & 1 << 4 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let migrated_from_max_id = if flags & 1 << 4 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let pinned_msg_id = if flags & 1 << 5 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };

                let stickerset = if flags & 1 << 8 != 0 {
                    Some(Box::new(buf.deserialize()?))
                } else {
                    None
                };

                let available_min_id = if flags & 1 << 9 != 0 {
                    Some(buf.deserialize()?)} else {
                    None
                };
            
                Ok(ChatFull::ChannelFull {
                    flags,
                    can_view_participants,
                    can_set_username,
                    can_set_stickers,
                    hidden_prehistory,
                    id,
                    about,
                    participants_count,
                    admins_count,
                    kicked_count,
                    banned_count,
                    read_inbox_max_id,
                    read_outbox_max_id,
                    unread_count,
                    chat_photo,
                    notify_settings,
                    exported_invite,
                    bot_info,
                    migrated_from_chat_id,
                    migrated_from_max_id,
                    pinned_msg_id,
                    stickerset,
                    available_min_id,
                })
            },

            0x2e02a614_u32 => {
                let id = buf.deserialize()?;
                let participants = Box::new(buf.deserialize()?);
                let chat_photo = Box::new(buf.deserialize()?);
                let notify_settings = Box::new(buf.deserialize()?);
                let exported_invite = Box::new(buf.deserialize()?);
                let bot_info = buf.deserialize()?;
            
                Ok(ChatFull::Full {
                    id,
                    participants,
                    chat_photo,
                    notify_settings,
                    exported_invite,
                    bot_info,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for TopPeer {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xedcdc05b);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<TopPeer> {
                let peer = Box::new(buf.deserialize()?);
        let rating = buf.deserialize()?;

        Ok(TopPeer {
            peer,
            rating,
        })
    }
}

impl Deserializable for WallPaper {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<WallPaper> {
        match id {
            0xccb03657_u32 => {
                let id = buf.deserialize()?;
                let title = buf.deserialize()?;
                let sizes = buf.deserialize()?;
                let color = buf.deserialize()?;
            
                Ok(WallPaper::Paper {
                    id,
                    title,
                    sizes,
                    color,
                })
            },

            0x63117f24_u32 => {
                let id = buf.deserialize()?;
                let title = buf.deserialize()?;
                let bg_color = buf.deserialize()?;
                let color = buf.deserialize()?;
            
                Ok(WallPaper::Solid {
                    id,
                    title,
                    bg_color,
                    color,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for ReceivedNotifyMessage {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xa384b779);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ReceivedNotifyMessage> {
                let id = buf.deserialize()?;
        let flags = buf.deserialize()?;

        Ok(ReceivedNotifyMessage {
            id,
            flags,
        })
    }
}

impl Deserializable for ExportedChatInvite {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<ExportedChatInvite> {
        match id {
            0xfc2e05bc_u32 => {
                let link = buf.deserialize()?;
            
                Ok(ExportedChatInvite::Exported {
                    link,
                })
            },

            0x69df3769_u32 => {
            
                Ok(ExportedChatInvite::Empty {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for MessagesRecentStickers {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesRecentStickers> {
        match id {
            0x0b17f890_u32 => {
            
                Ok(MessagesRecentStickers::NotModified {
                })
            },

            0x5ce20970_u32 => {
                let hash = buf.deserialize()?;
                let stickers = buf.deserialize()?;
            
                Ok(MessagesRecentStickers::Stickers {
                    hash,
                    stickers,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputPhoto {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputPhoto> {
        match id {
            0xfb95c6c4_u32 => {
                let id = buf.deserialize()?;
                let access_hash = buf.deserialize()?;
            
                Ok(InputPhoto::Photo {
                    id,
                    access_hash,
                })
            },

            0x1cd7bf0d_u32 => {
            
                Ok(InputPhoto::Empty {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for PeerSettings {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x818426cd);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PeerSettings> {
                let flags = buf.deserialize()?;

        let report_spam = flags & 1 << 0 != 0;

        Ok(PeerSettings {
            flags,
            report_spam,
        })
    }
}

impl Deserializable for PhoneCallDiscardReason {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PhoneCallDiscardReason> {
        match id {
            0xfaf7e8c9_u32 => {
            
                Ok(PhoneCallDiscardReason::Busy {
                })
            },

            0xe095c1a0_u32 => {
            
                Ok(PhoneCallDiscardReason::Disconnect {
                })
            },

            0x85e42301_u32 => {
            
                Ok(PhoneCallDiscardReason::Missed {
                })
            },

            0x57adc690_u32 => {
            
                Ok(PhoneCallDiscardReason::Hangup {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for PaymentSavedCredentials {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PaymentSavedCredentials> {
        match id {
            0xcdc27a1f_u32 => {
                let id = buf.deserialize()?;
                let title = buf.deserialize()?;
            
                Ok(PaymentSavedCredentials::Card {
                    id,
                    title,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for CdnFileHash {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x77eec38f);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<CdnFileHash> {
                let offset = buf.deserialize()?;
        let limit = buf.deserialize()?;
        let hash = buf.deserialize()?;

        Ok(CdnFileHash {
            offset,
            limit,
            hash,
        })
    }
}

impl Deserializable for PaymentRequestedInfo {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x909c3f94);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PaymentRequestedInfo> {
                let flags = buf.deserialize()?;

        let name = if flags & 1 << 0 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        let phone = if flags & 1 << 1 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        let email = if flags & 1 << 2 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        let shipping_address = if flags & 1 << 3 != 0 {
            Some(Box::new(buf.deserialize()?))
        } else {
            None
        };

        Ok(PaymentRequestedInfo {
            flags,
            name,
            phone,
            email,
            shipping_address,
        })
    }
}

impl Deserializable for TopPeerCategory {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<TopPeerCategory> {
        match id {
            0x0637b7ed_u32 => {
            
                Ok(TopPeerCategory::Correspondents {
                })
            },

            0x148677e2_u32 => {
            
                Ok(TopPeerCategory::BotsInline {
                })
            },

            0xab661b5b_u32 => {
            
                Ok(TopPeerCategory::BotsPm {
                })
            },

            0x1e76a78c_u32 => {
            
                Ok(TopPeerCategory::PhoneCalls {
                })
            },

            0x161d9628_u32 => {
            
                Ok(TopPeerCategory::Channels {
                })
            },

            0xbd17a14a_u32 => {
            
                Ok(TopPeerCategory::Groups {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for KeyboardButton {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<KeyboardButton> {
        match id {
            0x50f41ccf_u32 => {
                let text = buf.deserialize()?;
            
                Ok(KeyboardButton::Game {
                    text,
                })
            },

            0xb16a6c29_u32 => {
                let text = buf.deserialize()?;
            
                Ok(KeyboardButton::RequestPhone {
                    text,
                })
            },

            0xfc796b3f_u32 => {
                let text = buf.deserialize()?;
            
                Ok(KeyboardButton::RequestGeoLocation {
                    text,
                })
            },

            0x258aff05_u32 => {
                let text = buf.deserialize()?;
                let url = buf.deserialize()?;
            
                Ok(KeyboardButton::Url {
                    text,
                    url,
                })
            },

            0xafd93fbb_u32 => {
                let text = buf.deserialize()?;
            
                Ok(KeyboardButton::Buy {
                    text,
                })
            },

            0xa2fa4880_u32 => {
                let text = buf.deserialize()?;
            
                Ok(KeyboardButton::Button {
                    text,
                })
            },

            0x683a5e46_u32 => {
                let text = buf.deserialize()?;
                let data = buf.deserialize()?;
            
                Ok(KeyboardButton::Callback {
                    text,
                    data,
                })
            },

            0x0568a748_u32 => {
                let flags = buf.deserialize()?;

                let same_peer = flags & 1 << 0 != 0;
                let text = buf.deserialize()?;
                let query = buf.deserialize()?;
            
                Ok(KeyboardButton::SwitchInline {
                    flags,
                    same_peer,
                    text,
                    query,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputPhoneCall {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x1e36fded);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputPhoneCall> {
                let id = buf.deserialize()?;
        let access_hash = buf.deserialize()?;

        Ok(InputPhoneCall {
            id,
            access_hash,
        })
    }
}

impl Deserializable for PaymentsSavedInfo {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xfb8fe43c);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PaymentsSavedInfo> {
                let flags = buf.deserialize()?;

        let has_saved_credentials = flags & 1 << 1 != 0;

        let saved_info = if flags & 1 << 0 != 0 {
            Some(Box::new(buf.deserialize()?))
        } else {
            None
        };

        Ok(PaymentsSavedInfo {
            flags,
            has_saved_credentials,
            saved_info,
        })
    }
}

impl Deserializable for MessagesChats {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<MessagesChats> {
        match id {
            0x9cd81144_u32 => {
                let count = buf.deserialize()?;
                let chats = buf.deserialize()?;
            
                Ok(MessagesChats::Slice {
                    count,
                    chats,
                })
            },

            0x64ff9fd5_u32 => {
                let chats = buf.deserialize()?;
            
                Ok(MessagesChats::Chats {
                    chats,
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

impl Deserializable for InputSingleMedia {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x5eaa7809);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<InputSingleMedia> {
                let media = Box::new(buf.deserialize()?);
        let random_id = buf.deserialize()?;

        Ok(InputSingleMedia {
            media,
            random_id,
        })
    }
}

impl Deserializable for PaymentsPaymentForm {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0x3f56aea3);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PaymentsPaymentForm> {
                let flags = buf.deserialize()?;

        let can_save_credentials = flags & 1 << 2 != 0;

        let password_missing = flags & 1 << 3 != 0;
        let bot_id = buf.deserialize()?;
        let invoice = Box::new(buf.deserialize()?);
        let provider_id = buf.deserialize()?;
        let url = buf.deserialize()?;

        let native_provider = if flags & 1 << 4 != 0 {
            Some(buf.deserialize()?)} else {
            None
        };

        let native_params = if flags & 1 << 4 != 0 {
            Some(Box::new(buf.deserialize()?))
        } else {
            None
        };

        let saved_info = if flags & 1 << 0 != 0 {
            Some(Box::new(buf.deserialize()?))
        } else {
            None
        };

        let saved_credentials = if flags & 1 << 1 != 0 {
            Some(Box::new(buf.deserialize()?))
        } else {
            None
        };
        let users = buf.deserialize()?;

        Ok(PaymentsPaymentForm {
            flags,
            can_save_credentials,
            password_missing,
            bot_id,
            invoice,
            provider_id,
            url,
            native_provider,
            native_params,
            saved_info,
            saved_credentials,
            users,
        })
    }
}

impl Deserializable for CdnPublicKey {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xc982eaba);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<CdnPublicKey> {
                let dc_id = buf.deserialize()?;
        let public_key = buf.deserialize()?;

        Ok(CdnPublicKey {
            dc_id,
            public_key,
        })
    }
}

impl Deserializable for Game {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        assert_eq!(id, 0xbdf9653b);

        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<Game> {
                let flags = buf.deserialize()?;
        let id = buf.deserialize()?;
        let access_hash = buf.deserialize()?;
        let short_name = buf.deserialize()?;
        let title = buf.deserialize()?;
        let description = buf.deserialize()?;
        let photo = Box::new(buf.deserialize()?);

        let document = if flags & 1 << 0 != 0 {
            Some(Box::new(buf.deserialize()?))
        } else {
            None
        };

        Ok(Game {
            flags,
            id,
            access_hash,
            short_name,
            title,
            description,
            photo,
            document,
        })
    }
}

impl Deserializable for PrivacyRule {
    #[inline]
    fn deserialize_from<B: Read>(buf: &mut B) -> Result<Self> {
        let id = u32::deserialize_from(buf)?;
        Self::deserialize_bare(buf, id)
    }

    #[allow(unused_variables, unused_mut)]
    fn deserialize_bare<B: Read>(buf: &mut B, id: u32) -> Result<PrivacyRule> {
        match id {
            0xf888fa1a_u32 => {
            
                Ok(PrivacyRule::ValueDisallowContacts {
                })
            },

            0xfffe1bac_u32 => {
            
                Ok(PrivacyRule::ValueAllowContacts {
                })
            },

            0x0c7f49b7_u32 => {
                let users = buf.deserialize()?;
            
                Ok(PrivacyRule::ValueDisallowUsers {
                    users,
                })
            },

            0x4d5bbe0c_u32 => {
                let users = buf.deserialize()?;
            
                Ok(PrivacyRule::ValueAllowUsers {
                    users,
                })
            },

            0x65427b82_u32 => {
            
                Ok(PrivacyRule::ValueAllowAll {
                })
            },

            0x8b73e763_u32 => {
            
                Ok(PrivacyRule::ValueDisallowAll {
                })
            },

            _ => {
                panic!("ID not recognized lmao");
            },
        }
    }
}

