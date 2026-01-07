[api-resource-header-start name = 'Comment'; route = 'POST /api/v1/comments/:id/block'; creditsCost = 1; api-resource-header-end]

Denne API-endpoint giver mulighed for at blokere en bruger, der har skrevet en given kommentar. Det understøtter blokering fra kommentarer skrevet af FastComments.com-brugere, SSO-brugere og Tenant-brugere.

Det understøtter en `commentIdsToCheck` body-parameter til at kontrollere, om andre potentielt synlige kommentarer på klienten skal blokeres/afblokeres efter denne handling udføres.

Bemærkninger:

- Dette kald skal altid foretages i konteksten af en bruger. Brugeren kan være en FastComments.com-bruger, SSO-bruger eller Tenant-bruger.
- `userId` i anmodningen er brugeren, der *udfører blokeringen*. For eksempel: `Bruger A` vil blokere `Bruger B`. Angiv `userId=Bruger A` og kommentar-id'et som `Bruger B` skrev.
- Fuldstændig anonyme kommentarer (ingen bruger-id, ingen e-mail) kan ikke blokeres, og en fejl vil blive returneret.

[inline-code-attrs-start title = 'Comment Block cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=some-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

For anonym blokering skal vi angive en `anonUserId`. Dette kan være et ID, der repræsenterer den anonyme session, eller et tilfældigt UUID.
Dette giver os mulighed for at understøtte blokering af kommentarer, selvom en bruger ikke er logget ind, ved at hente kommentarerne med det samme `anonUserId`.

[inline-code-attrs-start title = 'Anonymous Comment Block cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/comments/some-comment-id/block?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-anon-user-id' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Comment Block Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface CommentBlockQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentIdsToCheck?: string[]
}
[inline-code-end]

[inline-code-attrs-start title = 'Comment Block Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
