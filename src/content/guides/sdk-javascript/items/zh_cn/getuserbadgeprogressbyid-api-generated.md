## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`GetUserBadgeProgressById200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadgeProgressById200Response.ts)

## 示例

[inline-code-attrs-start title = 'getUserBadgeProgressById 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'fastcomments-tenant-241';
const badgeId: string = 'user-78b3d-badge-3';
const response: GetUserBadgeProgressById200Response = await getUserBadgeProgressById(tenantId, badgeId);
const progress: UserBadgeProgress | undefined = (response as unknown as { progress?: UserBadgeProgress }).progress;
const percentComplete: number | undefined = progress?.percentage;
console.log('Badge progress percent complete:', percentComplete);
[inline-code-end]

---