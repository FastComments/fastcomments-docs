[api-resource-header-start name = 'Page Reacts'; route = 'POST /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

वर्तमान उपयोगकर्ता के रूप में किसी पेज पर एक रिएक्शन जोड़ता है। एक उपयोगकर्ता प्रत्येक रिएक्शन id को केवल एक बार जोड़ सकता है: इसे दोबारा जोड़ने पर `already-reacted` कोड के साथ सफलता मिलती है और काउंट नहीं बदलता। एक उपयोगकर्ता एक ही पेज पर कई अलग-अलग रिएक्शन जोड़ सकता है।

रिएक्शन ids आपके द्वारा चुने जाते हैं और 36 वर्णों तक के हो सकते हैं। यदि पेज अभी तक मौजूद नहीं है तो उसे बना दिया जाता है। पेज का शीर्षक सेट या अपडेट करने के लिए `title` पास करें।

[inline-code-attrs-start title = 'पेज रिएक्शन cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url&id=heart'
[inline-code-end]

[inline-code-attrs-start title = 'पेज रिएक्शन रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactRequestQueryParams {
    urlId: string
    /** रिएक्शन id, 36 वर्णों तक। **/
    id: string
    /** पेज का शीर्षक सेट करता है। **/
    title?: string
    /** आपके SSO ऑब्जेक्ट का URI एनकोडेड JSON। अनाम उपयोगकर्ताओं के लिए छोड़ दें। **/
    sso?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'पेज रिएक्शन रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactResponse {
    status: 'success' | 'failed'
    /** 'already-reacted' जब उपयोगकर्ता ने पहले ही यह रिएक्शन जोड़ लिया हो। 'react-id-too-long' (HTTP 422) जब id 36 वर्णों से अधिक हो। अन्यथा विफलता पर शामिल। **/
    code?: 'already-reacted' | 'react-id-too-long' | string
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]