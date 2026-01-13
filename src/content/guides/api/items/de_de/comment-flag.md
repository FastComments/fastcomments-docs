[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/flag'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, einen Kommentar für einen bestimmten Benutzer zu melden.

Hinweise:

- Dieser Aufruf muss immer im Kontext eines Benutzers erfolgen. Der Benutzer kann ein FastComments.com-Benutzer, SSO-Benutzer oder Tenant-Benutzer sein.
- Wenn ein Flag-to-Hide-Schwellenwert festgelegt ist, wird der Kommentar automatisch live ausgeblendet, nachdem er die definierte Anzahl von Malen gemeldet wurde.
- Nachdem er automatisch nicht genehmigt (ausgeblendet) wurde - kann der Kommentar nur von einem Administrator oder Moderator wieder genehmigt werden. Das Aufheben der Meldung wird den Kommentar nicht wieder genehmigen.

[inline-code-attrs-start title = 'Comment Flag cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Für anonymes Melden müssen wir eine `anonUserId` angeben. Dies kann eine ID sein, die die anonyme Sitzung repräsentiert, oder eine zufällige UUID.
Dies ermöglicht uns, das Melden und Aufheben der Meldung von Kommentaren zu unterstützen, auch wenn ein Benutzer nicht eingeloggt ist. Auf diese Weise kann der Kommentar als
gemeldet markiert werden, wenn Kommentare mit derselben `anonUserId` abgerufen werden.

[inline-code-attrs-start title = 'Anonymes Comment Flag cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Comment Flag Anfrage-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment Flag Antwort-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
