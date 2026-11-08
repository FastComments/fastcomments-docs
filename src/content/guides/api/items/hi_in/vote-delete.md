[api-resource-header-start name = 'Vote'; route = 'DELETE /api/v1/votes/:id'; creditsCost = 1; api-resource-header-end]

यह रूट एकल `Vote` को डिलीट करने की सुविधा प्रदान करता है।

[inline-code-attrs-start title = 'Vote डिलीट cURL उदाहरण'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/votes/some-id?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Vote डिलीट रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Vote डिलीट रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VoteDeleteResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]

नोट्स:

- यह API टेनेंट-स्तरीय सेटिंग्स का पालन करता है। उदाहरण के लिए, यदि आप किसी दिए गए पेज के लिए वोटिंग अक्षम करते हैं और API के माध्यम से वोट बनाने का प्रयास करते हैं, तो यह `voting-disabled` त्रुटि कोड के साथ विफल हो जाएगा।
- यह API डिफ़ॉल्ट रूप से लाइव है।
- यह API संबंधित `Comment` के `votes` को अपडेट करेगा।