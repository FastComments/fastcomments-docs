[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators'; creditsCost = 1; api-resource-header-end]

Deze route biedt de mogelijkheid om één `Moderator` toe te voegen.

Het aanmaken van een `Moderator` heeft de volgende beperkingen:

- Een `name` en `email` moeten altijd worden opgegeven. Een `userId` is optioneel.
- De volgende waarden mogen niet worden opgegeven bij het aanmaken van een `Moderator`:
    - `acceptedInvite`
    - `markReviewedCount`
    - `deletedCount`
    - `markedSpamCount`
    - `approvedCount`
    - `editedCount`
    - `bannedCount`
    - `verificationId`
    - `createdAt`
- Wanneer een `userId` is opgegeven, moet die gebruiker bestaan.
- Wanneer een `userId` is opgegeven, moet die tot dezelfde `tenantId` behoren die in de queryparameters is opgegeven.
- Twee moderators binnen dezelfde tenant kunnen niet worden toegevoegd met hetzelfde `email`.

We kunnen een `Moderator` aanmaken voor een gebruiker waarvan we alleen het e-mailadres kennen:

[inline-code-attrs-start title = 'Moderator aanmaken via e-mail cURL voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators?tenantId=demo&API_KEY=DEMO_API_SECRET' \
  --header 'Content-Type: application/json' \
  --data '{
    "name": "Some Name",
	"email": "someone@someone.com"
}'
[inline-code-end]

Of we kunnen een `Moderator` aanmaken voor een gebruiker die tot onze tenant behoort, om hun moderatiestatistieken bij te houden:

[inline-code-attrs-start title = 'Moderator aanmaken via tenant-gebruiker cURL voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
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

[inline-code-attrs-start title = 'Moderator aanmaken - Requeststructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostQueryParams {
    tenantId: string
    API_KEY: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator aanmaken - Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorPostResponse {
    status: 'success' | 'failed'
    /** Inbegrepen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'name-required' | 'email-required' | 'unexpected-param' | 'not-found'
    /** Inbegrepen bij mislukking. **/
    reason?: string
    moderator?: Moderator; // We geven de volledig aangemaakte moderator terug bij succes.
}
[inline-code-end]