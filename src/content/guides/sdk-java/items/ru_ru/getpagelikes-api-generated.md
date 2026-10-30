Like count for a page, optionally with the users who liked it.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| includeUserInfo | string | query | No | "\"public\" исключает пользователей с приватными профилями, \"include-private\" включает их." |

## Response

Returns: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPageLikesAPIResponse.java)

## Example

[inline-code-attrs-start title = 'Пример getPageLikes'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импортировать классы:
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
    
    // Настроить авторизацию с помощью API‑ключа: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Раскомментировать следующую строку, чтобы задать префикс для API‑ключа, например "Token" (по умолчанию null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | \"public\" исключает пользователей с приватными профилями, \"include-private\" включает их.
    try {
      GetPageLikesAPIResponse result = apiInstance.getPageLikes(tenantId, urlId)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // Исключение при вызове DefaultApi#getPageLikes
      System.err.println("Exception when calling DefaultApi#getPageLikes");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]