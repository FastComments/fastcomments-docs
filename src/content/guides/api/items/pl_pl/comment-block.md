[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Ten endpoint API umożliwia zablokowanie użytkownika, który napisał dany komentarz. Obsługuje blokowanie komentarzy napisanych przez użytkowników FastComments.com, użytkowników SSO oraz użytkowników najemcy.

Obsługuje parametr w treści żądania `commentIdsToCheck`, aby sprawdzić, czy inne potencjalnie widoczne komentarze po stronie klienta powinny zostać zablokowane/odblokowane po wykonaniu tej akcji.

Notes:

- To wywołanie musi być zawsze wykonane w kontekście użytkownika. Użytkownikiem może być użytkownik FastComments.com, użytkownik SSO lub użytkownik najemcy.
- The `userId` in the request is the user that is *doing the blocking*. For example: `User A` wants to Block `User B`. Pass `userId=User A` and the comment id that `User B` wrote.
- Całkowicie anonimowe komentarze (bez identyfikatora użytkownika, bez adresu e-mail) nie mogą być zablokowane i zostanie zwrócony błąd.

[inline-code-attrs-start title = 'Przykład cURL blokowania komentarza'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

For anonymous blocking, we must specify an `anonUserId`. This can be an ID that represents the anonymous session, or a random UUID.
This allows us to support blocking comments even if a user is not logged in by fetching the comments with the same `anonUserId`.

[inline-code-attrs-start title = 'Przykład cURL blokowania anonimowego komentarza'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania blokowania komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi blokowania komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentBlockResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    /** Jeśli commentIdsToCheck jest zdefiniowane, wpisy w tej mapie z wartością true również są zablokowane. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---