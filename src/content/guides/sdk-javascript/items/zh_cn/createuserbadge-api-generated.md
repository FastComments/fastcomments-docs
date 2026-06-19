## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|----------|-------------|
| tenantId | string | 是 |  |
| createUserBadgeParams | CreateUserBadgeParams | 是 |  |

## 响应

返回： [`APICreateUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/APICreateUserBadgeResponse.ts)

## 示例

[inline-code-attrs-start title = 'createUserBadge 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = 'tenant_84a2c6b2';
  const createUserBadgeParams: CreateUserBadgeParams = {
    name: 'Early Supporter',
    description: 'Awarded to users who joined during the alpha launch',
    iconUrl: 'https://cdn.fastcomments.com/badges/early-supporter.png',
    criteria: 'Joined before 2021-06-01',
    isActive: true,
    notifyUsers: true // 可选参数
  };
  const result: APICreateUserBadgeResponse = await createUserBadge(tenantId, createUserBadgeParams);
  console.log(result);
})();
[inline-code-end]

---