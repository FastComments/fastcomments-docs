टेनेंट के बैच जॉब (इंपोर्ट, एक्सपोर्ट, पेज माइग्रेशन...), नवीनतम पहले।

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| type | string | नहीं |  |
| status | string | नहीं |  |
| skip | number | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'getBatchJobs उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// सभी बैच जॉब प्राप्त करें
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// विफल पेज-माइग्रेशन जॉब प्राप्त करें, पहले 10 छोड़कर
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]