## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| forceRecalculate | boolean | query | Не |  |

## Одговор

Враћа: [`BulkAggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/BulkAggregateQuestionResults200Response.java)

## Пример

[inline-code-attrs-start title = 'Пример bulkAggregateQuestionResults'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импорт класа:
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
    
    // Конфигурисање овлашћења API кључа: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Откоментирајте следећу линију да бисте поставили префикс за API кључ, нпр. "Token" (подразумевано null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    BulkAggregateQuestionResultsRequest bulkAggregateQuestionResultsRequest = new BulkAggregateQuestionResultsRequest(); // BulkAggregateQuestionResultsRequest | 
    Boolean forceRecalculate = true; // Boolean | 
    try {
      BulkAggregateQuestionResults200Response result = apiInstance.bulkAggregateQuestionResults(tenantId, bulkAggregateQuestionResultsRequest)
            .forceRecalculate(forceRecalculate)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#bulkAggregateQuestionResults");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---