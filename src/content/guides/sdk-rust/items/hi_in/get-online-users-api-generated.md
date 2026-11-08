किसी पेज के वर्तमान में ऑनलाइन दर्शक: वे लोग जिनका websocket सेशन इस समय पेज की सदस्यता लिए हुए है।
anonCount + totalCount लौटाता है (पूरे रूम के सब्सक्राइबर, जिनमें वे अनाम दर्शक भी शामिल हैं जिन्हें हम सूचीबद्ध नहीं करते)।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| url_id | String | हाँ |  |
| after_name | String | नहीं |  |
| after_user_id | String | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_online_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_online_users उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example() -> Result<(), Error> {
    let params = GetOnlineUsersParams {
        tenant_id: "acme-corp-tenant".to_string(),
        url_id: "news/article".to_string(),
        after_name: Some("john_doe".to_string()),
        after_user_id: Some("user-123".to_string()),
    };
    let _response: PageUsersOnlineResponse = get_online_users(&config, params).await?;
    Ok(())
}
[inline-code-end]