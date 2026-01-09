[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Ten punkt końcowy API umożliwia odblokowanie użytkownika, który napisał dany komentarz. Obsługuje odblokowywanie komentarzy napisanych przez użytkowników FastComments.com, użytkowników SSO oraz użytkowników najemcy.

Obsługuje parametr w body `commentIdsToCheck`, aby sprawdzić, czy po wykonaniu tej akcji inne potencjalnie widoczne komentarze po stronie klienta powinny zostać zablokowane/odblokowane.

Uwagi:

- To wywołanie musi być zawsze wykonane w kontekście użytkownika. Użytkownik może być użytkownikiem FastComments.com, użytkownikiem SSO lub użytkownikiem najemcy.
- Pole `userId` w żądaniu to użytkownik, który *wykonuje odblokowanie*. Na przykład: `User A` chce odblokować `User B`. Przekaż `userId=User A` oraz identyfikator komentarza, który napisał `User B`.
- Całkowicie anonimowe komentarze (bez id użytkownika, bez adresu e-mail) nie mogą być zablokowane i zostanie zwrócony błąd.

[inline-code-attrs-start title = 'Przykład cURL — odblokowanie komentarza'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Przykład cURL — odblokowanie anonimowego komentarza'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura żądania odblokowania komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi odblokowania komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
    /** Jeśli commentIdsToCheck jest zdefiniowane, wpisy w tej mapie z wartością true są nadal zablokowane. Jeśli false, możesz chcieć przywrócić widoczność komentarzy dla użytkownika, aby nie musiał odświeżać strony. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]

---