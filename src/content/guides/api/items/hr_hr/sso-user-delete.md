[api-resource-header-start name = 'SSOUser'; route = 'DELETE /api/v1/sso-users/:id'; creditsCost = 1; api-resource-header-end]

Ova ruta omogućuje uklanjanje pojedinog SSO korisnika prema njegovom id-u.

Imajte na umu da ponovno učitavanje widgeta komentara s podacima za ovog korisnika jednostavno će ponovno stvoriti korisnika bez poteškoća.

Brisanje korisnikovih komentara moguće je putem upitnog parametra `deleteComments`. Imajte na umu da ako je ovo istinito:

1. Svi korisnikovi komentari bit će izbrisani uživo.
2. Svi __podređeni__ (sada bez roditelja) komentari bit će izbrisani ili anonimizirani ovisno o konfiguraciji stranice povezanoj sa svakim komentarom. Na primjer, ako je način brisanja niti "anonymize", tada će odgovori ostati, a korisnikovi komentari će biti anonimizirani. Ovo se primjenjuje samo kada je `commentDeleteMode` `Remove` (zadana vrijednost).
3. `creditsCost` postaje `2`.

### Anonimizirani komentari

Možete zadržati korisnikove komentare, ali ih jednostavno anonimizirati postavljanjem `commentDeleteMode=1`.

Ako su korisnikovi komentari anonimizirani, sljedeće vrijednosti postaju null:

    - commenterName
    - commenterEmail
    - avatarSrc
    - userId
    - anonUserId
    - mentions
    - badges

`isDeleted` i `isDeletedUser` postavljeni su na `true`.

Prilikom prikaza, widget komentara koristit će `DELETED_USER_PLACEHOLDER` (zadano: "[izbrisano]") za korisnikovo ime i `DELETED_CONTENT_PLACEHOLDER` za komentar. Ovo možete prilagoditi putem sučelja za prilagodbu widgeta.

### Primjeri

[inline-code-attrs-start title = 'Primjer cURL uklanjanja SSO korisnika'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request DELETE \
  --url 'https://fastcomments.com/api/v1/sso-users/xyz?tenantId=demo&API_KEY=DEMO_API_SECRET'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za uklanjanje SSO korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
enum SSOUserCommentDeleteMode {
    Remove = 0, // zadano
    Anonymize = 1
}

interface SSOUsersDeleteQueryParams {
    tenantId: string
    API_KEY: string
    /** Možete postaviti ovo na true da također izbrišete korisnikove komentare. Ovo će udvostručiti trošak kredita. **/
    deleteComments?: 'true' | 'false'
    /** Možete ovo postaviti po želji kako biste odredili kako postupati s korisnikovim komentarima. **/
    commentDeleteMode?: SSOUserCommentDeleteMode
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za uklanjanje SSO korisnika'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface SSOUsersResponse {
    status: 'success' | 'failed'
    /** Uključeno u slučaju neuspjeha. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-id' | 'user-does-not-exist'
    /** Uključeno u slučaju neuspjeha. **/
    reason?: string
    user?: SSOUser; // Vraćamo uklonjenog korisnika pri uspjehu.
}
[inline-code-end]