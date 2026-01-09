[api-resource-header-start name = 'Vote'; route = 'POST /api/v1/votes'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućava dodavanje jednog autorizovanog `Vote`. Glasovi mogu biti `up` (+1) ili `down` (-1).

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
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-user-id' | 'invalid-user-id' | 'invalid-comment-id' | 'invalid-direction' | 'duplicate' | 'voting-disabled'
    /** Uključeno u slučaju greške. **/
    reason?: string
    voteId?: string
}
[inline-code-end]

### Kreiranje anonimnih glasova

Anonimni glasovi se mogu kreirati postavljanjem `anonUserId` u parametrima upita umjesto `userId`.

Ovaj id ne mora odgovarati nijednom objektu korisnika bilo gdje (otuda anonimno). To je jednostavno identifikator za sesiju, tako da možete ponovo dohvatiti glasove u istoj sesiji, da provjerite da li je komentar već glasovan.

Ako nemate nešto poput "anonimnih sesija" kao što ih FastComments ima - možete jednostavno postaviti ovo na slučajan ID, poput UUID-a (iako preferiramo manje identifikatore kako bismo uštedjeli prostor).

### Ostale napomene

- Ovaj API poštuje postavke na nivou tenanta. Na primjer, ako onemogućite glasanje za određenu stranicu, i pokušate kreirati glas putem API-ja, to će propasti sa kodom greške `voting-disabled`.
- Ovaj API je podrazumevano aktivan.
- Ovaj API će ažurirati `votes` odgovarajućeg `Comment`.