## Parametreler

| Ad | Tür | Gerekli | Açıklama |
|------|------|----------|-------------|
| tenantId | string | Evet |  |
| id | string | Evet |  |

## Yanıt

Döndürür: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResult200Response.ts)

## Örnek

[inline-code-attrs-start title = 'getQuestionResult Örneği'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-42';
const id: string = 'question-9f8b7c';
const includeComments: boolean | undefined = true; // isteğe bağlı parametre örneği
const result: GetQuestionResult200Response = await getQuestionResult(tenantId, id);
console.log(result);
[inline-code-end]

---