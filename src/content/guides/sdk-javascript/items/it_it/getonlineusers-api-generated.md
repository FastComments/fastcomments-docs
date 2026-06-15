Attualmente gli spettatori online di una pagina: persone la cui sessione websocket è iscritta alla pagina in questo momento.
Restituisce anonCount + totalCount (iscritti alla stanza in generale, inclusi spettatori anonimi che non elenchiamo).

## Parametri

| Nome | Tipo | Richiesto | Descrizione |
|------|------|----------|-------------|
| tenantId | string | Sì |  |
| urlId | string | Sì |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Risposta

Restituisce: [`GetOnlineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsers200Response.ts)

## Esempio

[inline-code-attrs-start title = 'Esempio di getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_14f9c3';
const urlId: string = 'article_20250615';
const afterName: string = 'marie.curie';
const afterUserId: string = 'u_92b7';
const result: GetOnlineUsers200Response = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---