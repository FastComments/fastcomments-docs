## Параметры

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Да |  |
| id | string | Да |  |

## Ответ

Возвращает: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResult200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример getQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "3fa85f64-5717-4562-b3fc-2c963f66afa6";
const questionId: string = "question_72f1b9c3a4";
const result: GetQuestionResult200Response = await getQuestionResult(tenantId, questionId);
console.log(result);
[inline-code-end]

---