## Параметри

| Име | Тип | Location | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| userId | string | query | Не |  |
| limit | number | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`GetUserBadgeProgressList200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetUserBadgeProgressList200Response.java)

## Пример

[inline-code-attrs-start title = 'Пример за getUserBadgeProgressList'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Uncomment the following line to set a prefix for the API key, e.g. "Token" (по подразбиране null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String userId = "userId_example"; // String | 
    Double limit = 3.4D; // Double | 
    Double skip = 3.4D; // Double | 
    try {
      GetUserBadgeProgressList200Response result = apiInstance.getUserBadgeProgressList(tenantId)
            .userId(userId)
            .limit(limit)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getUserBadgeProgressList");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---