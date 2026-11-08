[api-resource-header-start name = 'FeedPost'; route = 'POST /api/v1/feed-posts'; creditsCost = 1; api-resource-header-end]

यह रूट एक सिंगल `FeedPost` बनाता है। हर पोस्ट का एक लेखक होता है, इसलिए `fromUserId` आवश्यक है और यह खाते पर मौजूद किसी FastComments या SSO उपयोगकर्ता की id होनी चाहिए।

[inline-code-attrs-start title = 'FeedPost बनाने का cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/feed-posts?tenantId=demo&isLive=true&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "fromUserId": "some-user-id",
    "title": "Release 2.0 is out",
    "contentHTML": "<p>Read the notes and tell us what you think.</p>",
    "tags": ["releases"],
    "links": [
        {
            "url": "https://example.com/releases/2.0",
            "title": "Release notes",
            "description": "Everything that changed in 2.0."
        }
    ]
}'
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost बनाने की रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostQueryParams {
    tenantId: string
    API_KEY: string
    /** पोस्ट को उन फ़ीड्स पर भेजें जो अभी ब्राउज़र में खुले हैं। डिफ़ॉल्ट false है। **/
    isLive?: boolean
    /** सेव करने से पहले पोस्ट को स्पैम इंजन से गुज़ारें। डिफ़ॉल्ट false है। **/
    doSpamCheck?: boolean
    /** doSpamCheck के हिस्से के रूप में चलने वाली दोहराए गए कंटेंट की जाँच को छोड़ दें। डिफ़ॉल्ट false है। **/
    skipDupCheck?: boolean
    /** 256 वर्णों तक। लाइव लिसनर्स को भेजा जाता है ताकि क्लाइंट अपना खुद का ब्रॉडकास्ट अनदेखा कर सके। **/
    broadcastId?: string
}

interface FeedPostPostBody {
    /** आवश्यक। एक FastComments या SSO उपयोगकर्ता id। **/
    fromUserId: string
    title?: string
    /** HTML। सेव करते समय सैनिटाइज़ किया जाता है। **/
    contentHTML?: string
    /** उपयोगकर्ता से लिया गया डिस्प्ले नाम ओवरराइड करता है। **/
    fromUserDisplayName?: string
    tags?: string[]
    media?: FeedPostMediaItem[]
    links?: FeedPostLink[]
    meta?: Record<string, string>
}
[inline-code-end]

[inline-code-attrs-start title = 'FeedPost बनाने की रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface FeedPostPostResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-from-user-id' | 'invalid-user' | 'broadcast-id-too-long' | 'spam-blocked' | 'user-rate-limited' | 'internal'
    /** विफलता पर शामिल। **/
    reason?: string
    feedPost?: FeedPost; // सफलता पर हम पूरी बनाई गई पोस्ट वापस करते हैं।
}
[inline-code-end]