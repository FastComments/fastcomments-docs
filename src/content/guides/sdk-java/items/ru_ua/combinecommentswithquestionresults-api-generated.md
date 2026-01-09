## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| questionId | string | query | Нет |  |
| questionIds | array | query | Нет |  |
| urlId | string | query | Нет |  |
| startDate | string | query | Нет |  |
| forceRecalculate | boolean | query | Нет |  |
| minValue | number | query | Нет |  |
| maxValue | number | query | Нет |  |
| limit | number | query | Нет |  |

## Ответ

Возвращает: [`CombineCommentsWithQuestionResults200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CombineCommentsWithQuestionResults200Response.java)

## Пример

[inline-code-attrs-start title = 'Пример combineCommentsWithQuestionResults'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Настроить авторизацию по API-ключу: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Раскомментируйте следующую строку, чтобы установить префикс для API-ключа, например "Token" (по умолчанию null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String questionId = "questionId_example"; // String | 
    List<String> questionIds = Arrays.asList(); // List<String> | 
    String urlId = "urlId_example"; // String | 
    OffsetDateTime startDate = OffsetDateTime.now(); // OffsetDateTime | 
    Boolean forceRecalculate = true; // Boolean | 
    Double minValue = 3.4D; // Double | 
    Double maxValue = 3.4D; // Double | 
    Double limit = 3.4D; // Double | 
    try {
      CombineCommentsWithQuestionResults200Response result = apiInstance.combineCommentsWithQuestionResults(tenantId)
            .questionId(questionId)
            .questionIds(questionIds)
            .urlId(urlId)
            .startDate(startDate)
            .forceRecalculate(forceRecalculate)
            .minValue(minValue)
            .maxValue(maxValue)
            .limit(limit)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#combineCommentsWithQuestionResults");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]