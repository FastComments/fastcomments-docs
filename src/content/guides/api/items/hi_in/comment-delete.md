[api-resource-header-start name = 'Comment'; route = 'DELETE /api/v1/comments/:id'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट एक कमेंट को डिलीट करने की सुविधा प्रदान करता है।

नोट्स:

- यह API चाहें तो कमेंट विजेट को "live" अपडेट कर सकता है (इससे `creditsCost` `1` से बढ़कर `2` हो जाता है)।
- यह API सभी चाइल्ड कमेंट्स को डिलीट कर देगा।
- यदि टारगेट कमेंट लॉक है (`isLocked: true`), तो रिक्वेस्ट `code: 'locked'` के साथ अस्वीकार कर दी जाती है। पहले कमेंट को अनलॉक करें, फिर डिलीट करें।

[inline-code-attrs-start title = 'Comment DELETE cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Comment DELETE रिक्वेस्ट स्ट्रक्चर'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentDeleteQueryParams {
    tenantId: string
    API_KEY: string
	/** अपडेट करने वाला यूज़र। चाहें तो इसका उपयोग यह जांचने के लिए किया जा सकता है कि वे कमेंट डिलीट कर सकते हैं।  **/
    contextUserId?: string
	/** क्या कमेंट को उसी urlId वाले कमेंट विजेट के इंस्टेंस देख रहे यूज़र्स के लिए "live" डिलीट किया जाना चाहिए। नोट: क्रेडिट लागत 1 से बढ़कर 2 हो जाती है। **/
    isLive?: 'true' | 'false'
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment DELETE रिस्पॉन्स स्ट्रक्चर'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentDeleteResponse {
    status: 'success' | 'failed'
    /** फेल होने पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'locked'
    /** फेल होने पर शामिल। **/
    reason?: string
}
[inline-code-end]