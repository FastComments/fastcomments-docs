## Parameters

| Naam | Type | Vereist | Beschrijving |
|------|------|----------|-------------|
| tenantId | string | Ja |  |
| postIds | Array<string> | Ja |  |
| sso | string | Nee |  |

## Respons

Geeft terug: [`GetFeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsStatsResponse.ts)

## Voorbeeld

[inline-code-attrs-start title = 'getFeedPostsStats Voorbeeld'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample(): Promise<void> {
    const tenantId: string = "tenant_12345";
    const postIds: string[] = ["post_a1b2c3", "post_d4e5f6"];
    const ssoToken: string = "sso_abcdef123456";

    const statsWithoutSso: GetFeedPostsStatsResponse = await getFeedPostsStats(tenantId, postIds);
    const statsWithSso: GetFeedPostsStatsResponse = await getFeedPostsStats(tenantId, postIds, ssoToken);
}

runExample();
[inline-code-end]