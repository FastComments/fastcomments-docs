[api-resource-header-start name = 'TenantUser'; route = 'DELETE /api/v1/tenant-users/:id'; creditsCost = 5; api-resource-header-end]

Ovaj endpoint omogućava uklanjanje `TenantUser` po id-u.

Brisanje korisnikovih komentara je moguće putem query parametra `deleteComments`. Imajte na umu da ako je ovo tačno:

1. Svi korisnikovi komentari biće obrisani uživo.
2. Svi __child__ (sada siroti) komentari biće obrisani ili anonimnišu na osnovu konfiguracije svake stranice kojoj komentar pripada. Na primer, ako je režim brisanja niti "anonymize", odgovori će ostati, a korisnikovi komentari će biti anonimizovani. Ovo se primenjuje samo kada je `commentDeleteMode` postavljen na `Remove` (podrazumevana vrednost).
3. `creditsCost` postaje `2`.

### Anonymized Comments

Možete zadržati korisnikove komentare, ali ih jednostavno anonimizovati tako što ćete postaviti `commentDeleteMode=1`.

Ako su korisnikovi komentari anonimni, sledeće vrednosti se postavljaju na null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` i `isDeletedUser` se postavljaju na `true`.

Pri prikazu, widget za komentare će koristiti `DELETED_USER_PLACEHOLDER` (podrazumevano: "[deleted]") za korisnikovo ime i `DELETED_CONTENT_PLACEHOLDER` za komentar. Ovo se može prilagoditi putem korisničkog interfejsa za prilagođavanje widgeta.

### Examples

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
    /** Možete postaviti ovo na true da biste takođe obrisali korisnikove komentare. Ovo će udvostručiti cenu u kreditima. **/
    deleteComments?: 'true' | 'false'
    /** Možete ovo podesiti kako biste odredili kako postupati sa korisnikovim komentarima. **/
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