[api-resource-header-start name = 'Page React Users'; route = 'GET /page-reacts/v2/:tenantId/list'; creditsCost = 0; api-resource-header-end]

किसी पेज पर रिएक्शन जोड़ने वाले उपयोगकर्ताओं के नाम, वर्णानुक्रम में क्रमबद्ध करके लौटाता है। 100 तक रिएक्ट देखे जाते हैं, और गुमनाम उपयोगकर्ता तथा निजी प्रोफ़ाइल गतिविधि वाले उपयोगकर्ता शामिल नहीं होते। SSO आवश्यक वाले पेजों पर, जो उपयोगकर्ता पेज नहीं देख सकते उन्हें `sso-required` या `page-forbidden` कोड के साथ `403` मिलता है।

अवतार और उपयोगकर्ता ids भी प्राप्त करने के लिए, `includeUserInfo=public` के साथ [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) का उपयोग करें।

[inline-code-attrs-start title = 'Page React Users cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo/list?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'Page React Users रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersRequestQueryParams {
    urlId: string
    /** रिएक्शन id। **/
    id: string
    /** आपके SSO ऑब्जेक्ट का URI encoded JSON। गुमनाम उपयोगकर्ताओं के लिए छोड़ दें। **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page React Users रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactUsersResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: string
    /** विफलता पर शामिल। **/
    reason?: string
    userNames: string[]
}
[inline-code-end]