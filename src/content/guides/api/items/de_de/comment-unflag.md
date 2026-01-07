[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-flag'; creditsCost = 1; api-resource-header-end]

Dieser API-Endpunkt bietet die Möglichkeit, die Markierung eines Kommentars für einen bestimmten Benutzer aufzuheben.

Hinweise:

- Dieser Aufruf muss immer im Kontext eines Benutzers erfolgen. Der Benutzer kann ein FastComments.com-Benutzer, SSO-Benutzer oder Tenant-Benutzer sein.
- Nachdem ein Kommentar automatisch nicht genehmigt (ausgeblendet) wurde - kann der Kommentar nur von einem Administrator oder Moderator wieder genehmigt werden. Das Aufheben der Markierung wird den Kommentar nicht wieder genehmigen.

[inline-code-attrs-start title = 'Kommentar Markierung-Aufheben cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

Für anonymes Markieren müssen wir eine `anonUserId` angeben. Dies kann eine ID sein, die die anonyme Sitzung repräsentiert, oder eine zufällige UUID.

[inline-code-attrs-start title = 'Anonymes Kommentar-Markieren cURL Beispiel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-flag?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]


[inline-code-attrs-start title = 'Kommentar Markierung-Aufheben Anfragestruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentFlagQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Kommentar Markierung-Aufheben Antwortstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]

interface CommentUnFlagResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'not-found' | 'missing-user-id' | 'missing-anon-user-id'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
