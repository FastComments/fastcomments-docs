Kiracının toplu işleri (içeri/dışarı aktarmalar, sayfa geçişleri...), en yeniden eskiye doğru.

## Parametreler

| Ad | Tür | Zorunlu | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| type | string | Hayır |  |
| status | string | Hayır |  |
| skip | number | Hayır |  |

## Yanıt

Döndürür: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Örnek

[inline-code-attrs-start title = 'getBatchJobs Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Tüm toplu işleri al
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Başarısız sayfa geçiş işlerini al, ilk 10'unu atla
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]