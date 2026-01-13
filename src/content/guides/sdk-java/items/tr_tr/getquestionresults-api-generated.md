## Parametreler

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Evet |  |
| urlId | string | query | Hayır |  |
| userId | string | query | Hayır |  |
| startDate | string | query | Hayır |  |
| questionId | string | query | Hayır |  |
| questionIds | string | query | Hayır |  |
| skip | number | query | Hayır |  |

## Yanıt

Döndürür: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetQuestionResults200Response.java)

## Örnek

[inline-code-attrs-start title = 'getQuestionResults Örneği'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String userId = "userId_example"; // String | 
    String startDate = "startDate_example"; // String | 
    String questionId = "questionId_example"; // String | 
    String questionIds = "questionIds_example"; // String | 
    Double skip = 3.4D; // Double | 
    try {
      GetQuestionResults200Response result = apiInstance.getQuestionResults(tenantId)
            .urlId(urlId)
            .userId(userId)
            .startDate(startDate)
            .questionId(questionId)
            .questionIds(questionIds)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getQuestionResults");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]