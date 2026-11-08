## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| id | string | path | हाँ |  |
| updateComments | string | query | नहीं |  |

## प्रतिक्रिया

लौटाता है: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIEmptyResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'replaceTenantUser उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// क्लासेस इम्पोर्ट करें:
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
    
    // API की ऑथराइज़ेशन कॉन्फ़िगर करें: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // API की के लिए प्रीफ़िक्स सेट करने के लिए निम्नलिखित लाइन को अनकमेंट करें, जैसे "Token" (डिफ़ॉल्ट null है)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String id = "id_example"; // String | 
    ReplaceTenantUserBody replaceTenantUserBody = new ReplaceTenantUserBody(); // ReplaceTenantUserBody | 
    String updateComments = "updateComments_example"; // String | 
    try {
      APIEmptyResponse result = apiInstance.replaceTenantUser(tenantId, id, replaceTenantUserBody)
            .updateComments(updateComments)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("DefaultApi#replaceTenantUser कॉल करते समय अपवाद");
      System.err.println("स्टेटस कोड: " + e.getCode());
      System.err.println("कारण: " + e.getResponseBody());
      System.err.println("प्रतिक्रिया हेडर: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]