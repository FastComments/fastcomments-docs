[api-resource-header-start name = 'Page Likes'; route = 'DELETE /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

वर्तमान उपयोगकर्ता का लाइक पेज से हटाता है। यदि उपयोगकर्ता ने पेज को लाइक नहीं किया है, तो अनुरोध `not-liked` कोड के साथ सफल होता है और गिनती नहीं बदलती।

[inline-code-attrs-start title = 'पेज अनलाइक cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'पेज अनलाइक अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeRequestQueryParams {
    urlId: string
    /** आपके SSO ऑब्जेक्ट का URI एन्कोडेड JSON। गुमनाम उपयोगकर्ताओं के लिए छोड़ दें। **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'पेज अनलाइक प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageUnlikeResponse {
    status: 'success' | 'failed'
    /** 'not-liked' जब उपयोगकर्ता ने पेज को लाइक नहीं किया था। अन्यथा विफलता पर शामिल। **/
    code?: 'not-liked' | string
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]