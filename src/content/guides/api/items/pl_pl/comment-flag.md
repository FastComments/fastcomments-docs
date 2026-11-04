[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Ten punkt końcowy API umożliwia zgłoszenie (oflagowanie) komentarza dla konkretnego użytkownika.

Uwagi:

- To wywołanie zawsze musi być wykonane w kontekście użytkownika. Użytkownikiem może być użytkownik FastComments.com, użytkownik SSO lub użytkownik dzierżawy (Tenant User).
- Jeśli ustawiony jest próg flag-do-ukrycia, komentarz zostanie automatycznie ukryty na żywo po zgłoszeniu go zdefiniowaną liczbę razy.
- Po automatycznym cofnięciu zatwierdzenia (ukryciu) - komentarz może zostać ponownie zatwierdzony tylko przez administratora lub moderatora. Usunięcie flagi nie spowoduje ponownego zatwierdzenia komentarza.

[inline-code-attrs-start title = 'Przykład cURL zgłoszenia komentarza'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

W przypadku zgłoszeń anonimowych musimy podać `anonUserId`. Może to być identyfikator reprezentujący sesję anonimową lub losowy UUID.
Pozwala to na obsługę zgłaszania i cofania zgłoszeń komentarzy nawet wtedy, gdy użytkownik nie jest zalogowany. Dzięki temu komentarz może zostać oznaczony jako zgłoszony, gdy komentarze są pobierane z tym samym `anonUserId`.

[inline-code-attrs-start title = 'Przykład cURL anonimowego zgłoszenia komentarza'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Struktura żądania zgłoszenia komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odpowiedzi zgłoszenia komentarza'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentFlagResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Included on failure. **/
    reason?: string
    /** Was the comment un-approved (hidden) due to being flagged too many times? **/
    wasUnapproved?: boolean;
}
[inline-code-end]