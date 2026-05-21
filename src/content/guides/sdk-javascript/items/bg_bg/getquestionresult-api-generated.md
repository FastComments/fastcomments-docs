## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Отговор

Връща: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResult200Response.ts)

## Пример

[inline-code-attrs-start title = 'Пример за getQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-42';
const id: string = 'question-9f8b7c';
const includeComments: boolean | undefined = true; // пример за незадължителен параметър
const result: GetQuestionResult200Response = await getQuestionResult(tenantId, id);
console.log(result);
[inline-code-end]

---