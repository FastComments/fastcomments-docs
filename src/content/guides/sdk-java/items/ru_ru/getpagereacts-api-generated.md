Custom react counts for a page, optionally with the users who reacted.

## Parameters

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| includeUserInfo | string | query | No | "public" исключает пользователей с приватными профилями, "include-private" включает их. |

## Response

Возвращает: [`GetPageReactsAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPageReactsAPIResponse.java)

## Example

[inline-code-attrs-start title = 'Пример getPageReacts'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импорт классов:
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
    
    // Настройка авторизации API-ключа: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Раскомментируйте следующую строку, чтобы установить префикс для API-ключа, например "Token" (по умолчанию null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | \"public\" исключает пользователей с приватными профилями, \"include-private\" включает их.
    try {
      GetPageReactsAPIResponse result = apiInstance.getPageReacts(tenantId, urlId)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // Исключение при вызове DefaultApi#getPageReacts
      System.err.println("Exception when calling DefaultApi#getPageReacts");
      // Код состояния: " + e.getCode()
      System.err.println("Status code: " + e.getCode());
      // Причина: " + e.getResponseBody()
      System.err.println("Reason: " + e.getResponseBody());
      // Заголовки ответа: " + e.getResponseHeaders()
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]