[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Deze route biedt de mogelijkheid om één `Moderator` uit te nodigen.

De volgende beperkingen gelden om een uitnodigingsmail naar een `Moderator` te sturen:
- De `Moderator` moet al bestaan.
- De `fromName` mag niet langer zijn dan `100 characters`.

**Opmerkingen:**
- Als een gebruiker met het opgegeven e-mailadres al bestaat, wordt hij uitgenodigd om de opmerkingen van uw tenant te modereren.
- Als een gebruiker met het opgegeven e-mailadres **niet bestaat**, leidt de uitnodigingslink die persoon door het aanmaken van een account.
- De uitnodiging verloopt na `30 days`.

We kunnen een `Moderator` aanmaken voor een gebruiker waarvan we alleen het e-mailadres kennen:

[inline-code-attrs-start title = 'Voorbeeld cURL-verzoek voor moderatoruitnodiging'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Dit zal een e-mail sturen zoals `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Structuur van het verzoek voor moderatoruitnodiging'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** De e-mail die naar de gebruiker wordt gestuurd zal afkomstig lijken van deze naam. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Structuur van de respons voor moderatoruitnodiging'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Opgenomen bij mislukking. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Opgenomen bij mislukking. **/
    reason?: string
}
[inline-code-end]

---