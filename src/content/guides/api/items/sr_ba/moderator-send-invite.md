[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Ova ruta omogućava slanje pozivnice jednom `Moderator`u.

Slijedeća ograničenja postoje za slanje email pozivnice `Moderator`u:
- `Moderator` mora već postojati.
- `fromName` ne smije biti duži od `100 characters`.

**Napomene:**
- Ako korisnik sa navedenim emailom već postoji, bit će pozvan da moderira komentare vašeg tenanta.
- Ako korisnik sa navedenim emailom **ne postoji** link za pozivnicu će ih provesti kroz proces kreiranja naloga.
- Pozivnica će isteći nakon `30 days`.

Možemo kreirati `Moderator`a za korisnika čiji email znamo:

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za pozivnicu moderatoru'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Ovo će poslati e-mail poput `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Struktura zahtjeva za pozivnicu moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** E-mail poslan korisniku će izgledati kao da je poslan s ovog imena. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za pozivnicu moderatora'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Included on failure. **/
    reason?: string
}
[inline-code-end]

---