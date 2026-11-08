किसी पेज के लिए नोटिफिकेशन सक्षम या अक्षम करें। जब उपयोगकर्ता किसी पेज की सदस्यता लेते हैं, तो नई रूट कमेंट्स के लिए नोटिफिकेशन बनाई जाती हैं, और साथ ही

## पैरामीटर्स

| नाम | प्रकार | आवश्यक | विवरण |
|------|------|----------|-------------|
| tenantId | string | हाँ |  |
| urlId | string | हाँ |  |
| url | string | नहीं |  |
| pageTitle | string | नहीं |  |
| subscribedOrUnsubscribed | string | नहीं |  |
| sso | string = "" | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`Option[UpdateUserNotificationPageSubscriptionStatusResponse]`](https://github.com/FastComments/fastcomments-nim/blob/master/client/fastcomments/models/model_update_user_notification_page_subscription_status_response.nim)

## उदाहरण

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus उदाहरण'; type = 'nim'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
let (optResp, httpResp) = client.updateUserNotificationPageSubscriptionStatus(
  tenantId = "my-tenant-123",
  urlId = "news/article-456",
  url = "https://example.com/news/article-456",
  pageTitle = "Breaking News: Something Happened",
  subscribedOrUnsubscribed = "subscribed",
  sso = ""
)

if optResp.isSome:
  let resp = optResp.get()
  # resp के साथ आगे की प्रोसेसिंग
[inline-code-end]