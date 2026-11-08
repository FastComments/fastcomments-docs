किसी पेज के लिए सूचनाएँ (notifications) सक्षम या अक्षम करें। जब उपयोगकर्ता किसी पेज की सदस्यता लेते हैं, तो नई रूट टिप्पणियों के लिए सूचनाएँ बनाई जाती हैं, और साथ ही

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| url_id | String | हाँ |  |
| url | String | हाँ |  |
| page_title | String | हाँ |  |
| subscribed_or_unsubscribed | String | हाँ |  |
| sso | String | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/update_user_notification_page_subscription_status_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'update_user_notification_page_subscription_status उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<UpdateUserNotificationPageSubscriptionStatusResponse, Error> {
    let params = UpdateUserNotificationPageSubscriptionStatusParams {
        tenant_id: "acme-corp-tenant".to_owned(),
        url_id: "news-article-2024".to_owned(),
        url: "https://news.example.com/articles/rust".to_owned(),
        page_title: "Rust Dominates the Programming World".to_owned(),
        subscribed_or_unsubscribed: "subscribed".to_owned(),
        sso: Some("sso-token-abc".to_owned()),
    };
    update_user_notification_page_subscription_status(&configuration, params).await
}
[inline-code-end]