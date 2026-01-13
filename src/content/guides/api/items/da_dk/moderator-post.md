[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Denne rute giver mulighed for at tilføje en enkelt `Moderator`.

Oprettelse af en `Moderator` har følgende begrænsninger:

- Et `name` og `email` skal altid angives. Et `userId` er valgfrit.
- Følgende værdier må ikke angives ved oprettelse af en `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Når et `userId` er angivet, skal den bruger eksistere.
- Når et `userId` er angivet, skal de tilhøre det samme `tenantId`, der er angivet i query-parametre.
- To moderatorer i den samme tenant kan ikke tilføjes med den samme `email`.

Vi kan oprette en `Moderator` for en bruger, hvor vi kun kender e-mailen:

[inline-code-attrs-start title = 'Moderator Opret via E-mail cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Eller vi kan oprette en `Moderator` for en bruger, der tilhører vores tenant, for at spore deres moderationsstatistikker:

[inline-code-attrs-start title = 'Moderator Opret via Tenant Bruger cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com",
    "userId": "some-tenant-user-id"
}'
[inline-code-end]

[inline-code-attrs-start title = 'Moderator Oprettelse Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator Oprettelse Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Included on failure. **/
    reason?: string
    moderator?: Moderator; // We return the complete created moderator on success.
}
[inline-code-end]
