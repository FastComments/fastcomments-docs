[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućuje dodavanje jednog autoriziranog `Vote`. Glasovi mogu biti `up` (+1) ili `down` (-1).

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za kreiranje glasa'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&userId=user-id&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za kreiranje anonimnog glasa'; type = 'bash'; useDemoTenantUser = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request POST \
  --url 'https://fastcomments.com/api/v1/votes?tenantId=demo&API_KEY=DEMO_API_SECRET&anonUserId=some-randomly-generated-identifier&commentId=comment-id&direction=up' \
  --header 'Content-Type: application/json'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za kreiranje glasa'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Kreiranje anonimnih glasova

Anonimne glasove možete kreirati postavljanjem `anonUserId` u parametrima upita umjesto `userId`.

Ovaj id ne mora odgovarati nijednom objektu korisnika (otuda anonimno). To je jednostavno identifikator sesije, tako da možete ponovno dohvatiti glasove u istoj sesiji kako biste provjerili je li komentar već glasovan.

Ako nemate nešto poput "anonimnih sesija" kao što FastComments ima - jednostavno možete postaviti ovo na nasumični ID, poput UUID-a (iako cijenimo manje identifikatore radi uštede prostora).

### Ostale napomene

- Ovaj API poštuje postavke na razini tenanta. Na primjer, ako onemogućite glasanje za određenu stranicu, a pokušate stvoriti glas putem API-ja, to će rezultirati pogreškom s kodom `voting-disabled`.
- Ovaj API je po zadanom aktivan.
- Ovaj API će ažurirati `votes` odgovarajućeg `Comment`.