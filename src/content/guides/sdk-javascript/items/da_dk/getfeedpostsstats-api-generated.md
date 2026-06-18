## Parametre

| Navn | Type | Påkrævet | Beskrivelse |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postIds | Array<string> | Ja |  |
| sso | string | Nej |  |

## Svar

Returnerer: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsStats200Response.ts)

## Eksempel

[inline-code-attrs-start title = 'getFeedPostsStats Eksempel'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_98765";
const postIds: string[] = ["post_a1b2c3", "post_d4e5f6"];
const ssoToken: string = "sso_jwt_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";

const statsWithoutSSO: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds);
const statsWithSSO: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds, ssoToken);
[inline-code-end]

---