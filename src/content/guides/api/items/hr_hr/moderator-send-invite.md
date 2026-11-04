[api-resource-header-start name = 'Moderator'; route = 'POST /api/v1/moderators/:id/send-invite'; creditsCost = 10; api-resource-header-end]

Ova ruta omogućuje pozivanje jednog `Moderator`a.

Za slanje pozivnice e-pošte `Moderator`u postoje sljedeća ograničenja:
- `Moderator` već mora postojati.
- `fromName` ne smije biti dulji od `100 znakova`.

**Napomene:**
- Ako korisnik s navedenom adresom e-pošte već postoji, bit će pozvan da moderira komentare vašeg tenanta.
- Ako korisnik s navedenom adresom e-pošte **ne postoji**, poveznica za pozivnicu vodit će ga kroz postupak stvaranja računa.
- Pozivnica istječe nakon `30 dana`.

Možemo stvoriti `Moderator`a za korisnika za kojeg znamo samo adresu e-pošte:

[inline-code-attrs-start title = 'Primjer cURL pozivnice moderatoru'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/moderators/xyz/send-invite?tenantId=demo&API_KEY=DEMO_API_SECRET&fromName=Bob' \
  --header 'Content-Type: application/json'
[inline-code-end]

Ovo će poslati e-poštu poput `Bob at TenantName is inviting you to be a moderator...`

[inline-code-attrs-start title = 'Struktura zahtjeva za pozivnicu moderatoru'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteQueryParams {
    tenantId: string
    API_KEY: string
    /** E-pošta poslana korisniku prikazat će se kao poslana s ovog imena. **/
    fromName: string
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora na pozivnicu moderatoru'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface ModeratorSendInviteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'unauthorized' | 'not-found | 'from-name-required' | 'from-name-invalid'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]