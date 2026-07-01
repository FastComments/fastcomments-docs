## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| id | string | 예 |  |

## 응답

반환: [`DeleteUserBadgeResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteUserBadgeResponse.ts)

## 예시

[inline-code-attrs-start title = 'deleteUserBadge 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function exampleDeleteBadge(): Promise<void> {
  const tenantId: string = "tenant_9f8b7c6a";
  const badgeId: string = "badge_4e3d2c1b";
  const result: DeleteUserBadgeResponse = await deleteUserBadge(tenantId, badgeId);
  console.log(result);
}
exampleDeleteBadge();
[inline-code-end]