किसी पेज के लिए नोटिफिकेशन सक्षम या अक्षम करें। जब उपयोगकर्ता किसी पेज की सदस्यता लेते हैं, तो नए रूट कमेंट्स के लिए नोटिफिकेशन बनाए जाते हैं, और साथ ही

## पैरामीटर

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| url | string | हाँ |  |
| pageTitle | string | हाँ |  |
| subscribedOrUnsubscribed | UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum | हाँ |  |
| sso | string | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-sdk-js/blob/main/src/generated/src/models/UpdateUserNotificationPageSubscriptionStatusResponse.ts)

## उदाहरण

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus उदाहरण'; type = 'typescript'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
async function runExample() {
  const tenantId: string = "tenant_12345";
  const urlId: string = "url_9876";
  const url: string = "https://example.com/articles/awesome-article";
  const pageTitle: string = "Awesome Article";
  const subscribedOrUnsubscribed: UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum =
    UpdateUserNotificationPageSubscriptionStatusSubscribedOrUnsubscribedEnum.Subscribed;
  const sso: string = "sso_token_abc";

  const response: UpdateUserNotificationPageSubscriptionStatusResponse = await updateUserNotificationPageSubscriptionStatus(
    tenantId,
    urlId,
    url,
    pageTitle,
    subscribedOrUnsubscribed,
    sso
  );

  console.log(response);
}

runExample();
[inline-code-end]