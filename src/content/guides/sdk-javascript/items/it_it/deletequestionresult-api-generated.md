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
const tenantIdEnv: string | undefined = process.env.FASTCOMMENTS_TENANT_ID;
const tenantId: string = tenantIdEnv ?? 'tenant_78b3f2';
const id: string = 'qres-9f2a3b1c';
const response: FlagCommentPublic200Response = await deleteQuestionResult(tenantId, id);
[inline-code-end]

---