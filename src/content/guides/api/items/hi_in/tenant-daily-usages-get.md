[api-resource-header-start name = 'TenantDailyUsage'; route = 'GET /api/v1/tenant-daily-usage'; creditsCost = 1; api-resource-header-end]

यह रूट किसी टेनेंट के उपयोग को वर्ष, माह और दिन के आधार पर खोजने की सुविधा देता है। अधिकतम 365 ऑब्जेक्ट लौटाए जा सकते हैं, और लागत प्रति 10 ऑब्जेक्ट पर 1 API क्रेडिट है।

रिस्पॉन्स ऑब्जेक्ट उनकी निर्माण तिथि के अनुसार क्रमबद्ध होते हैं (सबसे पुराना पहले)।

[inline-code-attrs-start title = 'TenantDailyUsage खोज cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/tenant-daily-usage?tenantId=demo&API_KEY=DEMO_API_SECRET&yearNumber=2022&monthNumber=2&dayNumber=10'
[inline-code-end]

[inline-code-attrs-start title = 'TenantDailyUsage रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageQueryParams {
    tenantId: string
    API_KEY: string
    /** UTC के आधार पर। **/
    yearNumber?: number
    /** शून्य-आधारित। UTC के आधार पर। **/
    monthNumber?: number
    /** एक-आधारित। UTC के आधार पर। **/
    dayNumber?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'TenantDailyUsage रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantDailyUsageResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found'
    /** विफलता पर शामिल। **/
    reason?: string
    tenantDailyUsages: TenantDailyUsage[]
}
[inline-code-end]