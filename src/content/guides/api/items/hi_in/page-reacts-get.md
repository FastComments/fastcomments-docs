[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

किसी पेज पर प्रत्येक रिएक्शन की संख्या, और वर्तमान उपयोगकर्ता द्वारा जोड़े गए रिएक्शन लौटाता है।

रिएक्ट करने वाले उपयोगकर्ताओं को भी पाने के लिए `includeUserInfo=public` पास करें, जिसमें प्रत्येक रिएक्ट के लिए एक प्रविष्टि उसके रिएक्शन id के साथ होती है, सबसे हालिया पहले। निजी प्रोफ़ाइल गतिविधि वाले उपयोगकर्ता और गुमनाम उपयोगकर्ता छोड़ दिए जाते हैं, और अधिकतम 100 रिएक्ट देखे जाते हैं। यहाँ केवल `public` स्वीकार किया जाता है; `include-private` एक `422` लौटाता है और API कुंजी के साथ [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) से उपलब्ध है। उन पेजों पर जिन्हें SSO की आवश्यकता होती है, `includeUserInfo` उन उपयोगकर्ताओं के लिए `403` लौटाता है जो पेज नहीं देख सकते।

[inline-code-attrs-start title = 'Page Reacts cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** आपके SSO ऑब्जेक्ट का URI encoded JSON। गुमनाम उपयोगकर्ताओं के लिए छोड़ दें। **/
    sso?: string
    /** रिएक्ट करने वाले उपयोगकर्ताओं को शामिल करने के लिए 'public' सेट करें। **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** विफलता पर शामिल। **/
    reason?: string
    /** प्रत्येक रिएक्शन id के लिए संख्या, उदाहरण के लिए {"heart": 12, "laugh": 3}। पेज पर कोई रिएक्शन न होने पर सेट नहीं होता। **/
    counts?: Record<string, number>
    /** अनुरोध करने वाले उपयोगकर्ता द्वारा जोड़े गए रिएक्शन ids। उन्होंने कोई न जोड़ा होने पर सेट नहीं होता। **/
    reactedIds?: string[]
    /** प्रत्येक रिएक्ट के लिए एक प्रविष्टि, सबसे हालिया पहले। केवल तब सेट होता है जब includeUserInfo पास किया गया हो। **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** रिएक्शन id, उदाहरण के लिए "heart"। **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]