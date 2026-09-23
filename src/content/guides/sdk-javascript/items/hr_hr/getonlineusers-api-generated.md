Trenutno online gledatelji stranice: ljudi čija je websocket sesija pretplaćena na stranicu upravo sada.  
Vraća anonCount + totalCount (pretplatnici u cijeloj sobi, uključujući anonimne gledatelje koje ne nabrajamo).

## Parametri

| Naziv | Tip | Obavezno | Opis |
|------|------|----------|------|
| tenantId | string | Yes |  |
| urlId | string | Yes |  |
| afterName | string | No |  |
| afterUserId | string | No |  |

## Odgovor

Vraća: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## Primjer

[inline-code-attrs-start title = 'Primjer getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function fetchOnlineUsers() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "article-9876";
  const afterName: string | undefined = "john_doe";
  const afterUserId: string | undefined = "user_456";

  const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(
    tenantId,
    urlId,
    afterName,
    afterUserId
  );

  console.log(onlineUsers);
}

fetchOnlineUsers();
[inline-code-end]