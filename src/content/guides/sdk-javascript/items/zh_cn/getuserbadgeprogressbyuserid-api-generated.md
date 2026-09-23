## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|------|
| tenantId | string | 是 |  |
| userId | string | 是 |  |

## 响应

返回: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeProgressResponse.ts)

## 示例

[inline-code-attrs-start title = 'getUserBadgeProgressByUserId 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp";
const userId: string = "user-42";

const badgeProgress: APIGetUserBadgeProgressResponse = await getUserBadgeProgressByUserId(tenantId, userId);

if (badgeProgress.status?.code !== undefined) {
  const statusCode: number = badgeProgress.status.code;
}

if (badgeProgress.progress?.length) {
  const firstProgress: UserBadgeProgress = badgeProgress.progress[0];
}
[inline-code-end]