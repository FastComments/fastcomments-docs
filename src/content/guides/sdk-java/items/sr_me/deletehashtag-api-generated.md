## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tag | string | path | Yes |  |
| tenantId | string | query | No |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIEmptyResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример deleteHashTag'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импорт класе:
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
    
    // Подесите ауторизацију помоћу API кључа: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Откоментаришите следећи ред да бисте поставили префикс за API кључ, нпр. "Token" (подразумевано null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tag = "tag_example"; // String | 
    String tenantId = "tenantId_example"; // String | 
    DeleteHashTagRequestBody deleteHashTagRequestBody = new DeleteHashTagRequestBody(); // DeleteHashTagRequestBody | 
    try {
      APIEmptyResponse result = apiInstance.deleteHashTag(tag)
            .tenantId(tenantId)
            .deleteHashTagRequestBody(deleteHashTagRequestBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#deleteHashTag");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]