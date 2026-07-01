## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| batchJobId | string | No |  |
| tenantId | string | No |  |
| sso | string | No |  |

## Відповідь

Повертає: [`GetApiExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiExportStatusResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getApiExportStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoExportStatus() {
    const batchJobId: string = "exportBatch-20231101-001";
    const tenantId: string = "tenant-abc123";
    const ssoToken: string = "sso-xyz789";

    const fullStatus: GetApiExportStatusResponse = await getApiExportStatus(batchJobId, tenantId, ssoToken);
    const simpleStatus: GetApiExportStatusResponse = await getApiExportStatus(batchJobId);
    console.log(fullStatus, simpleStatus);
}
[inline-code-end]