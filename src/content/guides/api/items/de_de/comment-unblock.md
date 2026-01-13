[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, einen Benutzer zu entsperren, der einen bestimmten Kommentar geschrieben hat. Es unterstützt das Entsperren von Kommentaren, die von FastComments.com-Benutzern, SSO-Benutzern und Tenant-Benutzern geschrieben wurden.

Es unterstützt einen `commentIdsToCheck`-Body-Parameter, um zu prüfen, ob andere potenziell sichtbare Kommentare auf dem Client nach dieser Aktion blockiert/entsperrt werden sollten.

Hinweise:

- Dieser Aufruf muss immer im Kontext eines Benutzers erfolgen. Der Benutzer kann ein FastComments.com-Benutzer, SSO-Benutzer oder Tenant-Benutzer sein.
- Die `userId` in der Anfrage ist der Benutzer, der *das Entsperren durchführt*. Zum Beispiel: `Benutzer A` möchte `Benutzer B` entsperren. Übergeben Sie `userId=Benutzer A` und die Kommentar-ID, die `Benutzer B` geschrieben hat.
- Vollständig anonyme Kommentare (keine Benutzer-ID, keine E-Mail) können nicht blockiert werden und es wird ein Fehler zurückgegeben.

[inline-code-attrs-start title = 'Comment Un-Block cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Anonymes Comment Un-Block cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Comment Un-Block Anfrage-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment Un-Block Antwort-Struktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnBlockResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id' | 'comment-cannot-be-blocked'
    /** Included on failure. **/
    reason?: string
    /** If commentIdsToCheck is defined, entries in this map with true are still blocked. If false, you might want to un-hide the comments from the user so they don't have to refresh. **/
    commentStatuses?: Record<string, boolean>;
}
[inline-code-end]
