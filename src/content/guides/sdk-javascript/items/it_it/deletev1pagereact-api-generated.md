## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |

## Risposta

Restituisce: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReact.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di deleteV1PageReact'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'acme-corp-tenant-01';
const urlId: string = 'post-5f2a8b3c9d';
const deletedReaction: CreateV1PageReact = await deleteV1PageReact(tenantId, urlId);
[inline-code-end]

---