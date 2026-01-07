[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, einen Benutzer zu blockieren, der einen bestimmten Kommentar geschrieben hat. Es unterstützt das Blockieren von Kommentaren, die von FastComments.com-Benutzern, SSO-Benutzern und Tenant-Benutzern geschrieben wurden.

Es unterstützt einen `commentIdsToCheck`-Body-Parameter, um zu prüfen, ob andere potenziell sichtbare Kommentare auf dem Client nach dieser Aktion blockiert/entsperrt werden sollten.

Hinweise:

- Dieser Aufruf muss immer im Kontext eines Benutzers erfolgen. Der Benutzer kann ein FastComments.com-Benutzer, SSO-Benutzer oder Tenant-Benutzer sein.
- Die `userId` in der Anfrage ist der Benutzer, der *das Blockieren durchführt*. Zum Beispiel: `Benutzer A` möchte `Benutzer B` blockieren. Übergeben Sie `userId=Benutzer A` und die Kommentar-ID, die `Benutzer B` geschrieben hat.
- Vollständig anonyme Kommentare (keine Benutzer-ID, keine E-Mail) können nicht blockiert werden und es wird ein Fehler zurückgegeben.

[inline-code-attrs-start title = 'Comment Block cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Für anonymes Blockieren müssen wir eine `anonUserId` angeben. Dies kann eine ID sein, die die anonyme Sitzung repräsentiert, oder eine zufällige UUID.
Dies ermöglicht uns, das Blockieren von Kommentaren zu unterstützen, auch wenn ein Benutzer nicht eingeloggt ist, indem die Kommentare mit derselben `anonUserId` abgerufen werden.

[inline-code-attrs-start title = 'Anonymes Comment Block cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Comment Block Anfrage-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment Block Antwort-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
