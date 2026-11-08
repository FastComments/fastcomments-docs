[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

यह रूट एकल अधिकृत `Vote` जोड़ने की क्षमता प्रदान करता है। वोट `up` (+1) या `down` (-1) हो सकते हैं।

[inline-code-attrs-start title = 'वोट बनाने का cURL उदाहरण'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'गुमनाम वोट बनाने का cURL उदाहरण'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'वोट बनाने के अनुरोध की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'वोट बनाने की प्रतिक्रिया की संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Included on failure. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### गुमनाम वोट बनाना

गुमनाम वोट `userId` के स्थान पर क्वेरी पैरामीटर में `anonUserId` सेट करके बनाए जा सकते हैं।

यह id को कहीं भी किसी user ऑब्जेक्ट से मेल खाना ज़रूरी नहीं है (इसीलिए यह गुमनाम है)। यह केवल सेशन के लिए एक पहचानकर्ता है,
ताकि आप उसी सेशन में वोट को फिर से प्राप्त कर सकें और जांच सकें कि किसी कमेंट पर
वोट किया गया है या नहीं।

यदि आपके पास FastComments की तरह "गुमनाम सेशन" जैसी कोई चीज़ नहीं है - तो आप इसे बस
एक यादृच्छिक ID, जैसे UUID पर सेट कर सकते हैं (हालाँकि स्थान बचाने के लिए हम छोटे पहचानकर्ताओं को प्राथमिकता देते हैं)।

### अन्य नोट्स

- यह API टेनेंट-स्तरीय सेटिंग्स का पालन करता है। उदाहरण के लिए, यदि आप किसी दिए गए पेज के लिए वोटिंग अक्षम करते हैं, और API के माध्यम से वोट बनाने का प्रयास करते हैं, तो यह त्रुटि कोड `voting-disabled` के साथ विफल हो जाएगा।
- यह API डिफ़ॉल्ट रूप से लाइव है।
- यह API संबंधित `Comment` के `votes` को अपडेट करेगा।