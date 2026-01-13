## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`GetQuestionResult200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetQuestionResult200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "3fa85f64-5717-4562-b3fc-2c963f66afa6";
const questionId: string = "question_72f1b9c3a4";
const result: GetQuestionResult200Response = await getQuestionResult(tenantId, questionId);
console.log(result);
[inline-code-end]

---