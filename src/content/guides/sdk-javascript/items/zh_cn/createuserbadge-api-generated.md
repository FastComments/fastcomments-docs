## 参数

| 名称 | 类型 | 必填 | 描述 |
|------|------|------|-------------|
| tenantId | string | 是 |  |
| createUserBadgeParams | CreateUserBadgeParams | 是 |  |

## 响应

返回: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## 示例

[inline-code-attrs-start title = 'createUserBadge 示例'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f3b2';
const createUserBadgeParams: CreateUserBadgeParams = {
  name: 'Top Contributor',
  slug: 'top-contributor',
  imageUrl: 'https://assets.fastcomments.com/badges/top-contributor.png',
  description: 'Awarded for 100 helpful comments',
  active: true,
  criteria: { commentsCount: 100 }, // 可选条件
  displayOrder: 10,
  metadata: { featured: true } // 可选元数据
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, createUserBadgeParams);
console.log(result);
[inline-code-end]

---