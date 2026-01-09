## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| broadcastId | string | query | Не |  |
| isLive | boolean | query | Не |  |
| doSpamCheck | boolean | query | Не |  |
| skipDupCheck | boolean | query | Не |  |

## Отговор

Връща: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateFeedPost200Response.java)

## Пример

[inline-code-attrs-start title = 'createFeedPost Пример'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Разкоментирайте следния ред, за да зададете префикс за API ключа, напр. "Token" (по подразбиране null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    CreateFeedPostParams createFeedPostParams = new CreateFeedPostParams(); // CreateFeedPostParams | 
    String broadcastId = "broadcastId_example"; // String | 
    Boolean isLive = true; // Boolean | 
    Boolean doSpamCheck = true; // Boolean | 
    Boolean skipDupCheck = true; // Boolean | 
    try {
      CreateFeedPost200Response result = apiInstance.createFeedPost(tenantId, createFeedPostParams)
            .broadcastId(broadcastId)
            .isLive(isLive)
            .doSpamCheck(doSpamCheck)
            .skipDupCheck(skipDupCheck)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createFeedPost");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]