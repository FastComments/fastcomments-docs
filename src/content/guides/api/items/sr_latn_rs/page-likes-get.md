[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Vraća broj lajkova na stranici i da li je trenutni korisnik lajkovao. Stranice koje još ne postoje vraćaju `likeCount` od `0`.

Prosledite `includeUserInfo=public` da biste takođe dobili korisnike koji su lajkovali stranicu, najnoviji prvi. Korisnici sa privatnom aktivnošću profila i anonimni korisnici su izostavljeni, a pretražuje se do 100 lajkova. Ovde je prihvaćen samo `public`; `include-private` vraća `422` i dostupan je iz [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) uz API ključ. Na stranicama koje zahtevaju SSO, `includeUserInfo` vraća `403` za korisnike koji ne mogu da vide stranicu.

[inline-code-attrs-start title = 'Primer cURL zahteva za lajkove stranice'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahteva za lajkove stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI enkodovan JSON vašeg SSO objekta. Izostavite za anonimne korisnike. **/
    sso?: string
    /** Postavite na 'public' da uključite korisnike koji su lajkovali stranicu. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za lajkove stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Uključeno pri grešci. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Uključeno pri grešci. **/
    reason?: string
    likeCount: number
    /** Da li je korisnik koji je poslao zahtev lajkovao stranicu. **/
    didLike: boolean
    /** Broj komentara najvišeg nivoa na stranici. **/
    commentCount: number
    /** ID koji se koristi za pretplatu na ažuriranja u realnom vremenu za ovu stranicu. **/
    urlIdWS: string
    /** Najnoviji prvi. Postavlja se samo kada je prosleđen includeUserInfo. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

---