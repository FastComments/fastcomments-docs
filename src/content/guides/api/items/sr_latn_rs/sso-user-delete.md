[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Ovaj endpoint omogućava uklanjanje pojedinačnog SSO korisnika po njegovom id-u.

Imajte na umu da ponovno učitavanje komentarskog widgeta sa payload-om za ovog korisnika jednostavno ponovo kreira korisnika bez prekida.

Brisanje korisnikovih komentara je moguće putem query parametra `deleteComments`. Imajte na umu da ako je ovo tačno:

1. Svi korisnikovi komentari biće obrisani uživo.
2. Svi __child__ (sada siroti) komentari biće obrisani ili anonimizovani u zavisnosti od konfiguracije stranice kojoj svaki komentar pripada. Na primer, ako je režim brisanja niti "anonymize", onda će odgovori ostati, a korisnikovi komentari će biti anonimizovani. Ovo se primenjuje samo kada je `commentDeleteMode` postavljen na `Remove` (podrazumevana vrednost).
3. `creditsCost` postaje `2`.

### Anonimizovani komentari

Možete zadržati korisnikove komentare, ali ih jednostavno anonimizovati postavljanjem `commentDeleteMode=1`.

Ako su korisnikovi komentari anonimizovani, sledeće vrednosti se postavljaju na null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` i `isDeletedUser` se postavljaju na `true`.

Prilikom renderovanja, komentarski widget će za korisničko ime koristiti `DELETED_USER_PLACEHOLDER` (podrazumevano: "[deleted]") a za sadržaj komentara `DELETED_CONTENT_PLACEHOLDER`. Ovo se može prilagoditi preko korisničkog interfejsa za prilagođavanje widgeta.

### Primeri

[inline-code-attrs-start title = 'Primer cURL zahteva za uklanjanje SSOUser'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za uklanjanje SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // podrazumevano
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Možete ovo postaviti na true da biste takođe obrisali komentare korisnika. Ovo će duplirati trošak u kreditima. **/
    deleteComments?: 'true' | 'false'
    /** Možete ovo postaviti po želji da odredite kako postupati sa komentarima korisnika. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za uklanjanje SSOUser'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju greške. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Uključeno u slučaju greške. **/
    reason?: string
    user?: SSOUser; // Vraćamo uklonjenog korisnika u slučaju uspeha.
}
[inline-code-end]