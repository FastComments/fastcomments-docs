[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Ovaj endpoint omogućuje slanje pozivnice jednom `Moderator`u.

Sljedeća ograničenja vrijede za slanje pozivnice e-poštom `Moderator`u:
- `Moderator` mora već postojati.
- `fromName` ne smije biti duže od `100 characters`.

**Napomene:**
- Ako korisnik s navedenom e-poštom već postoji, bit će pozvan da moderira komentare vašeg tenanta.
- Ako korisnik s navedenom e-poštom **ne postoji** poveznica za pozivnicu će ih voditi kroz stvaranje njihovog računa.
- Pozivnica će isteći nakon `30 days`.

Možemo stvoriti `Moderator` za korisnika za kojeg znamo samo e-poštu:

[inline-code-attrs-start title = 'Primjer cURL pozivnice za Moderatora'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

This will send an email like `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Struktura zahtjeva za pozivnicu Moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** E-pošta poslana korisniku prikazivat će se kao da je poslana s ovim imenom. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora na pozivnicu Moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]

---