## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| title | string | No |  |

## Risposta

Restituisce: [`CreateV1PageReactResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateV1PageReactResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio createV1PageReact'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const urlId: string = "article-2024-06-improvements";
const title: string = "FastComments API Integration Guide";

const responseWithTitle: CreateV1PageReactResponse = await createV1PageReact(tenantId, urlId, title);
const responseWithoutTitle: CreateV1PageReactResponse = await createV1PageReact(tenantId, urlId);
[inline-code-end]