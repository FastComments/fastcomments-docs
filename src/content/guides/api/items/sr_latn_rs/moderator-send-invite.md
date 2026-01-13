[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Ovaj endpoint omogućava da pozovete jednog `Moderator`.

Sledeća ograničenja važe za slanje email pozivnice `Moderator`-u:
- `Moderator` mora već postojati.
- `fromName` ne sme biti duže od `100 characters`.

**Napomene:**
- Ako korisnik sa datom email adresom već postoji, biće pozvan da moderira komentare vašeg tenanta.
- Ako korisnik sa datom email adresom **ne postoji**, link u pozivnici će ih uputiti kroz proces kreiranja naloga.
- Pozivnica ističe nakon `30 days`.

Možemo kreirati `Moderator` za korisnika za kog znamo samo email:

[inline-code-attrs-start title = 'Primer cURL pozivnice za Moderator'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Ovo će poslati email koji izgleda otprilike ovako: `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Struktura zahteva za poziv Moderator-u'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** Email poslat korisniku će izgledati kao da je poslat sa ovog imena. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za poziv Moderator-u'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
}
[inline-code-end]

---