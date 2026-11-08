[api-resource-header-start name = 'Page'; route = 'PATCH /api/v1/pages/:id'; creditsCost = 1; api-resource-header-end]

यह रूट एक सिंगल `Page` को अपडेट करने की सुविधा प्रदान करता है। संबंधित कमेंट्स भी अपडेट हो जाएंगे।

[inline-code-attrs-start title = 'Page अपडेट cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request PATCH \
  --url 'https://fastcomments.com/api/v1/pages/my-page-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"url": "https://example.com/some-page"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Page अपडेट रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Page अपडेट रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PagePatchResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id' | 'missing-id' | 'page-does-not-exist' | 'empty-request' | 'internal' | 'invalid-input' | 'invalid-title' | 'extra-params' | 'accessible-by-group-ids-not-array' | 'too-many-group-ids' | 'group-id-too-large'
    /** विफलता पर शामिल। **/
    reason?: string
    user?: Page; // सफलता पर हम पूरा अपडेट किया गया पेज वापस करते हैं।
}
[inline-code-end]

#### नोट

Page ऑब्जेक्ट के कुछ पैरामीटर स्वचालित रूप से अपडेट हो जाते हैं। ये counts और title एट्रिब्यूट्स हैं। Counts को API के माध्यम से अपडेट नहीं किया जा सकता क्योंकि वे गणना किए गए मान हैं। पेज का `title` API के माध्यम से सेट किया जा सकता है, लेकिन यदि उसी `urlId` वाले पेज पर कमेंट विजेट का उपयोग किया जाता है और पेज का टाइटल अलग है, तो यह ओवरराइट हो जाएगा।