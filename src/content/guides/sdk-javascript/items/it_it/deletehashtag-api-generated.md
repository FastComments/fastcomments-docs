## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tag | string | Sì |  |
| tenantId | string | No |  |
| deleteHashTagRequestBody | DeleteHashTagRequestBody | No |  |

## Risposta

Restituisce: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIEmptyResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteHashTag'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tag: string = "release-notes";
const tenantId: string = "tenant_7b2f9c";
const deleteHashTagRequestBody: DeleteHashTagRequestBody = { removedBy: "ops@acme-corp.com", force: true };
const result: APIEmptyResponse = await deleteHashTag(tag, tenantId, deleteHashTagRequestBody);
[inline-code-end]

---