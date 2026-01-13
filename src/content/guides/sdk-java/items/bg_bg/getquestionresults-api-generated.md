## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Да |  |
| urlId | string | query | Не |  |
| userId | string | query | Не |  |
| startDate | string | query | Не |  |
| questionId | string | query | Не |  |
| questionIds | string | query | Не |  |
| skip | number | query | Не |  |

## Отговор

Връща: [`GetQuestionResults200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetQuestionResults200Response.java)

## Пример

[inline-code-attrs-start title = 'Пример за getQuestionResults'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | 
    String userId = "userId_example"; // String | 
    String startDate = "startDate_example"; // String | 
    String questionId = "questionId_example"; // String | 
    String questionIds = "questionIds_example"; // String | 
    Double skip = 3.4D; // Double | 
    try {
      GetQuestionResults200Response result = apiInstance.getQuestionResults(tenantId)
            .urlId(urlId)
            .userId(userId)
            .startDate(startDate)
            .questionId(questionId)
            .questionIds(questionIds)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getQuestionResults");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---