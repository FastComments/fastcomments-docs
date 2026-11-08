[api-resource-header-start name = 'Tenant'; route = 'GET /api/v1/tenants'; creditsCost = 1; api-resource-header-end]

यह API उन tenants को लौटाती है जो आपके tenant द्वारा प्रबंधित हैं।

पेजिनेशन `skip` क्वेरी पैरामीटर द्वारा प्रदान की जाती है। Tenants `100` के पेजों में लौटाए जाते हैं, जो `signUpDate` और `id` के अनुसार क्रमबद्ध होते हैं।

लागत लौटाए गए tenants की संख्या पर आधारित है, जो `1 credit per 10` लौटाए गए tenants के अनुसार है।

[inline-code-attrs-start title = 'Tenant cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET'
[inline-code-end]

आप `Tenant` ऑब्जेक्ट्स पर `meta` पैरामीटर परिभाषित कर सकते हैं और मेल खाते tenants के लिए क्वेरी कर सकते हैं। उदाहरण के लिए, कुंजी `someKey` और meta मान `some-value` के लिए, हम इस कुंजी/मान जोड़ी के साथ एक JSON ऑब्जेक्ट बना सकते हैं और फिर इसे फ़िल्टर करने के लिए एक क्वेरी पैरामीटर के रूप में URI एनकोड कर सकते हैं:

[inline-code-attrs-start title = 'Meta द्वारा Tenant क्वेरी cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenants?tenantId=demo&skip=0&API_KEY=DEMO_API_SECRET&meta=%7B%22someKey%22%3A%22some-value%22%7D'
[inline-code-end]

[inline-code-attrs-start title = 'Tenant अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsRequestQueryParams {
    tenantId: string
    API_KEY: string
    /** पेजिनेशन के लिए छोड़े जाने वाले tenants की संख्या। **/
    skip?: number
    /** meta पैरामीटर द्वारा फ़िल्टर करें। **/
    meta?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Tenant प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantsResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key'
    /** विफलता पर शामिल। **/
    reason?: string
    tenants?: Tenant[]
}
[inline-code-end]