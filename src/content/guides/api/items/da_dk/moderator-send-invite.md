[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Denne rute giver mulighed for at invitere en enkelt `Moderator`.

Følgende begrænsninger gælder for at sende en invitations-e-mail til en `Moderator`:
- `Moderator`en skal allerede eksistere.
- `fromName` må ikke være længere end `100 tegn`.

**Bemærkninger:**
- Hvis en bruger med den angivne e-mail allerede eksisterer, vil de blive inviteret til at moderere din tenants kommentarer.
- Hvis en bruger med den angivne e-mail **ikke eksisterer**, vil invitationslinket guide dem gennem oprettelse af deres konto.
- Invitationen udløber efter `30 dage`.

Vi kan oprette en `Moderator` for en bruger, hvor vi kun kender e-mailen:

[inline-code-attrs-start title = 'Moderator Invitation cURL Eksempel'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Dette vil sende en e-mail som `Bob hos TenantName inviterer dig til at være moderator...`

[inline-code-attrs-start title = 'Moderator Invitation Anmodningsstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** The email sent to the user will appear to be sent from this name. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Moderator Invitation Svarstruktur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]
