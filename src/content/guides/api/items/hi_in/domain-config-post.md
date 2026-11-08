[api-resource-header-start name = 'DomainConfig'; route = 'POST /api/v1/domain-configs'; creditsCost = 1; api-resource-header-end]

यह API एंडपॉइंट डोमेन कॉन्फ़िगरेशन बनाने की क्षमता प्रदान करता है।

किसी डोमेन के लिए कॉन्फ़िगरेशन जोड़ने से वह डोमेन FastComments खाते के लिए अधिकृत हो जाता है।

इस API के सामान्य उपयोग के मामले प्रारंभिक सेटअप हैं, यदि कई डोमेन जोड़े जाने हों, या ईमेल भेजने के लिए कस्टम कॉन्फ़िगरेशन।

[inline-code-attrs-start title = 'DomainConfig POST cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/domain-configs?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
	"domain": "example.com",
	"emailFromName": "some from name",
	"emailFromEmail": "some@test.com",
	"logoSrc": "https://example.com/my-logo-big.png",
	"logoSrc100px": "https://example.com/my-logo-100px.png",
	"footerUnsubscribeURL": "http://example.com/unsubscribe-ui",
	"emailHeaders": {
		"List-Unsubscribe-Post": "List-Unsubscribe=One-Click",
		"List-Unsubscribe": "<https://example.com/opt-out/[userId]>"
	}
}'
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig POST अनुरोध संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface DomainConfigPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'DomainConfig POST प्रतिक्रिया संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface DomainConfigPostResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'empty-request' | 'internal' | 'invalid-input' | 'missing-domain' | 'configuration-exists-for-domain' | 'domain-too-long' | 'domain-invalid';  
    /** विफलता पर शामिल। **/
    reason?: string
    /** बनाया गया कॉन्फ़िगरेशन। **/
    configuration?: DomainConfig
}
[inline-code-end]