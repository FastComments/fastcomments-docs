पेज पर उन पिछले टिप्पणीकारों का डेटा जो इस समय ऑनलाइन नहीं हैं। displayName के अनुसार क्रमबद्ध।
"Members" सेक्शन रेंडर करने के लिए /users/online का उपयोग पूरा करने के बाद इसका उपयोग करें।
commenterName पर कर्सर पेजिनेशन: सर्वर afterName से आगे $gt के माध्यम से आंशिक {tenantId, urlId, commenterName}
इंडेक्स को ट्रैवर्स करता है, बिना $skip लागत के।

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenant_id | String | Yes |  |
| url_id | String | Yes |  |
| after_name | String | No |  |
| after_user_id | String | No |  |

## Response

रिटर्न करता है: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_offline_response.rs)

## Example

[inline-code-attrs-start title = 'get_offline_users उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn fetch_offline(config: &configuration::Configuration) -> Result<(), Error> {
    let params = GetOfflineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("alice".to_string()),
        after_user_id: Some("user-42".to_string()),
    };
    let _response = get_offline_users(config, params).await?;
    Ok(())
}
[inline-code-end]