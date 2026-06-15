## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| id | string | 是 |  |

## 响应

返回: [`GetUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetUserBadge200Response.ts)

## 示例

[inline-code-attrs-start title = 'getUserBadge 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_6b8f2a1c';
  const id: string = 'badge_9f3d4b2a';
  const response: GetUserBadge200Response = await getUserBadge(tenantId, id);
  const badge: UserBadge | undefined = response.userBadge;
  const badgeName: string | undefined = badge?.name;
  console.log('Retrieved badge name:', badgeName);
})();
[inline-code-end]

---