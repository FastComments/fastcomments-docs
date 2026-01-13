[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Ten endpoint API umożliwia usunięcie flagi z komentarza dla konkretnego użytkownika.

Notes:

- To wywołanie musi być zawsze wykonane w kontekście użytkownika. The user can be a FastComments.com User, SSO User, or Tenant User.
- Po tym, jak komentarz zostanie automatycznie odrzucony (ukryty) - komentarz może zostać ponownie zatwierdzony tylko przez administratora lub moderatora. Usunięcie flagi nie spowoduje ponownego zatwierdzenia komentarza.

[inline-code-attrs-start title = 'Przykład cURL: cofnięcie flagi komentarza'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

W przypadku anonimowego zgłaszania musimy określić `anonUserId`. Może to być identyfikator reprezentujący sesję anonimową lub losowy UUID.

[inline-code-attrs-start title = 'Przykład cURL: cofnięcie flagi anonimowego komentarza'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struktura żądania cofnięcia flagi komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi cofnięcia flagi komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Dołączone w przypadku niepowodzenia. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Dołączone w przypadku niepowodzenia. **/
    reason?: string
}
[inline-code-end]