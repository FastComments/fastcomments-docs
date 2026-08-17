## 매개변수

| 이름 | 유형 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| id | string | 예 |  |
| userId | string | 아니오 |  |

## 응답

반환: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteSubscriptionAPIResponse.ts)

## 예제

[inline-code-attrs-start title = 'deleteSubscription 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_9876";
  const subscriptionId: string = "sub_54321";
  const userId: string = "user_abc123";

  // 선택적 userId 포함
  const responseWithUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionId, userId);

  // 선택적 userId 없이
  const responseWithoutUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionId);
})();
[inline-code-end]