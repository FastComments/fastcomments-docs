## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| batchJobId | string | Nee |  |
| sso | string | Nee |  |

## Respons

Retourneert: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportStatusResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getApiExportStatus Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const statusByBatch: ModerationExportStatusResponse = await getApiExportStatus('export_20260615_84c2');
const statusBySso: ModerationExportStatusResponse = await getApiExportStatus(undefined, 'sso_84c2f1b2');
[inline-code-end]