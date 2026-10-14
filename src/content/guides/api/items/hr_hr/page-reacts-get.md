[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Vraća broj za svaku reakciju na stranici i koje je reakcije trenutni korisnik dodao.

Proslijedite `includeUserInfo=public` kako biste također dobili korisnike koji su reagirali, kao jedan unos po reakciji s njegovim ID-om reakcije, najnoviji prvi. Korisnici s privatnom aktivnošću profila i anonimni korisnici su izostavljeni, a pregledava se najviše 100 reakcija. Ovdje je prihvaćen samo `public`; `include-private` vraća `422` i dostupan je iz [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) s API ključem. Na stranicama koje zahtijevaju SSO, `includeUserInfo` vraća `403` za korisnike koji ne mogu pregledati stranicu.

[inline-code-attrs-start title = 'Primjer cURL zahtjeva za Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Struktura zahtjeva za Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who reacted. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktura odgovora za Page Reacts'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Included on failure. **/
    code?: 'include-private-not-allowed' | 'sso-required' | 'page-forbidden' | string
    /** Included on failure. **/
    reason?: string
    /** Count per reaction id, for example {"heart": 12, "laugh": 3}. Not set when the page has no reactions. **/
    counts?: Record<string, number>
    /** The reaction ids the user making the request has added. Not set when they have added none. **/
    reactedIds?: string[]
    /** One entry per react, most recent first. Only set when includeUserInfo is passed. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** The reaction id, for example "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]