## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| id | string | Sì |  |

## Risposta

Restituisce: [`FlagCommentPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/FlagCommentPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteQuestionResult'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-01";
const id: string = "qres_9f8b7c3a";
const response: FlagCommentPublic200Response = await deleteQuestionResult(tenantId, id);
const optionalResponse: FlagCommentPublic200Response | undefined = response;
[inline-code-end]

---