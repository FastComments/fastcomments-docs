[api-resource-header-start name = 'Page Likes'; route = 'POST /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

वर्तमान उपयोगकर्ता के रूप में पेज को लाइक करता है। प्रत्येक उपयोगकर्ता किसी पेज को एक बार लाइक कर सकता है: दोबारा लाइक करने पर `already-liked` कोड के साथ सफलता मिलती है और गिनती नहीं बदलती।

यदि पेज अभी तक मौजूद नहीं है तो उसे बनाया जाता है। पेज का टाइटल सेट या अपडेट करने के लिए `title` पास करें।

[inline-code-attrs-start title = 'पेज लाइक cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url&title=My%20Article'
[inline-code-end]

[inline-code-attrs-start title = 'पेज लाइक रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeRequestQueryParams {
    urlId: string
    /** पेज का टाइटल सेट करता है। **/
    title?: string
    /** आपके SSO ऑब्जेक्ट का URI एनकोडेड JSON। अनाम उपयोगकर्ताओं के लिए छोड़ दें। **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'पेज लाइक रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikeResponse {
    status: 'success' | 'failed'
    /** 'already-liked' जब उपयोगकर्ता पहले ही पेज लाइक कर चुका हो। अन्यथा विफलता पर शामिल। **/
    code?: 'already-liked' | string
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]