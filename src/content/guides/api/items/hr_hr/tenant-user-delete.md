[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Ova ruta omogućuje uklanjanje `TenantUser` po id-u.

Brisanje korisnikovih komentara moguće je putem query parametra `deleteComments`. Imajte na umu da ako je ovo true:

1. Svi korisnikovi komentari bit će izbrisani odmah.
2. Svi __child__ (sada siročad) komentari bit će izbrisani ili anonimizirani na temelju konfiguracije stranice povezane s pojedinim komentarom. Na primjer, ako je način brisanja threadova "anonymize", tada će odgovori ostati, a komentari korisnika će biti anonimizirani. Ovo se primjenjuje samo kada je `commentDeleteMode` `Remove` (zadana vrijednost).
3. `creditsCost` postaje `2`.

### Anonimizirani komentari

Možete zadržati komentare korisnika, ali ih jednostavno anonimizirati postavljanjem `commentDeleteMode=1`.

Ako su komentari korisnika anonimizirani, sljedeće vrijednosti se postavljaju na null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` i `isDeletedUser` postavljeni su na `true`.

Prilikom prikaza, widget komentara koristit će `DELETED_USER_PLACEHOLDER` (zadano: "[deleted]") za ime korisnika i `DELETED_CONTENT_PLACEHOLDER` za sadržaj komentara. To se može prilagoditi putem sučelja za prilagodbu widgeta.

### Primjeri

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za uklanjanje TenantUsera'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za uklanjanje TenantUsera'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // zadano
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Možete postaviti ovo na true da biste također izbrisali komentare korisnika. To će udvostručiti trošak kredita. **/
    deleteComments?: 'true' | 'false'
    /** Možete ovo postaviti prema potrebi kako biste odredili kako postupati s komentarima korisnika. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za uklanjanje TenantUsera'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]

---