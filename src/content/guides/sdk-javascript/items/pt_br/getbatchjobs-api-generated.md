Os trabalhos em lote (batch jobs) do tenant (importações, exportações, migrações de páginas...), do mais recente para o mais antigo.

## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|----------|-------------|
| tenantId | string | Sim |  |
| type | string | Não |  |
| status | string | Não |  |
| skip | number | Não |  |

## Resposta

Retorna: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetBatchJobsAPIResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getBatchJobs'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
declare function getBatchJobs(tenantId: string, type?: string, status?: string, skip?: number): Promise<GetBatchJobsAPIResponse>;

const tenantId: string = '5ced7a2b4b0af90017623a6b';

// Obter todos os batch jobs
const allJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId);

// Obter jobs de migração de páginas com falha, pulando os primeiros 10
const failedJobs: GetBatchJobsAPIResponse = await getBatchJobs(tenantId, 'page-migration', 'failed', 10);

console.log(`Found ${allJobs.status} response with ${allJobs.batchJobs?.length ?? 0} jobs`);
[inline-code-end]