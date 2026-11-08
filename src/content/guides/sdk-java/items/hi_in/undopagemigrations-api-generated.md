एक बैच जॉब कतार में जोड़ता है जो एक migrate-pages जॉब के हर माइग्रेशन को उल्टा करता है, नवीनतम पहले। 90 दिनों तक उपलब्ध।
माइग्रेशन के बाद बदले गए रिकॉर्ड को नहीं छेड़ा जाता, इसलिए बाद का कोई संपादन कभी अधिलेखित नहीं होता।
पुनर्स्थापित/छोड़े गए की गिनती के लिए GET /api/v1/batch-jobs/{batchJobId} को पोल करें।

## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | query | हाँ |  |
| undoId | string | path | हाँ |  |

## प्रतिक्रिया

रिटर्न: [`QueuePageMigrationJobAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/QueuePageMigrationJobAPIResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'undoPageMigrations उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String undoId = "undoId_example"; // String | 
    try {
      QueuePageMigrationJobAPIResponse result = apiInstance.undoPageMigrations(tenantId, undoId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#undoPageMigrations");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]