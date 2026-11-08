[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

किसी पेज पर लाइक की संख्या लौटाता है, और यह भी कि क्या वर्तमान उपयोगकर्ता ने उसे लाइक किया है। जो पेज अभी तक मौजूद नहीं हैं, वे `likeCount` के रूप में `0` लौटाते हैं।

पेज को लाइक करने वाले उपयोगकर्ताओं को भी पाने के लिए, नवीनतम पहले, `includeUserInfo=public` पास करें। निजी प्रोफ़ाइल गतिविधि वाले उपयोगकर्ताओं और गुमनाम उपयोगकर्ताओं को छोड़ दिया जाता है, और अधिकतम 100 लाइक देखे जाते हैं। यहाँ केवल `public` स्वीकार किया जाता है; `include-private` एक `422` लौटाता है और यह API कुंजी के साथ [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) से उपलब्ध है। जिन पेजों पर SSO आवश्यक है, वहाँ `includeUserInfo` उन उपयोगकर्ताओं के लिए `403` लौटाता है जो पेज नहीं देख सकते।

[inline-code-attrs-start title = 'Page Likes cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** आपके SSO ऑब्जेक्ट का URI एनकोडेड JSON। गुमनाम उपयोगकर्ताओं के लिए छोड़ दें। **/
    sso?: string
    /** पेज को लाइक करने वाले उपयोगकर्ताओं को शामिल करने के लिए 'public' सेट करें। **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** विफलता पर शामिल। **/
    reason?: string
    likeCount: number
    /** क्या अनुरोध करने वाले उपयोगकर्ता ने पेज लाइक किया है। **/
    didLike: boolean
    /** पेज पर टॉप लेवल कमेंट्स की संख्या। **/
    commentCount: number
    /** इस पेज के लिए लाइव अपडेट की सदस्यता लेने के लिए उपयोग की जाने वाली id। **/
    urlIdWS: string
    /** नवीनतम पहले। केवल तब सेट होता है जब includeUserInfo पास किया गया हो। **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]