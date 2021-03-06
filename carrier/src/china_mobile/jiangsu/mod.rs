mod model;

use chrono::Utc;
use reqwest::Client;
use serde_xml_rs::to_string;
use isahc::ResponseExt;

use crate::china_mobile::jiangsu::model::CardRequest;
use crate::{CardInfo, CardStatus, CarrierClient, Result};

static API_URL: &str = "http://221.178.251.182:80/internet_surfing";

// 江苏移动帐号密码信息
#[derive(Debug)]
pub struct JiangsuMobileClient {
    pub app_id: String,
    pub password: String,
    pub group_id: String,
    pub city_id: String,
}

impl JiangsuMobileClient {
    pub fn new(app_id: &str, password: &str, group_id: &str, city_id: &str) -> Self {
        JiangsuMobileClient {
            app_id: app_id.to_owned(),
            password: password.to_owned(),
            group_id: group_id.to_owned(),
            city_id: city_id.to_owned(),
        }
    }
    pub fn request(
        &self,
        process_code: &str,
        sign: &str,
        verify_code: &str,
        req_type: &str,
        terminal_id: &str,
        accept_seq: &str,
        req_seq: &str,
        iccid: &str,
        msisdn: &str,
        telnum: &str,
        service_number: &str,
        cycle: &str,
        oprtype: &str,
        reason: &str,
        service: &str,
        sub_service_status: &str,
    ) -> Result<String> {
        let dt = Utc::now().format("%Y%m%d%H%M%S").to_string();
        let item = dbg!(CardRequest::new(
            process_code,
            &self.app_id,
            &self.password,
            sign,
            verify_code,
            req_type,
            terminal_id,
            accept_seq,
            req_seq,
            &dt,
            &self.group_id,
            &self.city_id,
            iccid,
            msisdn,
            telnum,
            service_number,
            cycle,
            oprtype,
            reason,
            service,
            sub_service_status,
        ));
        let s = crate::isahc_client()?
            .post(API_URL, item)?
            // .body(item)
            // .send()?
            .text()?;
        for token in xmlparser::Tokenizer::from(s.as_ref()) {
            println!("{:?}", token);
        };
        Ok("123".to_string())
    }
}

impl CarrierClient for JiangsuMobileClient {}
