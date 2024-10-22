use serde::{Serialize,Deserialize};
use time::OffsetDateTime;
use crate::errors::{TwitterScrapeError, TwitterAuthError};
use playwright::{api::Page, Playwright};
use log::{error, info, warn};
use anyhow::{anyhow,Result};


#[derive(Debug,Serialize,Deserialize,Clone)]
pub struct TwitterCredentials{
    pub username:String,
    pub password:String,
    pub email:String
}


pub struct Tweet{
    pub id:String,
    pub date: OffsetDateTime
}

pub struct TwitterAuth{
    credentials:TwitterCredentials,
}

impl TwitterAuth{
    pub fn new(credentials: TwitterCredentials)-> Self{
        TwitterAuth{
            credentials
        }
    }
    pub async fn check_login_required(&self,page:&Page)-> Result<bool>{
        let login_indicator = page
            .query_selector("input[autocomplete=\"username\"], form[action=\"/i/flow/login\"]")
            .await.expect("failed to find the login indicator");
        Ok(login_indicator.is_some())
    }
    pub async fn authenticate(&self, page: &Page)->Result<bool>{
        info!("Authenticating Started.\n");
        info!("Waiting for username input...");

        page.wait_for_selector_builder("input[autocomplete=\"username\"]")
            .timeout(10000f64)
            .wait_for_selector().await?;

        page.fill_builder("input[autocomplete=\"username\"]", &self.credentials.username)
            .timeout(100f64)
            .fill()
            .await?;

        page.click_builder("button[role=\"button\"]:has-text(\"Next\")")
            .timeout(3000f64)
            .click()
            .await?;
        page.wait_for_timeout(3000.0).await;

        if let Ok(pass_entry) = page.wait_for_selector_builder("input[type=\"password\"]")
            .timeout(1000.0)
            .wait_for_selector().await {

            if pass_entry.is_none(){
                info!("Waiting for email input...");
                page.wait_for_selector_builder("input[type=\"text\"]")
                    .timeout(100f64)
                    .wait_for_selector().await?;

                page.fill_builder("input[type=\"text\"]", &self.credentials.email)
                    .fill().await?;

                page.click_builder("button[type=\"button\"]:has-text(\"Next\")").click().await?;
            }
        }

        page.wait_for_selector_builder("input[type=\"password\"]")
            .wait_for_selector()
            .await?;
        page.fill_builder("input[type=\"password\"]", &self.credentials.password)
            .fill()
            .await?;
        page.click_builder("button[type=\"button\"]:has-text(\"Log in\")")
            .click()
            .await?;

        page.wait_for_timeout(5000.0);

        match page.wait_for_selector_builder("[data-testid=\"AppTabBar_Home_Link\"], article[data-testid=\"tweet\"]")
            .wait_for_selector().await {
            Ok(_) => {
                info!("Authentication Successful.\n");
                Ok(true)
            },
            Err(e) => {
                error!("Authentication Failed.\n");
                Ok(false)
            }
        }
    }
}


