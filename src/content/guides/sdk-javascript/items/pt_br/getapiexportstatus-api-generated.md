## Parâmetros

| Nome | Tipo | Obrigatório | Descrição |
|------|------|-------------|-----------|
| tenantId | string | Sim |  |
| batchJobId | string | Não |  |
| sso | string | Não |  |

## Resposta

Retorna: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportStatusResponse.ts)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getApiExportStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function run() {
  const tenantId: string = "tenant_12345";
  const batchJobId: string = "job_98765";
  const ssoToken: string = "sso_abcde12345";

  const statusOnlyTenant: ModerationExportStatusResponse = await getApiExportStatus(tenantId);
  const statusWithBatch: ModerationExportStatusResponse = await getApiExportStatus(tenantId, batchJobId);
  const statusFull: ModerationExportStatusResponse = await getApiExportStatus(tenantId, batchJobId, ssoToken);
}
run();
[inline-code-end]

---