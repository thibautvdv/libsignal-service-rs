use crate::push_service::{
    ConfirmDeviceMessage, DeviceId, PushService, ServiceError,
    SmsVerificationCodeResponse, VoiceVerificationCodeResponse,
};

pub struct AccountManager<Service> {
    service: Service,
}

impl<Service: PushService> AccountManager<Service> {
    pub fn new(service: Service) -> Self {
        Self { service }
    }

    pub async fn request_sms_verification_code(
        &mut self,
        phone_number: &str,
    ) -> Result<SmsVerificationCodeResponse, ServiceError> {
        Ok(self
            .service
            .request_sms_verification_code(phone_number)
            .await?)
    }

    pub async fn request_voice_verification_code(
        &mut self,
        phone_number: &str,
    ) -> Result<VoiceVerificationCodeResponse, ServiceError> {
        Ok(self
            .service
            .request_voice_verification_code(phone_number)
            .await?)
    }

    pub async fn confirm_device(
        &mut self,
        confirmation_code: u32,
        confirm_device_message: ConfirmDeviceMessage,
    ) -> Result<DeviceId, ServiceError> {
        Ok(self
            .service
            .confirm_device(confirmation_code, confirm_device_message)
            .await?)
    }
}
