टेनेंट के बैच जॉब (इंपोर्ट, एक्सपोर्ट, पेज माइग्रेशन...), नवीनतम पहले।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| type | string | query | नहीं | केवल इस प्रकार के जॉब, उदाहरण के लिए migrate-pages। |
| status | string | query | नहीं | केवल इस स्थिति में जॉब: requested, running, success या failed। |
| skip | integer | query | नहीं | पेजिंग के लिए कितने जॉब छोड़ने हैं। |

## प्रतिक्रिया

रिटर्न करता है: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetBatchJobsAPIResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'getBatchJobs उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String type = "type_example"; // String | केवल इस प्रकार के जॉब, उदाहरण के लिए migrate-pages।
    String status = "status_example"; // String | केवल इस स्थिति में जॉब: requested, running, success या failed।
    Integer skip = 56; // Integer | पेजिंग के लिए कितने जॉब छोड़ने हैं।
    try {
      GetBatchJobsAPIResponse result = apiInstance.getBatchJobs(tenantId)
            .type(type)
            .status(status)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getBatchJobs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]