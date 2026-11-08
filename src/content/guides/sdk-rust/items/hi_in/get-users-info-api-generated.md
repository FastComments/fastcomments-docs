किसी टेनेंट के लिए बल्क यूज़र जानकारी। दिए गए userIds के लिए, User / SSOUser से डिस्प्ले जानकारी लौटाता है।
कमेंट विजेट द्वारा उन यूज़र्स को समृद्ध (enrich) करने के लिए उपयोग किया जाता है जो अभी प्रेज़ेंस इवेंट के माध्यम से दिखाई दिए।
कोई पेज संदर्भ नहीं: गोपनीयता समान रूप से लागू की जाती है (प्राइवेट प्रोफाइल मास्क कर दिए जाते हैं)।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| ids | String | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/page_users_info_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_users_info उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let params = GetUsersInfoParams {
    tenant_id: "acme-corp-tenant".to_string(),
    ids: "user-1,user-2".to_string(),
};
let page: PageUsersInfoResponse = get_users_info(&configuration, params).await?;
[inline-code-end]