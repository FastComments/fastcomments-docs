[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Ten punkt końcowy API umożliwia zablokowanie użytkownika, który napisał dany komentarz. Obsługuje blokowanie komentarzy napisanych przez użytkowników FastComments.com, użytkowników SSO oraz użytkowników Tenant.

Obsługuje parametr body `commentIdsToCheck`, aby sprawdzić, czy inne potencjalnie widoczne komentarze po stronie klienta powinny zostać zablokowane/odblokowane po wykonaniu tej akcji.

Uwagi:

- To wywołanie zawsze musi być wykonane w kontekście użytkownika. Użytkownikiem może być użytkownik FastComments.com, użytkownik SSO lub użytkownik Tenant.
- `userId` w żądaniu to użytkownik, który *dokonuje blokowania*. Na przykład: `User A` chce zablokować `User B`. Przekaż `userId=User A` oraz identyfikator komentarza napisanego przez `User B`.
- Całkowicie anonimowe komentarze (bez identyfikatora użytkownika, bez adresu e-mail) nie mogą być blokowane — zostanie zwrócony błąd.

[inline-code-attrs-start title = 'Przykład cURL blokowania komentarza'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

W przypadku blokowania anonimowego musimy określić `anonUserId`. Może to być identyfikator reprezentujący sesję anonimową lub losowy UUID.
Pozwala to na obsługę blokowania komentarzy nawet wtedy, gdy użytkownik nie jest zalogowany, poprzez pobranie komentarzy z tym samym `anonUserId`.

[inline-code-attrs-start title = 'Przykład cURL anonimowego blokowania komentarza'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Included on failure. **/
    reason?: string
    /** If commentIdsToCheck is defined, entries in this map with true are also blocked. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]