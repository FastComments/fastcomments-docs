[api-resource-header-start name = 'Page Likes'; route = 'GET /api/v1/pages/likes'; creditsCost = 1; api-resource-header-end]

किसी पेज पर लाइक की संख्या, और वैकल्पिक रूप से उन उपयोगकर्ताओं की सूची लौटाता है जिन्होंने उसे लाइक किया, सबसे हालिया पहले। यह [`GET /page-reacts/v1/likes/:tenantId`](/guide-api.html#page-likes-get) का सर्वर-साइड संस्करण है।

`includeUserInfo` नियंत्रित करता है कि कौन से उपयोगकर्ता लौटाए जाते हैं:

- `public` उन उपयोगकर्ताओं को छोड़ देता है जिन्होंने अपनी प्रोफ़ाइल गतिविधि निजी कर रखी है।
- `include-private` उन्हें शामिल करता है। इसका उपयोग केवल तब करें जब सूची सार्वजनिक रूप से न दिखाई जा रही हो।

गुमनाम (Anonymous) उपयोगकर्ता कभी शामिल नहीं किए जाते, और अधिकतम 100 लाइक ही देखे जाते हैं। जो पेज मौजूद नहीं हैं वे `likeCount` के रूप में `0` और खाली `users` सूची लौटाते हैं।

[inline-code-attrs-start title = 'Page Likes cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/likes?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** केवल लाइक संख्या प्राप्त करने के लिए इसे छोड़ दें। **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Likes रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** विफलता पर शामिल। **/
    reason?: string
    likeCount?: number
    /** सबसे हालिया पहले। केवल तब सेट होता है जब includeUserInfo पास किया गया हो। **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### उपयोगी सुझाव

`urlId` जैसे मानों को URI Encode करना न भूलें।