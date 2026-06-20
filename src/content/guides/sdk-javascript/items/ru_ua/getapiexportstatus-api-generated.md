---
## Параметры

| Имя | Тип | Требуется | Описание |
|------|------|----------|-------------|
| batchJobId | string | Нет |  |
| sso | string | Нет |  |

## Ответ

Возвращает: [`ModerationExportStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/ModerationExportStatusResponse.ts)

## Пример

[inline-code-attrs-start title = 'Пример getApiExportStatus'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const statusByBatch: ModerationExportStatusResponse = await getApiExportStatus('export_20260615_84c2');
const statusBySso: ModerationExportStatusResponse = await getApiExportStatus(undefined, 'sso_84c2f1b2');
[inline-code-end]

---