[api-resource-header-start name = 'SSOUser'; route = 'POST /api/v1/sso-users'; creditsCost = 1; api-resource-header-end]

यह रूट एक सिंगल SSO यूज़र के निर्माण की सुविधा देता है।

समान ID वाले दो यूज़र बनाने की कोशिश करने पर त्रुटि (error) मिलेगी।

[inline-code-attrs-start title = 'SSOUser निर्माण cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/sso-users?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"id": "my-user-id",
	"username": "fordperfect",
	"displayName": "Ford Perfect",
	"email": "fordperfect@galaxy.com",
    "groupIds": ["some-optional-group-id"]
}'
[inline-code-end]

इस उदाहरण में हम एक्सेस कंट्रोल के लिए `groupIds` निर्दिष्ट करते हैं, लेकिन यह वैकल्पिक (optional) है।

[inline-code-attrs-start title = 'SSOUser निर्माण रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'SSOUser निर्माण रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUserPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'invalid-input' | 'missing-id' | 'user-exists'
    /** Included on failure. **/
    reason?: string
    user?: SSOUser; // We return the created user on success.
}
[inline-code-end]

#### इंटीग्रेशन नोट

API द्वारा पास किया गया डेटा केवल एक अलग SSO User HMAC पेलोड पास करके ओवरराइड किया जा सकता है। उदाहरण के लिए, यदि
आप API के माध्यम से एक username सेट करते हैं, लेकिन फिर पेज लोड पर SSO फ्लो के माध्यम से एक अलग username पास करते हैं, तो हम
उनका username स्वचालित रूप से अपडेट कर देंगे।

हम इस फ्लो में यूज़र पैरामीटर अपडेट नहीं करेंगे जब तक कि आप उन्हें स्पष्ट रूप से निर्दिष्ट न करें या उन्हें null (undefined नहीं) पर सेट न करें।