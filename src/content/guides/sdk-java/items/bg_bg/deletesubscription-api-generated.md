## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| id | string | path | Yes |  |
| userId | string | query | No |  |

## Отговор

Връща: [`DeleteSubscriptionAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/DeleteSubscriptionAPIResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример за deleteSubscription'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импортиране на класове:
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
    
    // Конфигуриране на удостоверяване с API ключ: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Разкоментирайте следния ред, за да зададете префикс за API ключа, например "Token" (по подразбиране null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String id = "id_example"; // String | 
    String userId = "userId_example"; // String | 
    try {
      DeleteSubscriptionAPIResponse result = apiInstance.deleteSubscription(tenantId, id)
            .userId(userId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#deleteSubscription");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]