## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| createAPIUserSubscriptionData | CreateAPIUserSubscriptionData | हाँ |  |

## प्रतिक्रिया

रिटर्न करता है: [`CreateSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/CreateSubscriptionAPIResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'createSubscription उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
(async () => {
    const tenantId: string = "acme-corp-123";

    const subscriptionData: CreateAPIUserSubscriptionData = {
        userId: "user-456",
        planId: "premium-monthly",
        startDate: new Date(),
        trialPeriodDays: 14 // वैकल्पिक फ़ील्ड
    };

    const result: CreateSubscriptionAPIResponse = await createSubscription(tenantId, subscriptionData);
    console.log(result);
})();
[inline-code-end]