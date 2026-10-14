[api-resource-header-start name = 'Page Likes'; route = 'GET /page-reacts/v1/likes/:tenantId'; creditsCost = 0; api-resource-header-end]

Vraća broj lajkova na stranici i je li trenutni korisnik lajkao stranicu. Stranice koje još ne postoje vraćaju `likeCount` od `0`.

Proslijedite `includeUserInfo=public` kako biste također dobili korisnike koji su lajkovali stranicu, najnoviji prvi. Korisnici s privatnom aktivnošću profila i anonimni korisnici su izostavljeni, a pretražuje se do 100 lajkova. Ovdje je prihvaćen samo `public`; `include-private` vraća `422` i dostupan je putem [`GET /api/v1/pages/likes`](/guide-api.html#pages-likes-get) s API ključem. Na stranicama koje zahtijevaju SSO, `includeUserInfo` vraća `403` za korisnike koji ne mogu vidjeti stranicu.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za lajkove stranice'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v1/likes/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za lajkove stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who liked the page. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za lajkove stranice'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageLikesResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Included on failure. **/
    reason?: string
    likeCount: number
    /** Whether the user making the request has liked the page. **/
    didLike: boolean
    /** The number of top level comments on the page. **/
    commentCount: number
    /** The id used to subscribe to live updates for this page. **/
    urlIdWS: string
    /** Most recent first. Only set when includeUserInfo is passed. **/
    users?: PageReactUser[]
}

interface PageReactUser {
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]