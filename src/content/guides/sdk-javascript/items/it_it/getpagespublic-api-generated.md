Elenca le pagine per un tenant. Utilizzato dal client desktop FChat per popolare la lista delle sue stanze.
Richiede che `enableFChat` sia true nella configurazione personalizzata risolta per ogni pagina.
Le pagine che richiedono SSO vengono filtrate in base ai gruppi di accesso dell'utente che effettua la richiesta.

## Parametri

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| cursor | string | No |  |
| limit | number | No |  |
| q | string | No |  |
| sortBy | PagesSortBy | No |  |
| hasComments | boolean | No |  |

## Risposta

Restituisce: [`GetPagesPublic200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetPagesPublic200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getPagesPublic'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3b2c';
const cursor: string = 'eyJwYWdlIjoiMTIwIn0';
const limit: number = 25;
const q: string = 'homepage hero';
const hasComments: boolean = true;

const response: GetPagesPublic200Response = await getPagesPublic(
  tenantId,
  cursor,
  limit,
  q,
  undefined,
  hasComments
);
[inline-code-end]

---