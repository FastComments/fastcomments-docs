Obecnie oglądający stronę: osoby, których sesja websocket jest subskrybowana na tę stronę w tej chwili.
Zwraca anonCount + totalCount (subskrybenci całego pokoju, włącznie z anonimowymi widzami, których nie wyszczególniamy).

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Odpowiedź

Zwraca: [`GetOnlineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsers200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_14f9c3';
const urlId: string = 'article_20250615';
const afterName: string = 'marie.curie';
const afterUserId: string = 'u_92b7';
const result: GetOnlineUsers200Response = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---