## Parameters

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| id | string | Da |  |

## Odgovor

Vrača: [`GetQuestionConfig200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionConfig200Response.ts)

## Primer

[inline-code-attrs-start title = 'Primer getQuestionConfig'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-47";
const questionId: string = "q-4f2b9a";
const includeDrafts: boolean | undefined = undefined; // rezervirano mesto za neobvezni parameter
const result: GetQuestionConfig200Response = await getQuestionConfig(tenantId, questionId);
console.log(result);
[inline-code-end]

---