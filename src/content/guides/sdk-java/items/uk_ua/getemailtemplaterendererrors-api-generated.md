## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| id | string | path | Так |  |
| skip | number | query | Ні |  |

## Відповідь

Повертає: [`GetEmailTemplateRenderErrors200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetEmailTemplateRenderErrors200Response.java)

## Приклад

[inline-code-attrs-start title = 'Приклад getEmailTemplateRenderErrors'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Імпорт класів:
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
    
    // Налаштування авторизації API ключем: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Розкоментуйте наступний рядок, щоб встановити префікс для API ключа, наприклад "Token" (за замовчуванням null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String id = "id_example"; // String | 
    Double skip = 3.4D; // Double | 
    try {
      GetEmailTemplateRenderErrors200Response result = apiInstance.getEmailTemplateRenderErrors(tenantId, id)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getEmailTemplateRenderErrors");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]