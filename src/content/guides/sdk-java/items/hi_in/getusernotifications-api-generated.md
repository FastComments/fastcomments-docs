## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | नहीं | यह निर्धारित करने के लिए उपयोग किया जाता है कि क्या वर्तमान पृष्ठ सब्सक्राइब है। |
| pageSize | integer | query | नहीं |  |
| afterId | string | query | नहीं |  |
| includeContext | boolean | query | नहीं |  |
| afterCreatedAt | integer | query | नहीं |  |
| unreadOnly | boolean | query | नहीं |  |
| dmOnly | boolean | query | नहीं |  |
| noDm | boolean | query | नहीं |  |
| includeTranslations | boolean | query | नहीं |  |
| includeTenantNotifications | boolean | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`GetMyNotificationsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetMyNotificationsResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'getUserNotifications उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// क्लास इम्पोर्ट करें:
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
    String urlId = "urlId_example"; // String | यह निर्धारित करने के लिए उपयोग किया जाता है कि क्या वर्तमान पृष्ठ सब्सक्राइब है।
    Integer pageSize = 56; // Integer | 
    String afterId = "afterId_example"; // String | 
    Boolean includeContext = true; // Boolean | 
    Long afterCreatedAt = 56L; // Long | 
    Boolean unreadOnly = true; // Boolean | 
    Boolean dmOnly = true; // Boolean | 
    Boolean noDm = true; // Boolean | 
    Boolean includeTranslations = true; // Boolean | 
    Boolean includeTenantNotifications = true; // Boolean | 
    String sso = "sso_example"; // String | 
    try {
      GetMyNotificationsResponse result = apiInstance.getUserNotifications(tenantId)
            .urlId(urlId)
            .pageSize(pageSize)
            .afterId(afterId)
            .includeContext(includeContext)
            .afterCreatedAt(afterCreatedAt)
            .unreadOnly(unreadOnly)
            .dmOnly(dmOnly)
            .noDm(noDm)
            .includeTranslations(includeTranslations)
            .includeTenantNotifications(includeTenantNotifications)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getUserNotifications");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]