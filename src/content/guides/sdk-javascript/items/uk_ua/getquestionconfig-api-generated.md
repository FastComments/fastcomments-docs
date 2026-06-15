## Параметри

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| id | string | Так |  |

## Відповідь

Повертає: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfig200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-47";
const questionId: string = "q-4f2b9a";
const includeDrafts: boolean | undefined = undefined; // заповнювач для необов'язкового параметра
const result: GetQuestionConfig200Response = await getQuestionConfig(tenantId, questionId);
console.log(result);
[inline-code-end]