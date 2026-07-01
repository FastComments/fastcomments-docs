## Параметри

| Назва | Тип | Обов’язково | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| postIds | Array<string> | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetFeedPostsStatsResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsStatsResponse.ts)

## Приклад

[inline-code-attrs-start title = 'getFeedPostsStats Приклад'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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

---