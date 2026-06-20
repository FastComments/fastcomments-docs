## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| commentId | string | query | Ні |  |
| externalId | string | query | Ні |  |
| eventType | string | query | Ні |  |
| type | string | query | Ні |  |
| domain | string | query | Ні |  |
| attemptCountGT | number | query | Ні |  |

## Відповідь

Повертає: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPendingWebhookEventCountResponse.java)

## Приклад

[inline-code-attrs-start title = 'Приклад getPendingWebhookEventCount'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Імпортувати класи:
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
    
    // Налаштування авторизації API ключа: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Розкоментуйте наступний рядок, щоб встановити префікс для API ключа, наприклад "Token" (за замовчуванням null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    String externalId = "externalId_example"; // String | 
    String eventType = "eventType_example"; // String | 
    String type = "type_example"; // String | 
    String domain = "domain_example"; // String | 
    Double attemptCountGT = 3.4D; // Double | 
    try {
      GetPendingWebhookEventCountResponse result = apiInstance.getPendingWebhookEventCount(tenantId)
            .commentId(commentId)
            .externalId(externalId)
            .eventType(eventType)
            .type(type)
            .domain(domain)
            .attemptCountGT(attemptCountGT)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getPendingWebhookEventCount");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---