[api-resource-header-start name = 'BatchJob'; route = 'GET /api/v1/batch-jobs'; creditsCost = 1; api-resource-header-end]

यह रूट टेनेंट के सभी प्रकार के बैच जॉब लौटाता है, नवीनतम पहले, प्रति पेज 100। सूची को सीमित करने के लिए `type` और `status` का उपयोग करें, और पेजिंग के लिए `skip` का उपयोग करें।

[inline-code-attrs-start title = 'BatchJob सूची cURL उदाहरण'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/batch-jobs?tenantId=demo&API_KEY=DEMO_API_SECRET&type=migrate-pages&status=running'
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob सूची रिक्वेस्ट संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsQueryParams {
    tenantId: string
    API_KEY: string
    /** केवल इस प्रकार के जॉब, उदाहरण के लिए migrate-pages। **/
    type?: string
    status?: 'requested' | 'running' | 'success' | 'failed'
    /** पेजिंग के लिए कितने जॉब छोड़ने हैं। एक गैर-ऋणात्मक पूर्णांक। **/
    skip?: number
}
[inline-code-end]

[inline-code-attrs-start title = 'BatchJob सूची रिस्पॉन्स संरचना'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface BatchJobsResponse {
    status: 'success' | 'failed'
    /** विफलता पर शामिल। **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'invalid-skip' | 'internal'
    /** विफलता पर शामिल। **/
    reason?: string
    /** नवीनतम पहले, अधिकतम 100। **/
    batchJobs?: BatchJob[]
}
[inline-code-end]