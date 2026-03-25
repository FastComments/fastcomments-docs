## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| state | number | query | Не |  |
| skip | number | query | Не |  |
| limit | number | query | Не |  |

## Одговор

Враћа: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetTickets200Response.java)

## Пример

[inline-code-attrs-start title = 'getTickets пример'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Увоз класа:
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
    
    // Конфигуришите ауторизацију API кључа: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Откоментујте следећу линију да бисте поставили префикс за API кључ, нпр. "Token" (подразумевано null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String userId = "userId_example"; // String | 
    Double state = 3.4D; // Double | 
    Double skip = 3.4D; // Double | 
    Double limit = 3.4D; // Double | 
    try {
      GetTickets200Response result = apiInstance.getTickets(tenantId)
            .userId(userId)
            .state(state)
            .skip(skip)
            .limit(limit)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getTickets");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---