## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| batchJobId | string | Nej |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportStatusResponse.ts)

## Eksempel

[inline-code-attrs-start title = 'getApiExportStatus Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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