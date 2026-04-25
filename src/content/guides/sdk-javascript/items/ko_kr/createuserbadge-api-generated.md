## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| createUserBadgeParams | CreateUserBadgeParams | 예 |  |

## 응답

반환: [`CreateUserBadge200Response`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateUserBadge200Response.ts)

## 예제

[inline-code-attrs-start title = 'createUserBadge 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_9a8b7c";
const params: CreateUserBadgeParams = {
  name: "Top Contributor",
  slug: "top-contributor",
  description: "Awarded for 100 approved comments",
  iconUrl: "https://cdn.fastcomments.com/badges/top-contributor.png",
  active: true,
  criteria: { approvedComments: 100 },
  customConfig: { showOnProfile: true } // 선택적 매개변수
};
const result: CreateUserBadge200Response = await createUserBadge(tenantId, params);
[inline-code-end]

---