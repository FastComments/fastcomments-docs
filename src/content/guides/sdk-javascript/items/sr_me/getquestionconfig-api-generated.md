## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| id | string | Yes |  |

## Odgovor

Vraća: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfig200Response.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-47";
const questionId: string = "q-4f2b9a";
const includeDrafts: boolean | undefined = undefined; // rezervisano za neobavezni parametar
const result: GetQuestionConfig200Response = await getQuestionConfig(tenantId, questionId);
console.log(result);
[inline-code-end]