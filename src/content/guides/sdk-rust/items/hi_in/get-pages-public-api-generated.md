किसी टेनेंट के लिए पेजों की सूची लाता है। FChat डेस्कटॉप क्लाइंट द्वारा अपनी रूम सूची भरने के लिए उपयोग किया जाता है।
प्रत्येक पेज के लिए रिज़ॉल्व किए गए कस्टम कॉन्फ़िग में `enableFChat` true होना आवश्यक है।
SSO की आवश्यकता वाले पेज अनुरोध करने वाले उपयोगकर्ता के ग्रुप एक्सेस के आधार पर फ़िल्टर किए जाते हैं।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenant_id | String | हाँ |  |
| cursor | String | नहीं |  |
| limit | i32 | नहीं |  |
| q | String | नहीं |  |
| sort_by | models::PagesSortBy | नहीं |  |
| has_comments | bool | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-rust/blob/main/client/src/models/get_public_pages_response.rs)

## उदाहरण

[inline-code-attrs-start title = 'get_pages_public उदाहरण'; type = 'rust'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async fn example(configuration: &configuration::Configuration) -> Result<(), Error> {
    let params = GetPagesPublicParams {
        tenant_id: "acme-corp-tenant".to_string(),
        cursor: Some("page_20".to_string()),
        limit: Some(50),
        q: Some("news/article".to_string()),
        sort_by: Some(models::PagesSortBy::CreatedDesc),
        has_comments: Some(true),
    };
    let _response = get_pages_public(configuration, params).await?;
    Ok(())
}
[inline-code-end]