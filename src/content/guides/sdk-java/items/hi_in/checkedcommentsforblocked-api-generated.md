## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| commentIds | string | query | हाँ | कमेंट आईडी की कॉमा से अलग की गई सूची। |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न करता है: [`CheckBlockedCommentsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CheckBlockedCommentsResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'checkedCommentsForBlocked उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// क्लासेस इम्पोर्ट करें:
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
    String commentIds = "commentIds_example"; // String | कमेंट आईडी की कॉमा से अलग की गई सूची।
    String sso = "sso_example"; // String | 
    try {
      CheckBlockedCommentsResponse result = apiInstance.checkedCommentsForBlocked(tenantId, commentIds)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#checkedCommentsForBlocked");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]