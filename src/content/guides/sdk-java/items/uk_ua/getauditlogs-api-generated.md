## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| limit | number | query | Ні |  |
| skip | number | query | Ні |  |
| order | string | query | Ні |  |
| after | number | query | Ні |  |
| before | number | query | Ні |  |

## Response

Повертає: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogs200Response.java)

## Example

[inline-code-attrs-start title = 'Приклад getAuditLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Імпортувати класи:
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
    
    // Налаштування авторизації за API-ключем: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Розкоментуйте наступний рядок, щоб встановити префікс для API-ключа, наприклад "Token" (за замовчуванням null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    Double limit = 3.4D; // Double | 
    Double skip = 3.4D; // Double | 
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | 
    Double after = 3.4D; // Double | 
    Double before = 3.4D; // Double | 
    try {
      GetAuditLogs200Response result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]