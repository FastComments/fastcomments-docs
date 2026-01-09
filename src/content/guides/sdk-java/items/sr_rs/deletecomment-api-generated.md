## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| id | string | path | Да |  |
| contextUserId | string | query | Не |  |
| isLive | boolean | query | Не |  |

## Одговор

Враћа: [`DeleteComment200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/DeleteComment200Response.java)

## Пример

[inline-code-attrs-start title = 'deleteComment Пример'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Конфигуришите овлашћење API кључа: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Откоментирајте следећи ред да бисте поставили префикс за API кључ, нпр. "Token" (подразумевано null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String id = "id_example"; // String | 
    String contextUserId = "contextUserId_example"; // String | 
    Boolean isLive = true; // Boolean | 
    try {
      DeleteComment200Response result = apiInstance.deleteComment(tenantId, id)
            .contextUserId(contextUserId)
            .isLive(isLive)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#deleteComment");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]