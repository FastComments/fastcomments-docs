---
Aktualnie online oglądający stronę: osoby, których sesja websocket jest subskrybowana do tej strony w tym momencie.
Zwraca anonCount + totalCount (subskrybenci na poziomie pokoju, w tym anonimowi oglądający, których nie wymieniamy).

## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Response

Zwraca: [`GetOnlineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsersResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_98765";

  // Z opcjonalnymi parametrami paginacji
  const pagedResult: GetOnlineUsersResponse = await getOnlineUsers(
    tenantId,
    urlId,
    "alice_smith",
    "user_9"
  );

  // Bez opcjonalnych parametrów paginacji
  const fullResult: GetOnlineUsersResponse = await getOnlineUsers(tenantId, urlId);
}
[inline-code-end]

---