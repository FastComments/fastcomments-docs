## Parameter

| Name       | Typ    | Erforderlich | Beschreibung |
|------------|--------|--------------|--------------|
| batchJobId | string | Nein         |  |
| tenantId   | string | Nein         |  |
| sso        | string | Nein         |  |

## Antwort

Rückgabe: [`GetApiExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetApiExportStatusResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getApiExportStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---