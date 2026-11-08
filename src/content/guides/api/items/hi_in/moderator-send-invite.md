[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

यह रूट एक सिंगल `Moderator` को आमंत्रित करने की सुविधा प्रदान करता है।

`Moderator` को आमंत्रण ईमेल भेजने के लिए निम्नलिखित प्रतिबंध मौजूद हैं:
- `Moderator` पहले से मौजूद होना चाहिए।
- `fromName` `100 characters` से अधिक लंबा नहीं हो सकता।

**नोट्स:**
- यदि दिए गए ईमेल वाला उपयोगकर्ता पहले से मौजूद है, तो उन्हें आपके टेनेंट की टिप्पणियों को मॉडरेट करने के लिए आमंत्रित किया जाएगा।
- यदि दिए गए ईमेल वाला उपयोगकर्ता **मौजूद नहीं है**, तो आमंत्रण लिंक उन्हें अपना खाता बनाने की प्रक्रिया से गुज़रने के लिए मार्गदर्शन करेगा।
- आमंत्रण `30 days` के बाद समाप्त हो जाएगा।

हम एक उपयोगकर्ता के लिए `Moderator` बना सकते हैं जिसका हमें केवल ईमेल पता है:

[inline-code-attrs-start title = 'मॉडरेटर आमंत्रण cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

इससे `Bob at TenantName is inviting you to be a moderator...` जैसा एक ईमेल भेजा जाएगा।

[inline-code-attrs-start title = 'मॉडरेटर आमंत्रण अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** उपयोगकर्ता को भेजा गया ईमेल इस नाम से भेजा गया दिखाई देगा। **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'मॉडरेटर आमंत्रण प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** विफलता पर शामिल। **/
    reason?: string
}
[inline-code-end]