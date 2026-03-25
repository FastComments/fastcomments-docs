## Parameters

| Name | Type | Required | Description |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createUserBadgeParams | CreateUserBadgeParams | 예 |  |

## Response

반환: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## Example

[inline-code-attrs-start title = 'createUserBadge 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_84f3b2';
const createUserBadgeParams: CreateUserBadgeParams = {
  name: 'Top Contributor',
  slug: 'top-contributor',
  imageUrl: 'https://assets.fastcomments.com/badges/top-contributor.png',
  description: 'Awarded for 100 helpful comments',
  active: true,
  criteria: { commentsCount: 100 }, // 선택적 기준
  displayOrder: 10,
  metadata: { featured: true } // 선택적 메타데이터
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, createUserBadgeParams);
console.log(result);
[inline-code-end]

---