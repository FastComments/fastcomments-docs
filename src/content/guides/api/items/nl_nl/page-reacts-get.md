[api-resource-header-start name = 'Page Reacts'; route = 'GET /page-reacts/v2/:tenantId'; creditsCost = 0; api-resource-header-end]

Retourneert het aantal voor elke reactie op een pagina, en welke reacties de huidige gebruiker heeft toegevoegd.

Geef `includeUserInfo=public` op om ook de gebruikers die gereageerd hebben te krijgen, als één entry per reactie met zijn reactie‑id, meest recent eerst. Gebruikers met privé‑profielactiviteit en anonieme gebruikers worden weggelaten, en tot 100 reacties worden opgezocht. Alleen `public` wordt hier geaccepteerd; `include-private` geeft een `422` terug en is beschikbaar via [`GET /api/v1/pages/reacts`](/guide-api.html#pages-reacts-get) met een API‑sleutel. Op pagina's die SSO vereisen, geeft `includeUserInfo` een `403` terug voor gebruikers die de pagina niet kunnen bekijken.

[inline-code-attrs-start title = 'Page Reacts cURL Voorbeeld'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/page-reacts/v2/demo?urlId=example-id-or-url'
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts Verzoekstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    urlId: string
    /** URI encoded JSON of your SSO object. Omit for anonymous users. **/
    sso?: string
    /** Set to 'public' to include the users who reacted. **/
    includeUserInfo?: 'public'
}
[inline-code-end]

[inline-code-attrs-start title = 'Page Reacts Responsstructuur'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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