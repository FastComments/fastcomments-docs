## Параметри

| Назва | Тип | Обов'язковий | Опис |
|------|------|----------|-------------|
| batchJobId | string | Ні |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportStatusResponse.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getApiExportStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const statusByBatch: ModerationExportStatusResponse = await getApiExportStatus('export_20260615_84c2');
const statusBySso: ModerationExportStatusResponse = await getApiExportStatus(undefined, 'sso_84c2f1b2');
[inline-code-end]

---