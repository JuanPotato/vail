impl Serializable for ReqPq {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x60469778_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.nonce)?;

        Ok(())
    }
 }

impl Serializable for ReqDhParams {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xd712e4be_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.nonce)?;
        buf.serialize(&self.server_nonce)?;
        buf.serialize(&self.p)?;
        buf.serialize(&self.q)?;
        buf.serialize(&self.public_key_fingerprint)?;
        buf.serialize(&self.encrypted_data)?;

        Ok(())
    }
 }

impl Serializable for SetClientDhParams {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf5045f1f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.nonce)?;
        buf.serialize(&self.server_nonce)?;
        buf.serialize(&self.encrypted_data)?;

        Ok(())
    }
 }

impl Serializable for DestroyAuthKey {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xd1435160_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for RpcDropAnswer {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x58e4a740_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.req_msg_id)?;

        Ok(())
    }
 }

impl Serializable for GetFutureSalts {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xb921bd04_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.num)?;

        Ok(())
    }
 }

impl Serializable for Ping {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x7abe77ec_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.ping_id)?;

        Ok(())
    }
 }

impl Serializable for PingDelayDisconnect {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf3427b8c_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.ping_id)?;
        buf.serialize(&self.disconnect_delay)?;

        Ok(())
    }
 }

impl Serializable for DestroySession {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe7512126_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.session_id)?;

        Ok(())
    }
 }

impl Serializable for ContestSaveDeveloperInfo {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x9a5f6e95_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.vk_id)?;
        buf.serialize(&self.name)?;
        buf.serialize(&self.phone_number)?;
        buf.serialize(&self.age)?;
        buf.serialize(&self.city)?;

        Ok(())
    }
 }

impl Serializable for AuthCheckPhone {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x6fe51dfb_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.phone_number)?;

        Ok(())
    }
 }

impl Serializable for AuthSendCode {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x86aef0ec_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.allow_flashcall {
            ser_flags |= 1 << 0;
        }

        if self.current_number.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.phone_number)?;

        if let Some(ref value) = self.current_number {
            buf.serialize(value)?;
        }

        buf.serialize(&self.api_id)?;
        buf.serialize(&self.api_hash)?;

        Ok(())
    }
 }

impl Serializable for AuthSignUp {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x1b067634_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.phone_number)?;
        buf.serialize(&self.phone_code_hash)?;
        buf.serialize(&self.phone_code)?;
        buf.serialize(&self.first_name)?;
        buf.serialize(&self.last_name)?;

        Ok(())
    }
 }

impl Serializable for AuthSignIn {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xbcd51581_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.phone_number)?;
        buf.serialize(&self.phone_code_hash)?;
        buf.serialize(&self.phone_code)?;

        Ok(())
    }
 }

impl Serializable for AuthLogOut {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x5717da40_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for AuthResetAuthorizations {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x9fab0d1a_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for AuthSendInvites {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x771c1d97_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.phone_numbers)?;
        buf.serialize(&self.message)?;

        Ok(())
    }
 }

impl Serializable for AuthExportAuthorization {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe5bfffcd_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.dc_id)?;

        Ok(())
    }
 }

impl Serializable for AuthImportAuthorization {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe3ef9613_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;
        buf.serialize(&self.bytes)?;

        Ok(())
    }
 }

impl Serializable for AuthBindTempAuthKey {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xcdd42a05_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.perm_auth_key_id)?;
        buf.serialize(&self.nonce)?;
        buf.serialize(&self.expires_at)?;
        buf.serialize(&self.encrypted_message)?;

        Ok(())
    }
 }

impl Serializable for AuthImportBotAuthorization {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x67a3ff2c_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.api_id)?;
        buf.serialize(&self.api_hash)?;
        buf.serialize(&self.bot_auth_token)?;

        Ok(())
    }
 }

impl Serializable for AuthCheckPassword {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x0a63011e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.password_hash)?;

        Ok(())
    }
 }

impl Serializable for AuthRequestPasswordRecovery {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xd897bc66_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for AuthRecoverPassword {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x4ea56e92_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.code)?;

        Ok(())
    }
 }

impl Serializable for AuthResendCode {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3ef1a9bf_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.phone_number)?;
        buf.serialize(&self.phone_code_hash)?;

        Ok(())
    }
 }

impl Serializable for AuthCancelCode {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x1f040578_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.phone_number)?;
        buf.serialize(&self.phone_code_hash)?;

        Ok(())
    }
 }

impl Serializable for AuthDropTempAuthKeys {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x8e48a188_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.except_auth_keys)?;

        Ok(())
    }
 }

impl Serializable for AccountRegisterDevice {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x001389cc_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.token_type)?;
        buf.serialize(&self.token)?;
        buf.serialize(&self.app_sandbox)?;
        buf.serialize(&self.other_uids)?;

        Ok(())
    }
 }

impl Serializable for AccountUnregisterDevice {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3076c4bf_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.token_type)?;
        buf.serialize(&self.token)?;
        buf.serialize(&self.other_uids)?;

        Ok(())
    }
 }

impl Serializable for AccountUpdateNotifySettings {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x84be5b93_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(self.settings.as_ref())?;

        Ok(())
    }
 }

impl Serializable for AccountGetNotifySettings {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x12b3ad31_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;

        Ok(())
    }
 }

impl Serializable for AccountResetNotifySettings {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xdb7e1747_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for AccountUpdateProfile {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x78515775_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.first_name.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.last_name.is_some() {
            ser_flags |= 1 << 1;
        }

        if self.about.is_some() {
            ser_flags |= 1 << 2;
        }

        buf.serialize(&ser_flags)?;

        if let Some(ref value) = self.first_name {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.last_name {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.about {
            buf.serialize(value)?;
        }


        Ok(())
    }
 }

impl Serializable for AccountUpdateStatus {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x6628562c_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.offline)?;

        Ok(())
    }
 }

impl Serializable for AccountGetWallPapers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc04cfac2_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for AccountReportPeer {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xae189d5f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(self.reason.as_ref())?;

        Ok(())
    }
 }

impl Serializable for AccountCheckUsername {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x2714d86c_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.username)?;

        Ok(())
    }
 }

impl Serializable for AccountUpdateUsername {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3e0bdd7c_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.username)?;

        Ok(())
    }
 }

impl Serializable for AccountGetPrivacy {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xdadbc950_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.key.as_ref())?;

        Ok(())
    }
 }

impl Serializable for AccountSetPrivacy {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc9f81ce8_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.key.as_ref())?;
        buf.serialize(&self.rules)?;

        Ok(())
    }
 }

impl Serializable for AccountDeleteAccount {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x418d4e0b_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.reason)?;

        Ok(())
    }
 }

impl Serializable for AccountGetAccountTtl {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x08fc711d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for AccountSetAccountTtl {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x2442485e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.ttl.as_ref())?;

        Ok(())
    }
 }

impl Serializable for AccountSendChangePhoneCode {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x08e57deb_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.allow_flashcall {
            ser_flags |= 1 << 0;
        }

        if self.current_number.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.phone_number)?;

        if let Some(ref value) = self.current_number {
            buf.serialize(value)?;
        }


        Ok(())
    }
 }

impl Serializable for AccountChangePhone {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x70c32edb_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.phone_number)?;
        buf.serialize(&self.phone_code_hash)?;
        buf.serialize(&self.phone_code)?;

        Ok(())
    }
 }

impl Serializable for AccountUpdateDeviceLocked {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x38df3532_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.period)?;

        Ok(())
    }
 }

impl Serializable for AccountGetAuthorizations {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe320c158_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for AccountResetAuthorization {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xdf77f3bc_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for AccountGetPassword {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x548a30f5_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for AccountGetPasswordSettings {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xbc8d11bb_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.current_password_hash)?;

        Ok(())
    }
 }

impl Serializable for AccountUpdatePasswordSettings {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xfa7c4b86_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.current_password_hash)?;
        buf.serialize(self.new_settings.as_ref())?;

        Ok(())
    }
 }

impl Serializable for AccountSendConfirmPhoneCode {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x1516d7bd_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.allow_flashcall {
            ser_flags |= 1 << 0;
        }

        if self.current_number.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.hash)?;

        if let Some(ref value) = self.current_number {
            buf.serialize(value)?;
        }


        Ok(())
    }
 }

impl Serializable for AccountConfirmPhone {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x5f2178c3_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.phone_code_hash)?;
        buf.serialize(&self.phone_code)?;

        Ok(())
    }
 }

impl Serializable for AccountGetTmpPassword {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x4a82327e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.password_hash)?;
        buf.serialize(&self.period)?;

        Ok(())
    }
 }

impl Serializable for UsersGetUsers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x0d91a548_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for UsersGetFullUser {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xca30a5b1_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.id.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ContactsGetStatuses {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc4a353ee_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for ContactsGetContacts {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc023849f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for ContactsImportContacts {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x2c800be5_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.contacts)?;

        Ok(())
    }
 }

impl Serializable for ContactsDeleteContact {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x8e953744_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.id.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ContactsDeleteContacts {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x59ab389e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for ContactsBlock {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x332b49fc_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.id.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ContactsUnblock {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe54100bd_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.id.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ContactsGetBlocked {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf57c350f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.offset)?;
        buf.serialize(&self.limit)?;

        Ok(())
    }
 }

impl Serializable for ContactsExportCard {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x84e53737_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for ContactsImportCard {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x4fe196fe_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.export_card)?;

        Ok(())
    }
 }

impl Serializable for ContactsSearch {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x11f812d8_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.q)?;
        buf.serialize(&self.limit)?;

        Ok(())
    }
 }

impl Serializable for ContactsResolveUsername {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf93ccba3_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.username)?;

        Ok(())
    }
 }

impl Serializable for ContactsGetTopPeers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xd4982db5_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.correspondents {
            ser_flags |= 1 << 0;
        }

        if self.bots_pm {
            ser_flags |= 1 << 1;
        }

        if self.bots_inline {
            ser_flags |= 1 << 2;
        }

        if self.phone_calls {
            ser_flags |= 1 << 3;
        }

        if self.groups {
            ser_flags |= 1 << 10;
        }

        if self.channels {
            ser_flags |= 1 << 15;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.offset)?;
        buf.serialize(&self.limit)?;
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for ContactsResetTopPeerRating {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x1ae373ac_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.category.as_ref())?;
        buf.serialize(self.peer.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ContactsResetSaved {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x879537f1_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for MessagesGetMessages {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x4222fa74_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetDialogs {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x05c0fae2_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.exclude_pinned {
            ser_flags |= 1 << 0;
        }

        if self.feed_id.is_some() {
            ser_flags |= 1 << 1;
        }

        buf.serialize(&ser_flags)?;

        if let Some(ref value) = self.feed_id {
            buf.serialize(value)?;
        }

        buf.serialize(&self.offset_date)?;
        buf.serialize(&self.offset_id)?;
        buf.serialize(self.offset_peer.as_ref())?;
        buf.serialize(&self.limit)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetHistory {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xdcbb8260_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.offset_id)?;
        buf.serialize(&self.offset_date)?;
        buf.serialize(&self.add_offset)?;
        buf.serialize(&self.limit)?;
        buf.serialize(&self.max_id)?;
        buf.serialize(&self.min_id)?;
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for MessagesSearch {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x039e9ea0_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.from_id.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.q)?;

        if let Some(ref value) = self.from_id {
            buf.serialize(value.as_ref())?;
        }

        buf.serialize(self.filter.as_ref())?;
        buf.serialize(&self.min_date)?;
        buf.serialize(&self.max_date)?;
        buf.serialize(&self.offset_id)?;
        buf.serialize(&self.add_offset)?;
        buf.serialize(&self.limit)?;
        buf.serialize(&self.max_id)?;
        buf.serialize(&self.min_id)?;

        Ok(())
    }
 }

impl Serializable for MessagesReadHistory {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x0e306d3a_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.max_id)?;

        Ok(())
    }
 }

impl Serializable for MessagesDeleteHistory {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x1c015b09_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.just_clear {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.max_id)?;

        Ok(())
    }
 }

impl Serializable for MessagesDeleteMessages {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe58e95d2_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.revoke {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for MessagesReceivedMessages {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x05a954c0_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.max_id)?;

        Ok(())
    }
 }

impl Serializable for MessagesSetTyping {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xa3825e50_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(self.action.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesSendMessage {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xfa88427a_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.no_webpage {
            ser_flags |= 1 << 1;
        }

        if self.silent {
            ser_flags |= 1 << 5;
        }

        if self.background {
            ser_flags |= 1 << 6;
        }

        if self.clear_draft {
            ser_flags |= 1 << 7;
        }

        if self.reply_to_msg_id.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.reply_markup.is_some() {
            ser_flags |= 1 << 2;
        }

        if self.entities.is_some() {
            ser_flags |= 1 << 3;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.peer.as_ref())?;

        if let Some(ref value) = self.reply_to_msg_id {
            buf.serialize(value)?;
        }

        buf.serialize(&self.message)?;
        buf.serialize(&self.random_id)?;

        if let Some(ref value) = self.reply_markup {
            buf.serialize(value.as_ref())?;
        }


        if let Some(ref value) = self.entities {
            buf.serialize(value)?;
        }


        Ok(())
    }
 }

impl Serializable for MessagesSendMedia {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc8f16791_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.silent {
            ser_flags |= 1 << 5;
        }

        if self.background {
            ser_flags |= 1 << 6;
        }

        if self.clear_draft {
            ser_flags |= 1 << 7;
        }

        if self.reply_to_msg_id.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.reply_markup.is_some() {
            ser_flags |= 1 << 2;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.peer.as_ref())?;

        if let Some(ref value) = self.reply_to_msg_id {
            buf.serialize(value)?;
        }

        buf.serialize(self.media.as_ref())?;
        buf.serialize(&self.random_id)?;

        if let Some(ref value) = self.reply_markup {
            buf.serialize(value.as_ref())?;
        }


        Ok(())
    }
 }

impl Serializable for MessagesForwardMessages {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x708e0195_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.silent {
            ser_flags |= 1 << 5;
        }

        if self.background {
            ser_flags |= 1 << 6;
        }

        if self.with_my_score {
            ser_flags |= 1 << 8;
        }

        if self.grouped {
            ser_flags |= 1 << 9;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.from_peer.as_ref())?;
        buf.serialize(&self.id)?;
        buf.serialize(&self.random_id)?;
        buf.serialize(self.to_peer.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesReportSpam {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xcf1592db_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesHideReportSpam {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xa8f1709b_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesGetPeerSettings {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3672e09c_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesGetChats {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3c6aa187_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetFullChat {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3b831c66_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.chat_id)?;

        Ok(())
    }
 }

impl Serializable for MessagesEditChatTitle {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xdc452855_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.chat_id)?;
        buf.serialize(&self.title)?;

        Ok(())
    }
 }

impl Serializable for MessagesEditChatPhoto {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xca4c79d8_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.chat_id)?;
        buf.serialize(self.photo.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesAddChatUser {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf9a0aa09_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.chat_id)?;
        buf.serialize(self.user_id.as_ref())?;
        buf.serialize(&self.fwd_limit)?;

        Ok(())
    }
 }

impl Serializable for MessagesDeleteChatUser {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe0611f16_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.chat_id)?;
        buf.serialize(self.user_id.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesCreateChat {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x09cb126e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.users)?;
        buf.serialize(&self.title)?;

        Ok(())
    }
 }

impl Serializable for MessagesForwardMessage {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x33963bf9_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.id)?;
        buf.serialize(&self.random_id)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetDhConfig {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x26cf8950_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.version)?;
        buf.serialize(&self.random_length)?;

        Ok(())
    }
 }

impl Serializable for MessagesRequestEncryption {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf64daf43_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.user_id.as_ref())?;
        buf.serialize(&self.random_id)?;
        buf.serialize(&self.g_a)?;

        Ok(())
    }
 }

impl Serializable for MessagesAcceptEncryption {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3dbc0415_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.g_b)?;
        buf.serialize(&self.key_fingerprint)?;

        Ok(())
    }
 }

impl Serializable for MessagesDiscardEncryption {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xedd923c5_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.chat_id)?;

        Ok(())
    }
 }

impl Serializable for MessagesSetEncryptedTyping {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x791451ed_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.typing)?;

        Ok(())
    }
 }

impl Serializable for MessagesReadEncryptedHistory {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x7f4b690a_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.max_date)?;

        Ok(())
    }
 }

impl Serializable for MessagesSendEncrypted {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xa9776773_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.random_id)?;
        buf.serialize(&self.data)?;

        Ok(())
    }
 }

impl Serializable for MessagesSendEncryptedFile {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x9a901b66_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.random_id)?;
        buf.serialize(&self.data)?;
        buf.serialize(self.file.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesSendEncryptedService {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x32d439a4_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.random_id)?;
        buf.serialize(&self.data)?;

        Ok(())
    }
 }

impl Serializable for MessagesReceivedQueue {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x55a5bb66_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.max_qts)?;

        Ok(())
    }
 }

impl Serializable for MessagesReportEncryptedSpam {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x4b0c8c0f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesReadMessageContents {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x36a73f77_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetAllStickers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x1c9618b1_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetWebPagePreview {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x25223e24_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.message)?;

        Ok(())
    }
 }

impl Serializable for MessagesExportChatInvite {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x7d885289_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.chat_id)?;

        Ok(())
    }
 }

impl Serializable for MessagesCheckChatInvite {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3eadb1bb_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for MessagesImportChatInvite {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x6c50051c_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetStickerSet {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x2619a90e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.stickerset.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesInstallStickerSet {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc78fe460_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.stickerset.as_ref())?;
        buf.serialize(&self.archived)?;

        Ok(())
    }
 }

impl Serializable for MessagesUninstallStickerSet {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf96e55de_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.stickerset.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesStartBot {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe6df7378_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.bot.as_ref())?;
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.random_id)?;
        buf.serialize(&self.start_param)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetMessagesViews {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc4c8a55d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.id)?;
        buf.serialize(&self.increment)?;

        Ok(())
    }
 }

impl Serializable for MessagesToggleChatAdmins {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xec8bd9e1_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.chat_id)?;
        buf.serialize(&self.enabled)?;

        Ok(())
    }
 }

impl Serializable for MessagesEditChatAdmin {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xa9e69f2e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.chat_id)?;
        buf.serialize(self.user_id.as_ref())?;
        buf.serialize(&self.is_admin)?;

        Ok(())
    }
 }

impl Serializable for MessagesMigrateChat {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x15a3b8e3_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.chat_id)?;

        Ok(())
    }
 }

impl Serializable for MessagesSearchGlobal {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x9e3cacb0_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.q)?;
        buf.serialize(&self.offset_date)?;
        buf.serialize(self.offset_peer.as_ref())?;
        buf.serialize(&self.offset_id)?;
        buf.serialize(&self.limit)?;

        Ok(())
    }
 }

impl Serializable for MessagesReorderStickerSets {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x78337739_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.masks {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.order)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetDocumentByHash {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x338e2464_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.sha256)?;
        buf.serialize(&self.size)?;
        buf.serialize(&self.mime_type)?;

        Ok(())
    }
 }

impl Serializable for MessagesSearchGifs {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xbf9a776b_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.q)?;
        buf.serialize(&self.offset)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetSavedGifs {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x83bf3d52_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for MessagesSaveGif {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x327a30cb_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.id.as_ref())?;
        buf.serialize(&self.unsave)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetInlineBotResults {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x514e999d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.geo_point.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.bot.as_ref())?;
        buf.serialize(self.peer.as_ref())?;

        if let Some(ref value) = self.geo_point {
            buf.serialize(value.as_ref())?;
        }

        buf.serialize(&self.query)?;
        buf.serialize(&self.offset)?;

        Ok(())
    }
 }

impl Serializable for MessagesSetInlineBotResults {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xeb5ea206_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.gallery {
            ser_flags |= 1 << 0;
        }

        if self.private {
            ser_flags |= 1 << 1;
        }

        if self.next_offset.is_some() {
            ser_flags |= 1 << 2;
        }

        if self.switch_pm.is_some() {
            ser_flags |= 1 << 3;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.query_id)?;
        buf.serialize(&self.results)?;
        buf.serialize(&self.cache_time)?;

        if let Some(ref value) = self.next_offset {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.switch_pm {
            buf.serialize(value.as_ref())?;
        }


        Ok(())
    }
 }

impl Serializable for MessagesSendInlineBotResult {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xb16e06fe_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.silent {
            ser_flags |= 1 << 5;
        }

        if self.background {
            ser_flags |= 1 << 6;
        }

        if self.clear_draft {
            ser_flags |= 1 << 7;
        }

        if self.reply_to_msg_id.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.peer.as_ref())?;

        if let Some(ref value) = self.reply_to_msg_id {
            buf.serialize(value)?;
        }

        buf.serialize(&self.random_id)?;
        buf.serialize(&self.query_id)?;
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetMessageEditData {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xfda68d36_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for MessagesEditMessage {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x05d1b8dd_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.no_webpage {
            ser_flags |= 1 << 1;
        }

        if self.stop_geo_live {
            ser_flags |= 1 << 12;
        }

        if self.message.is_some() {
            ser_flags |= 1 << 11;
        }

        if self.reply_markup.is_some() {
            ser_flags |= 1 << 2;
        }

        if self.entities.is_some() {
            ser_flags |= 1 << 3;
        }

        if self.geo_point.is_some() {
            ser_flags |= 1 << 13;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.id)?;

        if let Some(ref value) = self.message {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.reply_markup {
            buf.serialize(value.as_ref())?;
        }


        if let Some(ref value) = self.entities {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.geo_point {
            buf.serialize(value.as_ref())?;
        }


        Ok(())
    }
 }

impl Serializable for MessagesEditInlineBotMessage {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xb0e08243_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.no_webpage {
            ser_flags |= 1 << 1;
        }

        if self.stop_geo_live {
            ser_flags |= 1 << 12;
        }

        if self.message.is_some() {
            ser_flags |= 1 << 11;
        }

        if self.reply_markup.is_some() {
            ser_flags |= 1 << 2;
        }

        if self.entities.is_some() {
            ser_flags |= 1 << 3;
        }

        if self.geo_point.is_some() {
            ser_flags |= 1 << 13;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.id.as_ref())?;

        if let Some(ref value) = self.message {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.reply_markup {
            buf.serialize(value.as_ref())?;
        }


        if let Some(ref value) = self.entities {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.geo_point {
            buf.serialize(value.as_ref())?;
        }


        Ok(())
    }
 }

impl Serializable for MessagesGetBotCallbackAnswer {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x810a9fec_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.game {
            ser_flags |= 1 << 1;
        }

        if self.data.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.msg_id)?;

        if let Some(ref value) = self.data {
            buf.serialize(value)?;
        }


        Ok(())
    }
 }

impl Serializable for MessagesSetBotCallbackAnswer {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xd58f130a_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.alert {
            ser_flags |= 1 << 1;
        }

        if self.message.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.url.is_some() {
            ser_flags |= 1 << 2;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.query_id)?;

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

impl Serializable for MessagesGetPeerDialogs {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe470bcfd_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.peers)?;

        Ok(())
    }
 }

impl Serializable for MessagesSaveDraft {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xbc39e14b_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.no_webpage {
            ser_flags |= 1 << 1;
        }

        if self.reply_to_msg_id.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.entities.is_some() {
            ser_flags |= 1 << 3;
        }

        buf.serialize(&ser_flags)?;

        if let Some(ref value) = self.reply_to_msg_id {
            buf.serialize(value)?;
        }

        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.message)?;

        if let Some(ref value) = self.entities {
            buf.serialize(value)?;
        }


        Ok(())
    }
 }

impl Serializable for MessagesGetAllDrafts {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x6a3f8d65_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for MessagesGetFeaturedStickers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x2dacca4f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for MessagesReadFeaturedStickers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x5b118126_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetRecentStickers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x5ea192c9_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.attached {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for MessagesSaveRecentSticker {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x392718f8_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.attached {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.id.as_ref())?;
        buf.serialize(&self.unsave)?;

        Ok(())
    }
 }

impl Serializable for MessagesClearRecentStickers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x8999602d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.attached {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetArchivedStickers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x57f17692_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.masks {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.offset_id)?;
        buf.serialize(&self.limit)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetMaskStickers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x65b8c79f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetAttachedStickers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xcc5b67cc_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.media.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesSetGameScore {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x8ef8ecc0_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.edit_message {
            ser_flags |= 1 << 0;
        }

        if self.force {
            ser_flags |= 1 << 1;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.id)?;
        buf.serialize(self.user_id.as_ref())?;
        buf.serialize(&self.score)?;

        Ok(())
    }
 }

impl Serializable for MessagesSetInlineGameScore {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x15ad9f64_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.edit_message {
            ser_flags |= 1 << 0;
        }

        if self.force {
            ser_flags |= 1 << 1;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.id.as_ref())?;
        buf.serialize(self.user_id.as_ref())?;
        buf.serialize(&self.score)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetGameHighScores {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe822649d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.id)?;
        buf.serialize(self.user_id.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesGetInlineGameHighScores {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x0f635e1b_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.id.as_ref())?;
        buf.serialize(self.user_id.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesGetCommonChats {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x0d0a48c4_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.user_id.as_ref())?;
        buf.serialize(&self.max_id)?;
        buf.serialize(&self.limit)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetAllChats {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xeba80ff0_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.except_ids)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetWebPage {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x32ca8f91_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.url)?;
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for MessagesToggleDialogPin {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xa731e257_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.pinned {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.peer.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesReorderPinnedDialogs {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x5b51d63f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.force {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.order)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetPinnedDialogs {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe254d64e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for MessagesSetBotShippingResults {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe5f672fa_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.error.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.shipping_options.is_some() {
            ser_flags |= 1 << 1;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.query_id)?;

        if let Some(ref value) = self.error {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.shipping_options {
            buf.serialize(value)?;
        }


        Ok(())
    }
 }

impl Serializable for MessagesSetBotPrecheckoutResults {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x09c2dd95_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.success {
            ser_flags |= 1 << 1;
        }

        if self.error.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.query_id)?;

        if let Some(ref value) = self.error {
            buf.serialize(value)?;
        }


        Ok(())
    }
 }

impl Serializable for MessagesUploadMedia {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x519bc2b1_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(self.media.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesSendScreenshotNotification {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc97df020_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.reply_to_msg_id)?;
        buf.serialize(&self.random_id)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetFavedStickers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x21ce0b0e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for MessagesFaveSticker {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xb9ffc55b_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.id.as_ref())?;
        buf.serialize(&self.unfave)?;

        Ok(())
    }
 }

impl Serializable for MessagesGetUnreadMentions {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x46578472_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.offset_id)?;
        buf.serialize(&self.add_offset)?;
        buf.serialize(&self.limit)?;
        buf.serialize(&self.max_id)?;
        buf.serialize(&self.min_id)?;

        Ok(())
    }
 }

impl Serializable for MessagesReadMentions {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x0f0189d3_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;

        Ok(())
    }
 }

impl Serializable for MessagesGetRecentLocations {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x249431e2_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.limit)?;

        Ok(())
    }
 }

impl Serializable for MessagesSendMultiMedia {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x2095512f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.silent {
            ser_flags |= 1 << 5;
        }

        if self.background {
            ser_flags |= 1 << 6;
        }

        if self.clear_draft {
            ser_flags |= 1 << 7;
        }

        if self.reply_to_msg_id.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.peer.as_ref())?;

        if let Some(ref value) = self.reply_to_msg_id {
            buf.serialize(value)?;
        }

        buf.serialize(&self.multi_media)?;

        Ok(())
    }
 }

impl Serializable for MessagesUploadEncryptedFile {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x5057c497_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(self.file.as_ref())?;

        Ok(())
    }
 }

impl Serializable for UpdatesGetState {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xedd4882a_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for UpdatesGetDifference {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x25939651_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.pts_total_limit.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.pts)?;

        if let Some(ref value) = self.pts_total_limit {
            buf.serialize(value)?;
        }

        buf.serialize(&self.date)?;
        buf.serialize(&self.qts)?;

        Ok(())
    }
 }

impl Serializable for UpdatesGetChannelDifference {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x03173d78_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.force {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(self.filter.as_ref())?;
        buf.serialize(&self.pts)?;
        buf.serialize(&self.limit)?;

        Ok(())
    }
 }

impl Serializable for PhotosUpdateProfilePhoto {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf0bb5152_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.id.as_ref())?;

        Ok(())
    }
 }

impl Serializable for PhotosUploadProfilePhoto {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x4f32c098_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.file.as_ref())?;

        Ok(())
    }
 }

impl Serializable for PhotosDeletePhotos {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x87cf7f2f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for PhotosGetUserPhotos {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x91cd32a8_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.user_id.as_ref())?;
        buf.serialize(&self.offset)?;
        buf.serialize(&self.max_id)?;
        buf.serialize(&self.limit)?;

        Ok(())
    }
 }

impl Serializable for UploadSaveFilePart {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xb304a621_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.file_id)?;
        buf.serialize(&self.file_part)?;
        buf.serialize(&self.bytes)?;

        Ok(())
    }
 }

impl Serializable for UploadGetFile {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe3a6cfb5_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.location.as_ref())?;
        buf.serialize(&self.offset)?;
        buf.serialize(&self.limit)?;

        Ok(())
    }
 }

impl Serializable for UploadSaveBigFilePart {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xde7b673d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.file_id)?;
        buf.serialize(&self.file_part)?;
        buf.serialize(&self.file_total_parts)?;
        buf.serialize(&self.bytes)?;

        Ok(())
    }
 }

impl Serializable for UploadGetWebFile {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x24e6818d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.location.as_ref())?;
        buf.serialize(&self.offset)?;
        buf.serialize(&self.limit)?;

        Ok(())
    }
 }

impl Serializable for UploadGetCdnFile {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x2000bcc3_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.file_token)?;
        buf.serialize(&self.offset)?;
        buf.serialize(&self.limit)?;

        Ok(())
    }
 }

impl Serializable for UploadReuploadCdnFile {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x1af91c09_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.file_token)?;
        buf.serialize(&self.request_token)?;

        Ok(())
    }
 }

impl Serializable for UploadGetCdnFileHashes {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf715c87b_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.file_token)?;
        buf.serialize(&self.offset)?;

        Ok(())
    }
 }

impl Serializable for HelpGetConfig {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc4f9186b_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for HelpGetNearestDc {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x1fb33026_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for HelpGetAppUpdate {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xae2de196_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for HelpSaveAppLog {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x6f02f748_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.events)?;

        Ok(())
    }
 }

impl Serializable for HelpGetInviteText {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x4d392343_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for HelpGetSupport {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x9cdf08cd_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for HelpGetAppChangelog {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x9010ef6f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.prev_app_version)?;

        Ok(())
    }
 }

impl Serializable for HelpGetTermsOfService {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x350170f3_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for HelpSetBotUpdatesStatus {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xec22cfcd_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.pending_updates_count)?;
        buf.serialize(&self.message)?;

        Ok(())
    }
 }

impl Serializable for HelpGetCdnConfig {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x52029342_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for HelpGetRecentMeUrls {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3dc0f114_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.referer)?;

        Ok(())
    }
 }

impl Serializable for ChannelsReadHistory {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xcc104937_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.max_id)?;

        Ok(())
    }
 }

impl Serializable for ChannelsDeleteMessages {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x84c1fd4e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for ChannelsDeleteUserHistory {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xd10dd71b_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(self.user_id.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ChannelsReportSpam {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xfe087810_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(self.user_id.as_ref())?;
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for ChannelsGetMessages {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x93d7b347_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for ChannelsGetParticipants {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x123e05e9_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(self.filter.as_ref())?;
        buf.serialize(&self.offset)?;
        buf.serialize(&self.limit)?;
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for ChannelsGetParticipant {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x546dd7a6_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(self.user_id.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ChannelsGetChannels {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x0a7f6bbb_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for ChannelsGetFullChannel {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x08736a09_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ChannelsCreateChannel {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf4893d7f_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.broadcast {
            ser_flags |= 1 << 0;
        }

        if self.megagroup {
            ser_flags |= 1 << 1;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.title)?;
        buf.serialize(&self.about)?;

        Ok(())
    }
 }

impl Serializable for ChannelsEditAbout {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x13e27f1e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.about)?;

        Ok(())
    }
 }

impl Serializable for ChannelsEditAdmin {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x20b88214_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(self.user_id.as_ref())?;
        buf.serialize(self.admin_rights.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ChannelsEditTitle {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x566decd0_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.title)?;

        Ok(())
    }
 }

impl Serializable for ChannelsEditPhoto {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf12e57c9_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(self.photo.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ChannelsCheckUsername {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x10e6bd2c_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.username)?;

        Ok(())
    }
 }

impl Serializable for ChannelsUpdateUsername {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3514b3de_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.username)?;

        Ok(())
    }
 }

impl Serializable for ChannelsJoinChannel {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x24b524c5_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ChannelsLeaveChannel {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf836aa95_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ChannelsInviteToChannel {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x199f3a6c_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.users)?;

        Ok(())
    }
 }

impl Serializable for ChannelsExportInvite {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc7560885_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ChannelsDeleteChannel {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xc0111fe3_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ChannelsToggleInvites {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x49609307_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.enabled)?;

        Ok(())
    }
 }

impl Serializable for ChannelsExportMessageLink {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xceb77163_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.id)?;
        buf.serialize(&self.grouped)?;

        Ok(())
    }
 }

impl Serializable for ChannelsToggleSignatures {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x1f69b606_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.enabled)?;

        Ok(())
    }
 }

impl Serializable for ChannelsUpdatePinnedMessage {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xa72ded52_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.silent {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for ChannelsGetAdminedPublicChannels {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x8d8d82d7_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for ChannelsEditBanned {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xbfd915cd_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(self.user_id.as_ref())?;
        buf.serialize(self.banned_rights.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ChannelsGetAdminLog {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x33ddf480_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.events_filter.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.admins.is_some() {
            ser_flags |= 1 << 1;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.q)?;

        if let Some(ref value) = self.events_filter {
            buf.serialize(value.as_ref())?;
        }


        if let Some(ref value) = self.admins {
            buf.serialize(value)?;
        }

        buf.serialize(&self.max_id)?;
        buf.serialize(&self.min_id)?;
        buf.serialize(&self.limit)?;

        Ok(())
    }
 }

impl Serializable for ChannelsSetStickers {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xea8ca4f9_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(self.stickerset.as_ref())?;

        Ok(())
    }
 }

impl Serializable for ChannelsReadMessageContents {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xeab5dc38_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.id)?;

        Ok(())
    }
 }

impl Serializable for ChannelsDeleteHistory {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xaf369d42_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.max_id)?;

        Ok(())
    }
 }

impl Serializable for ChannelsTogglePreHistoryHidden {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xeabbb94c_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.channel.as_ref())?;
        buf.serialize(&self.enabled)?;

        Ok(())
    }
 }

impl Serializable for ChannelsGetFeed {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x18117df2_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.offset_to_max_read {
            ser_flags |= 1 << 3;
        }

        if self.offset_position.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.max_position.is_some() {
            ser_flags |= 1 << 1;
        }

        if self.min_position.is_some() {
            ser_flags |= 1 << 2;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.feed_id)?;

        if let Some(ref value) = self.offset_position {
            buf.serialize(value.as_ref())?;
        }

        buf.serialize(&self.add_offset)?;
        buf.serialize(&self.limit)?;

        if let Some(ref value) = self.max_position {
            buf.serialize(value.as_ref())?;
        }


        if let Some(ref value) = self.min_position {
            buf.serialize(value.as_ref())?;
        }

        buf.serialize(&self.sources_hash)?;
        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for ChannelsSearchFeed {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x88325369_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.feed_id)?;
        buf.serialize(&self.q)?;
        buf.serialize(&self.offset_date)?;
        buf.serialize(self.offset_peer.as_ref())?;
        buf.serialize(&self.offset_id)?;
        buf.serialize(&self.limit)?;

        Ok(())
    }
 }

impl Serializable for ChannelsGetFeedSources {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xd8ce236e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.feed_id.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;

        if let Some(ref value) = self.feed_id {
            buf.serialize(value)?;
        }

        buf.serialize(&self.hash)?;

        Ok(())
    }
 }

impl Serializable for ChannelsChangeFeedBroadcast {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x2528871e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.feed_id.is_some() {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.channel.as_ref())?;

        if let Some(ref value) = self.feed_id {
            buf.serialize(value)?;
        }


        Ok(())
    }
 }

impl Serializable for ChannelsSetFeedBroadcasts {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x7e91b8f2_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.feed_id)?;
        buf.serialize(&self.channels)?;
        buf.serialize(&self.also_newly_joined)?;

        Ok(())
    }
 }

impl Serializable for ChannelsReadFeed {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x09c3011d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.feed_id)?;
        buf.serialize(self.max_position.as_ref())?;

        Ok(())
    }
 }

impl Serializable for BotsSendCustomRequest {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xaa2769ed_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.custom_method)?;
        buf.serialize(self.params.as_ref())?;

        Ok(())
    }
 }

impl Serializable for BotsAnswerWebhookJsonquery {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xe6213f4d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.query_id)?;
        buf.serialize(self.data.as_ref())?;

        Ok(())
    }
 }

impl Serializable for PaymentsGetPaymentForm {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x99f09745_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.msg_id)?;

        Ok(())
    }
 }

impl Serializable for PaymentsGetPaymentReceipt {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xa092a980_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.msg_id)?;

        Ok(())
    }
 }

impl Serializable for PaymentsValidateRequestedInfo {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x770a8e74_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.save {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.msg_id)?;
        buf.serialize(self.info.as_ref())?;

        Ok(())
    }
 }

impl Serializable for PaymentsSendPaymentForm {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x2b8879b3_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.requested_info_id.is_some() {
            ser_flags |= 1 << 0;
        }

        if self.shipping_option_id.is_some() {
            ser_flags |= 1 << 1;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(&self.msg_id)?;

        if let Some(ref value) = self.requested_info_id {
            buf.serialize(value)?;
        }


        if let Some(ref value) = self.shipping_option_id {
            buf.serialize(value)?;
        }

        buf.serialize(self.credentials.as_ref())?;

        Ok(())
    }
 }

impl Serializable for PaymentsGetSavedInfo {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x227d824b_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for PaymentsClearSavedInfo {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xd83d70c1_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.credentials {
            ser_flags |= 1 << 0;
        }

        if self.info {
            ser_flags |= 1 << 1;
        }

        buf.serialize(&ser_flags)?;

        Ok(())
    }
 }

impl Serializable for StickersCreateStickerSet {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x9bd86e6a_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        let mut ser_flags: u32 = 0;

        if self.masks {
            ser_flags |= 1 << 0;
        }

        buf.serialize(&ser_flags)?;
        buf.serialize(self.user_id.as_ref())?;
        buf.serialize(&self.title)?;
        buf.serialize(&self.short_name)?;
        buf.serialize(&self.stickers)?;

        Ok(())
    }
 }

impl Serializable for StickersRemoveStickerFromSet {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xf7760f51_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.sticker.as_ref())?;

        Ok(())
    }
 }

impl Serializable for StickersChangeStickerPosition {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0xffb6d4ca_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.sticker.as_ref())?;
        buf.serialize(&self.position)?;

        Ok(())
    }
 }

impl Serializable for StickersAddStickerToSet {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x8653febe_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.stickerset.as_ref())?;
        buf.serialize(self.sticker.as_ref())?;

        Ok(())
    }
 }

impl Serializable for PhoneGetCallConfig {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x55451fa9_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

impl Serializable for PhoneRequestCall {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x5b95b3d4_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.user_id.as_ref())?;
        buf.serialize(&self.random_id)?;
        buf.serialize(&self.g_a_hash)?;
        buf.serialize(self.protocol.as_ref())?;

        Ok(())
    }
 }

impl Serializable for PhoneAcceptCall {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x3bd2b4a0_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.g_b)?;
        buf.serialize(self.protocol.as_ref())?;

        Ok(())
    }
 }

impl Serializable for PhoneConfirmCall {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x2efe1722_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.g_a)?;
        buf.serialize(&self.key_fingerprint)?;
        buf.serialize(self.protocol.as_ref())?;

        Ok(())
    }
 }

impl Serializable for PhoneReceivedCall {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x17d54f61_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;

        Ok(())
    }
 }

impl Serializable for PhoneDiscardCall {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x78d413a6_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.duration)?;
        buf.serialize(self.reason.as_ref())?;
        buf.serialize(&self.connection_id)?;

        Ok(())
    }
 }

impl Serializable for PhoneSetCallRating {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x1c536a34_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(&self.rating)?;
        buf.serialize(&self.comment)?;

        Ok(())
    }
 }

impl Serializable for PhoneSaveCallDebug {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x277add7e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(self.peer.as_ref())?;
        buf.serialize(self.debug.as_ref())?;

        Ok(())
    }
 }

impl Serializable for LangpackGetLangPack {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x9ab5c58e_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.lang_code)?;

        Ok(())
    }
 }

impl Serializable for LangpackGetStrings {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x2e1ee318_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.lang_code)?;
        buf.serialize(&self.keys)?;

        Ok(())
    }
 }

impl Serializable for LangpackGetDifference {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x0b2e4d7d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&self.from_version)?;

        Ok(())
    }
 }

impl Serializable for LangpackGetLanguages {
    #[inline]
    fn serialize_into<B: Write>(&self, buf: &mut B) -> Result<()> {
        buf.serialize(&0x800fd57d_u32)?; 
        self.serialize_bare(buf)
    }

    #[allow(unused_variables, unused_mut)]
    fn serialize_bare<B: Write>(&self, buf: &mut B) -> Result<()> {

        Ok(())
    }
 }

