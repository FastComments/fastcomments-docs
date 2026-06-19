Osoby obecnie online na stronie: osoby, których sesja websocket jest aktualnie subskrybowana na tę stronę.
Zwraca anonCount + totalCount (subskrybenci w całym pokoju, w tym anonimowi widzowie, których nie wymieniamy).

## Parametry

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| afterName | string | Nie |  |
| afterUserId | string | Nie |  |

## Odpowiedź

Zwraca: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład użycia getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3c2b7';
const urlId: string = 'article-2026-06-19-site-update';
const afterName: string = 'michael.hansen';
const afterUserId: string = 'user_00421';
const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---