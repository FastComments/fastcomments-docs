## פרמטרים

| שם | סוג | נדרש | תיאור |
|------|------|----------|-------------|
| tenantId | string | כן |  |
| id | string | כן |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | כן |  |
| userId | string | לא |  |

## תגובה

מחזיר: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateSubscriptionAPIResponse.ts)

## דוגמה

[inline-code-attrs-start title = 'דוגמת updateSubscription'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
const tenantId: string = "tenant_12345";
const subscriptionId: string = "sub_98765";

const updateData: UpdateAPIUserSubscriptionData = {
  planId: "premium_plan",
  status: "active",
  renewalDate: "2024-12-31",
};

const userId: string = "user_abcde";

const response: UpdateSubscriptionAPIResponse = await updateSubscription(
  tenantId,
  subscriptionId,
  updateData,
  userId
);
[inline-code-end]