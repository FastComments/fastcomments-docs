[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint omogućava dodavanje jednog autorizovanog `Vote`. Glasovi mogu biti `up` (+1) ili `down` (-1).

[inline-code-attrs-start title = 'Primer cURL zahteva za kreiranje glasa'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Primer anonimnog cURL zahteva za kreiranje glasa'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za kreiranje glasa'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostQueryParams {
    tenantId: string
    API_KEY: string
    userId?: string
    anonUserId?: string
    commentId: string
    direction: 'up' | 'down'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za kreiranje glasa'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface VotePostResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Kreiranje anonimnih glasova

Anonimni glasovi se mogu kreirati postavljanjem `anonUserId` u parametrima upita umesto `userId`.

Ovaj id ne mora odgovarati nijednom objektu korisnika (otuda anonimno). To je jednostavno identifikator
za sesiju, tako da možete ponovo dohvatiti glasove u istoj sesiji, da proverite da li je komentar
dobio glas.

Ako nemate takvu stvar kao „anonimne sesije“ kao što FastComments ima - možete jednostavno
postaviti ovo na nasumični ID, kao što je UUID (iako cenimo manje identifikatore radi uštede prostora).

### Ostale napomene

- Ovaj API poštuje podešavanja na nivou tenanta. Na primer, ako onemogućite glasanje za određenu stranicu, i pokušate da kreirate glas preko API-ja, to će se završiti greškom sa kodom `voting-disabled`.
- Ovaj API je podrazumevano aktivan.
- Ovaj API će ažurirati `votes` odgovarajućeg `Comment`.

---