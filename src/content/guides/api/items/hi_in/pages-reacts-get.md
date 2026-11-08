[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

किसी पेज पर प्रत्येक रिएक्शन की संख्या लौटाता है, और वैकल्पिक रूप से उन उपयोगकर्ताओं को भी जिन्होंने रिएक्ट किया, प्रत्येक रिएक्ट के लिए एक एंट्री के रूप में उसके रिएक्शन आईडी के साथ, सबसे हालिया पहले। यह [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get) का सर्वर-साइड संस्करण है।

`includeUserInfo` नियंत्रित करता है कि कौन से उपयोगकर्ता लौटाए जाते हैं:

- `public` उन उपयोगकर्ताओं को छोड़ देता है जिन्होंने अपनी प्रोफ़ाइल गतिविधि निजी रखी है।
- `include-private` उन्हें शामिल करता है। इसका उपयोग केवल तब करें जब सूची सार्वजनिक रूप से न दिखाई जा रही हो।

गुमनाम (Anonymous) उपयोगकर्ता कभी शामिल नहीं किए जाते, और अधिकतम 100 रिएक्ट देखे जाते हैं। जो पेज मौजूद नहीं हैं वे कोई `counts` नहीं और खाली `users` सूची लौटाते हैं।

[inline-code-attrs-start title = 'Page Reacts cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** केवल counts प्राप्त करने के लिए इसे छोड़ दें। **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** विफलता पर शामिल। **/
    reason?: string
    /** प्रत्येक रिएक्शन आईडी के लिए संख्या, उदाहरण के लिए {"heart": 12, "laugh": 3}। पेज पर कोई रिएक्शन न होने पर सेट नहीं होता। **/
    counts?: Record<string, number>
    /** प्रत्येक रिएक्ट के लिए एक एंट्री, सबसे हालिया पहले। केवल तब सेट होता है जब includeUserInfo पास किया गया हो। **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** रिएक्शन आईडी, उदाहरण के लिए "heart"। **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### उपयोगी सुझाव

`urlId` जैसे मानों को URI Encode करना न भूलें।