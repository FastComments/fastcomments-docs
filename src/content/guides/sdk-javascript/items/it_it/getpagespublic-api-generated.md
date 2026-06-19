Elenca le pagine per un tenant. Utilizzato dal client desktop FChat per popolare la sua lista di stanze.
Richiede `enableFChat` to be true on the resolved custom config for each page.
Le pagine che richiedono SSO vengono filtrate in base all'accesso di gruppo dell'utente richiedente.

## Parametri

| Nome | Tipo | Obbligatorio | Descrizione |
|------|------|--------------|-------------|
| tenantId | string | Sì |  |
| cursor | string | No |  |
| limit | number | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | boolean | No |  |

## Risposta

Restituisce: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPublicPagesResponse.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_7f9b2c";
const cursor: string = "cursor_0001a2b3";
const limit: number = 25;
const q: string = "product page";
const hasComments: boolean = true;
const response: GetPublicPagesResponse = await getPagesPublic(tenantId, cursor, limit, q, undefined, hasComments);
[inline-code-end]

---