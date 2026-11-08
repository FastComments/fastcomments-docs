किसी पेज के लिए कस्टम रिएक्ट काउंट्स, वैकल्पिक रूप से रिएक्ट करने वाले उपयोगकर्ताओं के साथ।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| urlId | string | query | हाँ |  |
| includeUserInfo | string | query | नहीं | "public" प्राइवेट प्रोफ़ाइल वाले उपयोगकर्ताओं को छोड़ देता है, "include-private" उन्हें शामिल करता है। |

## प्रतिक्रिया

लौटाता है: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPageReactsAPIResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'getPageReacts उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Configure API key authorization: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Uncomment the following line to set a prefix for the API key, e.g. "Token" (defaults to null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | \"public\" प्राइवेट प्रोफ़ाइल वाले उपयोगकर्ताओं को छोड़ देता है, \"include-private\" उन्हें शामिल करता है।
    try {
      GetPageReactsAPIResponse result = apiInstance.getPageReacts(tenantId, urlId)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getPageReacts");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]