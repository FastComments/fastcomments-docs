## Параметри

| Назва | Type | Обов'язкове | Опис |
|------|------|----------|-------------|
| tenantId | string | Так |  |
| postIds | Array<string> | Так |  |
| sso | string | Ні |  |

## Відповідь

Повертає: [`GetFeedPostsStats200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetFeedPostsStats200Response.ts)

## Приклад

[inline-code-attrs-start title = 'Приклад getFeedPostsStats'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_98765";
const postIds: string[] = ["post_a1b2c3", "post_d4e5f6"];
const ssoToken: string = "sso_jwt_eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9";

const statsWithoutSSO: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds);
const statsWithSSO: GetFeedPostsStats200Response = await getFeedPostsStats(tenantId, postIds, ssoToken);
[inline-code-end]

---