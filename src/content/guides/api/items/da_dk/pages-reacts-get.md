[api-resource-header-start name = 'Page Reacts'; route = 'GET /api/v1/pages/reacts'; creditsCost = 1; api-resource-header-end]

Returnerer antallet for hver reaktion på en side, og valgfrit brugerne der reagerede, som én post per reaktion med dens reaktions‑id, seneste først. Dette er server‑side‑versionen af [`GET /page-reacts/v2/:tenantId`](/guide-api.html#page-reacts-get).

`includeUserInfo` styrer hvilke brugere der returneres:

- `public` udelader brugere, der har gjort deres profilaktivitet privat.
- `include-private` inkluderer dem. Brug kun dette når listen ikke vises for offentligheden.

Anonyme brugere er aldrig inkluderet, og op til 100 reaktioner slås op. Sider der ikke findes returnerer ingen `counts` og en tom `users`‑liste.

[inline-code-attrs-start title = 'Eksempel på cURL for Page Reacts'; type = 'bash'; useDemoTenant = true; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
curl --request GET \
  --url 'https://fastcomments.com/api/v1/pages/reacts?tenantId=demo&API_KEY=DEMO_API_SECRET&urlId=example-id-or-url&includeUserInfo=public'
[inline-code-end]

[inline-code-attrs-start title = 'Struktur for Page Reacts-anmodning'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsRequestQueryParams {
    tenantId: string
    API_KEY: string
    urlId: string
    /** Udelad for kun at få tællingerne. **/
    includeUserInfo?: 'public' | 'include-private'
}
[inline-code-end]

[inline-code-attrs-start title = 'Struktur for Page Reacts-svar'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
interface PageReactsResponse {
    status: 'success' | 'failed'
    /** Inkluderet ved fejl. **/
    code?: 'missing-tenant-id' | 'invalid-tenant-id' | 'invalid-api-key' | 'missing-api-key' | 'missing-url-id'
    /** Inkluderet ved fejl. **/
    reason?: string
    /** Tælling per reaktions-id, for eksempel {"heart": 12, "laugh": 3}. Ikke angivet når siden har ingen reaktioner. **/
    counts?: Record<string, number>
    /** En post per reaktion, seneste først. Kun angivet når includeUserInfo er medsendt. **/
    users?: PageCustomReactUser[]
}

interface PageCustomReactUser {
    /** Reaktions-id'et, for eksempel "heart". **/
    reactId: string
    userId: string
    username: string
    displayName?: string
    avatarSrc?: string
}
[inline-code-end]

#### Nyttigt tip

Husk at URI‑enkode værdier som `urlId`.

---