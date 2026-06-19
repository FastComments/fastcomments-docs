---
Wcześniejsi komentujący na stronie, którzy NIE są obecnie online. Posortowane według displayName.
Użyj tego po wyczerpaniu /users/online, aby wyświetlić sekcję "Członkowie".
Paginacja kursorowa po commenterName: serwer przeszukuje częściowy indeks {tenantId, urlId, commenterName}
indeks od afterName w przód za pomocą $gt, bez kosztu $skip.

## Parametry

| Name | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| afterName | string | Nie |  |
| afterUserId | string | Nie |  |

## Odpowiedź

Zwraca: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/PageUsersOfflineResponse.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant-9f4b2a6c';
const urlId: string = 'articles/product-launch-2025';

const offlinePageFirst: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId);

const afterName: string = 'samantha.r';
const afterUserId: string = 'user_7d3a21f9';
const offlinePageNext: PageUsersOfflineResponse = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---