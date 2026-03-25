## 매개변수

| 이름 | 형식 | 필수 | 설명 |
|------|------|----------|-------------|
| tenantId | string | 예 |  |
| userId | string | 아니오 |  |

## 응답

반환: [`GetSubscriptionsAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/GetSubscriptionsAPIResponse.ts)

## 예제

[inline-code-attrs-start title = 'getSubscriptions 예제'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = 'tenant_acme_corp_01';
const userId: string = 'user_76a3b9f2';
const subscriptionsForUser: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId, userId);
const subscriptionsForTenant: GetSubscriptionsAPIResponse = await getSubscriptions(tenantId);
[inline-code-end]

---