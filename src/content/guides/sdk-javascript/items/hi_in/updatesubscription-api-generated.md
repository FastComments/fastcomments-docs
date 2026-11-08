## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| updateAPIUserSubscriptionData | UpdateAPIUserSubscriptionData | हाँ |  |
| userId | string | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`UpdateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateSubscriptionAPIResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'updateSubscription उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
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