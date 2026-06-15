Byli komentujący na stronie, którzy NIE są obecnie online. Posortowane według displayName.
Użyj tego po wyczerpaniu /users/online, aby wyświetlić sekcję "Członkowie".
Paginacja kursorowa po commenterName: serwer przechodzi częściowy indeks {tenantId, urlId, commenterName} od afterName w przód za pomocą $gt, bez kosztu $skip.

## Parameters

| Nazwa | Typ | Wymagane | Opis |
|------|------|----------|-------------|
| tenantId | string | Tak |  |
| urlId | string | Tak |  |
| afterName | string | Nie |  |
| afterUserId | string | Nie |  |

## Odpowiedź

Zwraca: [`GetOfflineUsers200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetOfflineUsers200Response.ts)

## Przykład

[inline-code-attrs-start title = 'Przykład getOfflineUsers'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_prod_001';
const urlId: string = 'article-2026-06-15-how-ai-impacts';
const afterName: string = 'michael.smith';
const afterUserId: string = 'user_72b9';

const response: GetOfflineUsers200Response = await getOfflineUsers(tenantId, urlId, afterName, afterUserId);
[inline-code-end]

---