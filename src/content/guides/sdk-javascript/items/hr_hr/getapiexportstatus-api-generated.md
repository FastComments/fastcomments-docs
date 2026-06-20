## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|-------------|
| batchJobId | string | Ne |  |
| sso | string | Ne |  |

## Odgovor

Vraća: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportStatusResponse.ts)

## Primjer

[inline-code-attrs-start title = 'getApiExportStatus Primjer'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const statusByBatch: ModerationExportStatusResponse = await getApiExportStatus('export_20260615_84c2');
const statusBySso: ModerationExportStatusResponse = await getApiExportStatus(undefined, 'sso_84c2f1b2');
[inline-code-end]