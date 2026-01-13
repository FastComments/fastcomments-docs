[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Ovaj endpoint omogućava uklanjanje `TenantUser` po id-u.

Brisanje komentara korisnika je moguće putem query parametra `deleteComments`. Imajte na umu da ako je ovo true:

1. Svi komentari korisnika će biti obrisani uživo.
2. Svi __podređeni__ (sada siročadi) komentari će biti obrisani ili anonimizirani na osnovu konfiguracije stranice povezanе sa svakim komentarom. Na primjer, ako je način brisanja niti "anonymize", onda će odgovori ostati, a komentari korisnika će biti anonimizirani. Ovo se primjenjuje samo kada je `commentDeleteMode` postavljen na `Remove` (podrazumijevana vrijednost).
3. Vrijednost `creditsCost` postaje `2`.

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

`isDeleted` i `isDeletedUser` su postavljeni na `true`.

Pri prikazu, widget komentara će koristiti `DELETED_USER_PLACEHOLDER` (zadano: "[deleted]") za ime korisnika i `DELETED_CONTENT_PLACEHOLDER` za sadržaj komentara. Ovo se može prilagoditi putem UI za prilagođavanje widgeta.

### Primjeri

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za uklanjanje TenantUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za uklanjanje TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // podrazumijevano
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Možete ovo postaviti na true da biste također izbrisali komentare korisnika. Ovo će uduplati trošak kredita. **/
    deleteComments?: 'true' | 'false'
    /** Možete ovo postaviti po želji da odredite kako tretirati komentare korisnika. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za uklanjanje TenantUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
}
[inline-code-end]