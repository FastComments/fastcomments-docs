## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| id | string | हाँ |  |
| userId | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/DeleteSubscriptionAPIResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'deleteSubscription उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
  const tenantId: string = "tenant_9876";
  const subscriptionId: string = "sub_54321";
  const userId: string = "user_abc123";

  // वैकल्पिक userId के साथ
  const responseWithUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionId, userId);

  // वैकल्पिक userId के बिना
  const responseWithoutUser: DeleteSubscriptionAPIResponse = await deleteSubscription(tenantId, subscriptionId);
})();
[inline-code-end]