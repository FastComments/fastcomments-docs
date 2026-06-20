## Параметри

| Име | Тип | Локација | Потребно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Не |  |

## Одговор

Враћа: [`BulkCreateHashTagsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/BulkCreateHashTagsResponse.java)

## Пример

[inline-code-attrs-start title = 'addHashTagsBulk Пример'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Увези класе:
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
    
    // Конфигуриши ауторизацију API кључа: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Откоментирај следећи ред да подесиш префикс за API кључ, нпр. "Token" (подразумевано је null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    BulkCreateHashTagsBody bulkCreateHashTagsBody = new BulkCreateHashTagsBody(); // BulkCreateHashTagsBody | 
    try {
      BulkCreateHashTagsResponse result = apiInstance.addHashTagsBulk()
            .tenantId(tenantId)
            .bulkCreateHashTagsBody(bulkCreateHashTagsBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#addHashTagsBulk");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]