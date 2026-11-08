[api-resource-header-start name = 'Poll'; route = 'DELETE /api/v1/polls/:commentId'; creditsCost = 1; api-resource-header-end]

किसी पोल को उसके कमेंट से हटा देता है, साथ ही उस पर डाले गए सभी वोट भी हटा देता है। कमेंट खुद वैसा ही रहता है।

कमेंट को डिलीट करने से भी उसका पोल और वोट हट जाते हैं, इसलिए यह केवल तब ज़रूरी है जब आप कमेंट को बनाए रखना चाहते हैं।

[inline-code-attrs-start title = 'पोल डिलीट cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/polls/comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'पोल डिलीट रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'पोल डिलीट रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PollDeleteResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'poll-not-found'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]