Trenutni gledatelji na stranici: osobe čija je websocket sesija trenutno pretplaćena na stranicu. Vraća anonCount + totalCount (pretplatnici po cijeloj sobi, uključujući anonimne gledatelje koje ne izlistavamo).

## Parameters

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| afterName | string | Ne |  |
| afterUserId | string | Ne |  |

## Odgovor

Vraća: [`GetOnlineUsersResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOnlineUsersResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function demoOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_98765";

  // Sa opcionalnim parametrima paginacije
  const pagedResult: GetOnlineUsersResponse = await getOnlineUsers(
    tenantId,
    urlId,
    "alice_smith",
    "user_9"
  );

  // Bez opcionalnih parametara paginacije
  const fullResult: GetOnlineUsersResponse = await getOnlineUsers(tenantId, urlId);
}
[inline-code-end]