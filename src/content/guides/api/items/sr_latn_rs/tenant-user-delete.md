[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Ova ruta omogućava uklanjanje `TenantUser` objekta po id-u.

Brisanje korisničkih komentara je moguće putem `deleteComments` query parametra. Napomena: ako je ovo true:

1. Svi komentari korisnika biće obrisani uživo.
2. Svi __child__ (sada orphan) komentari biće obrisani ili anonimizirani u zavisnosti od konfiguracije stranice povezane sa svakim komentarom. Na primer, ako je režim brisanja nita "anonymize", odgovori će ostati, a korisnički komentari će biti anonimizirani. Ovo važi samo kada je `commentDeleteMode` `Remove` (podrazumevana vrednost).
3. `creditsCost` postaje `2`.

### Anonimizirani komentari

Možete zadržati komentare korisnika, ali ih jednostavno anonimizirati postavljanjem `commentDeleteMode=1`.

Ako su komentari korisnika anonimizirani, sledeće vrednosti se postavljaju na null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` i `isDeletedUser` se postavljaju na `true`.

Prilikom renderovanja, widget za komentare će koristiti `DELETED_USER_PLACEHOLDER` (podrazumevano: "[deleted]") za ime korisnika i `DELETED_CONTENT_PLACEHOLDER` za komentar. Ovo se može prilagoditi putem Widget Customization UI-ja.

### Primeri

[inline-code-attrs-start title = 'Primer cURL zahteva za uklanjanje TenantUser-a'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/tenant-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za uklanjanje TenantUser-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum TenantUserCommentDeleteMode {
    Remove = 0, // podrazumevano
    Anonymize = 1
}

interface TenantUserDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Možete postaviti ovo na true da biste takođe obrisali komentare korisnika. Ovo duplira cenu u kreditima. **/
    deleteComments?: 'true' | 'false'
    /** Možete postaviti ovo po želji da biste odredili kako se postupati sa komentarima korisnika. **/
    commentDeleteMode?: TenantUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za uklanjanje TenantUser-a'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface TenantUserDeleteResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'not-found' | 'unauthorized'
    /** Uključeno u slučaju neuspeha. **/
    reason?: string
}
[inline-code-end]