## Parameter

| Name | Typ | Erforderlich | Beschreibung |
|------|------|----------|-------------|
| batchJobId | string | Nein |  |
| sso | string | Nein |  |

## Antwort

Gibt zurück: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportStatusResponse.ts)

## Beispiel

[inline-code-attrs-start title = 'getApiExportStatus Beispiel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const statusByBatch: ModerationExportStatusResponse = await getApiExportStatus('export_20260615_84c2');
const statusBySso: ModerationExportStatusResponse = await getApiExportStatus(undefined, 'sso_84c2f1b2');
[inline-code-end]

---