## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|------|------|
| tenantId | string | 예 |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | 예 |  |

## 응답

반환: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## 예제

[inline-code-attrs-start title = 'createSubscription 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "acme-corp-tenant-123";
const createAPIUserSubscriptionData: CreateAPIUserSubscriptionData = {
  userId: "user_98765",
  planId: "pro_monthly",
  paymentMethod: { type: "card", cardId: "card_abc123" },
  autoRenew: true,
  trialDays: 14, // 선택적 매개변수 예시
  metadata: { campaign: "spring_launch" } // 선택적 매개변수 예시
};
const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, createAPIUserSubscriptionData);
[inline-code-end]

---