## 参数

| 名称 | 类型 | 必需 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`APIGetUserBadgeProgressResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APIGetUserBadgeProgressResponse.ts)

## 示例

[inline-code-attrs-start title = 'getUserBadgeProgressById 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-tenant-72b1";
const badgeId: string = "badge-4d9f12";
const result: APIGetUserBadgeProgressResponse = await getUserBadgeProgressById(tenantId, badgeId);
const status: APIStatus | undefined = result?.status;
const progressList: UserBadgeProgress[] | undefined = result?.progress;
const firstProgress: UserBadgeProgress | undefined = progressList?.[0];
[inline-code-end]

---