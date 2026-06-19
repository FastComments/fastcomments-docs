Trenutno prisotni gledalci strani: osebe, katerih websocket seja je trenutno naročena na stran.
Vrača anonCount + totalCount (naročniki na ravni sobe, vključno z anonimnimi gledalci, ki jih ne navajamo).

## Parametri

| Ime | Tip | Obvezno | Opis |
|------|------|----------|-------------|
| tenantId | string | Da |  |
| urlId | string | Da |  |
| afterName | string | Ne |  |
| afterUserId | string | Ne |  |

## Odgovor

Vrača: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOnlineResponse.ts)

## Primer

[inline-code-attrs-start title = 'Primer getOnlineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_8f3c2b7';
const urlId: string = 'article-2026-06-19-site-update';
const afterName: string = 'michael.hansen';
const afterUserId: string = 'user_00421';
const onlineUsers: PageUsersOnlineResponse = await getOnlineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]