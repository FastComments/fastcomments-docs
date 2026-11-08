## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| domain | string | path | हाँ |  |

## प्रतिक्रिया

लौटाता है: [`GetDomainConfigResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetDomainConfigResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'getDomainConfig उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // API key authorization कॉन्फ़िगर करें: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // API key के लिए एक prefix सेट करने के लिए निम्नलिखित पंक्ति को अनकमेंट करें, जैसे "Token" (डिफ़ॉल्ट null है)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String domain = "domain_example"; // String | 
    try {
      GetDomainConfigResponse result = apiInstance.getDomainConfig(tenantId, domain)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getDomainConfig");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]