पेज पर उन पिछले टिप्पणीकारों की सूची जो इस समय ऑनलाइन नहीं हैं। displayName के अनुसार क्रमबद्ध।
"Members" सेक्शन रेंडर करने के लिए /users/online का उपयोग पूरा करने के बाद इसका उपयोग करें।
commenterName पर कर्सर पेजिनेशन: सर्वर आंशिक {tenantId, urlId, commenterName}
इंडेक्स को afterName से $gt के माध्यम से आगे बढ़ाता है, कोई $skip लागत नहीं।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| urlId | string | query | हाँ | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)। |
| afterName | string | query | नहीं | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें। |
| afterUserId | string | query | नहीं | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। afterName सेट होने पर आवश्यक ताकि समान नाम वाली प्रविष्टियाँ न छूटें। |

## प्रतिक्रिया

रिटर्न: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'getOfflineUsers उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | पेज URL पहचानकर्ता (सर्वर-साइड पर साफ़ किया गया)।
    String afterName = "afterName_example"; // String | कर्सर: पिछली प्रतिक्रिया से nextAfterName पास करें।
    String afterUserId = "afterUserId_example"; // String | कर्सर टाइब्रेकर: पिछली प्रतिक्रिया से nextAfterUserId पास करें। afterName सेट होने पर आवश्यक ताकि समान नाम वाली प्रविष्टियाँ न छूटें।
    try {
      PageUsersOfflineResponse result = apiInstance.getOfflineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOfflineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]