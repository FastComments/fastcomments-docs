## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Ответ

Возвращает: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример deleteQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-tenant-01';
const questionResultId: string = '6f1a2b3c-4d5e-6789-abcd-ef0123456789';
const deletedResult: FlagCommentPublic200Response = await deleteQuestionResult(tenantId, questionResultId);
console.log(deletedResult);
[inline-code-end]

---