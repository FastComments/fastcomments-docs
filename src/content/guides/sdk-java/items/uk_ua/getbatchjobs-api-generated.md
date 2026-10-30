The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Parameters

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| type | string | query | Ні | Лише завдання цього типу, наприклад migrate-pages. |
| status | string | query | Ні | Лише завдання у цьому статусі: requested, running, success або failed. |
| skip | integer | query | Ні | Скільки завдань пропустити, для пагінації. |

## Response

Повертає: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetBatchJobsAPIResponse.java)

## Example

[inline-code-attrs-start title = 'Приклад getBatchJobs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Налаштування авторизації API ключа: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Розкоментуйте наступний рядок, щоб встановити префікс для API ключа, e.g. "Token" (defaults to null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String type = "type_example"; // String | Лише завдання цього типу, наприклад migrate-pages.
    String status = "status_example"; // String | Лише завдання у цьому статусі: requested, running, success або failed.
    Integer skip = 56; // Integer | Скільки завдань пропустити, для пагінації.
    try {
      GetBatchJobsAPIResponse result = apiInstance.getBatchJobs(tenantId)
            .type(type)
            .status(status)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getBatchJobs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]