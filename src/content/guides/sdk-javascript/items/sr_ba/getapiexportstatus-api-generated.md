## Parametri

| Naziv | Тип | Обавезно | Опис |
|------|------|----------|-------------|
| batchJobId | string | Ne |  |
| sso | string | Ne |  |

## Одговор

Враћа: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportStatusResponse.ts)

## Примјер

[inline-code-attrs-start title = 'Primjer getApiExportStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const statusByBatch: ModerationExportStatusResponse = await getApiExportStatus('export_20260615_84c2');
const statusBySso: ModerationExportStatusResponse = await getApiExportStatus(undefined, 'sso_84c2f1b2');
[inline-code-end]

---