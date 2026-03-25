## Параметри

| Назва | Тип | Обов'язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |

## Відповідь

Повертає: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfig200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-92';
const id: string = 'question-2026-07-42';
const response: GetQuestionConfig200Response = await getQuestionConfig(tenantId, id);

function summarize(cfg: GetQuestionConfig200Response, includeDetails?: boolean): string {
  return includeDetails ? 'Question config (detailed)' : 'Question config (summary)';
}

const summary: string = summarize(response);
[inline-code-end]

---