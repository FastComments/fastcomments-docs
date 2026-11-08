किसी टेनेंट के लिए बल्क यूज़र जानकारी। दिए गए userIds के लिए, User / SSOUser से डिस्प्ले जानकारी लौटाता है।
कमेंट विजेट द्वारा उपयोग किया जाता है ताकि प्रेज़ेंस इवेंट के माध्यम से अभी दिखाई देने वाले यूज़र्स को समृद्ध (enrich) किया जा सके।
कोई पेज संदर्भ नहीं: गोपनीयता समान रूप से लागू की जाती है (प्राइवेट प्रोफाइल मास्क कर दी जाती हैं)।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| ids | string | query | हाँ | कॉमा से अलग किए गए userIds। |

## प्रतिक्रिया

लौटाता है: [`PageUsersInfoResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersInfoResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'getUsersInfo उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String ids = "ids_example"; // String | कॉमा से अलग किए गए userIds।
    try {
      PageUsersInfoResponse result = apiInstance.getUsersInfo(tenantId, ids)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getUsersInfo");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]