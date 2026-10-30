Recent comments in exactly the shape webhook deliveries use, for building integrations (for example Zapier
sample data). Every event delivers the same comment object, so `event` only has to be valid.

## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| event | string | query | No |  |
| limit | number | query | No |  |

## Відповідь

Повертає: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetWebhookSamplePayloadsResponse.java)

## Приклад

[inline-code-attrs-start title = 'getWebhookSamplePayloads Приклад'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    WebhookEventName event = WebhookEventName.fromValue("comment-created"); // WebhookEventName | 
    Double limit = 3.4D; // Double | 
    try {
      GetWebhookSamplePayloadsResponse result = apiInstance.getWebhookSamplePayloads(tenantId)
            .event(event)
            .limit(limit)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // Exception when calling DefaultApi#getWebhookSamplePayloads
      System.err.println("Exception when calling DefaultApi#getWebhookSamplePayloads");
      System.err.println("Status code: " + e.getCode()); // Status code: 
      System.err.println("Reason: " + e.getResponseBody()); // Reason: 
      System.err.println("Response headers: " + e.getResponseHeaders()); // Response headers: 
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---