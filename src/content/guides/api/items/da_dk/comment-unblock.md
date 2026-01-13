[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/un-block'; creditsCost = 1; api-resource-header-end]

Denne API-endpoint giver mulighed for at afblokere en bruger, der har skrevet en given kommentar. Det understøtter afblokering fra kommentarer skrevet af FastComments.com-brugere, SSO-brugere og Tenant-brugere.

Det understøtter en `commentIdsToCheck` body-parameter til at kontrollere, om andre potentielt synlige kommentarer på klienten skal blokeres/afblokeres efter denne handling udføres.

Bemærkninger:

- Dette kald skal altid foretages i konteksten af en bruger. Brugeren kan være en FastComments.com-bruger, SSO-bruger eller Tenant-bruger.
- `userId` i anmodningen er brugeren, der *udfører afblokeringen*. For eksempel: `Bruger A` vil afblokere `Bruger B`. Angiv `userId=Bruger A` og kommentar-id'et som `Bruger B` skrev.
- Fuldstændig anonyme kommentarer (ingen bruger-id, ingen e-mail) kan ikke blokeres, og en fejl vil blive returneret.

[inline-code-attrs-start title = 'Comment Un-Block cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Anonymous Comment Un-Block cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/un-block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Comment Un-Block Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentUnBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment Un-Block Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
