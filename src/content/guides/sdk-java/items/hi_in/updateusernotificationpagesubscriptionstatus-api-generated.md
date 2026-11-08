किसी पेज के लिए नोटिफिकेशन सक्षम या अक्षम करें। जब उपयोगकर्ता किसी पेज की सदस्यता लेते हैं, तो नई रूट कमेंट्स के लिए नोटिफिकेशन बनाए जाते हैं, और साथ ही

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | हाँ |  |
| url | string | query | हाँ |  |
| pageTitle | string | query | हाँ |  |
| subscribedOrUnsubscribed | string | path | हाँ |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`UpdateUserNotificationPageSubscriptionStatusResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UpdateUserNotificationPageSubscriptionStatusResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'updateUserNotificationPageSubscriptionStatus उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    String url = "url_example"; // String | 
    String pageTitle = "pageTitle_example"; // String | 
    String subscribedOrUnsubscribed = "subscribe"; // String | 
    String sso = "sso_example"; // String | 
    try {
      UpdateUserNotificationPageSubscriptionStatusResponse result = apiInstance.updateUserNotificationPageSubscriptionStatus(tenantId, urlId, url, pageTitle, subscribedOrUnsubscribed)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#updateUserNotificationPageSubscriptionStatus");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]