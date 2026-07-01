## Параметры

| Имя | Тип | Местоположение | Обязательно | Описание |
|------|------|----------------|-------------|----------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Ответ

Возвращает: [`ModerationAPIGetLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationAPIGetLogsResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример getLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
// Импорт классов:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.ModerationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    ModerationApi apiInstance = new ModerationApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      ModerationAPIGetLogsResponse result = apiInstance.getLogs(tenantId, commentId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#getLogs");
      // Исключение при вызове ModerationApi#getLogs
      System.err.println("Status code: " + e.getCode());
      // Код статуса: 
      System.err.println("Reason: " + e.getResponseBody());
      // Причина: 
      System.err.println("Response headers: " + e.getResponseHeaders());
      // Заголовки ответа: 
      e.printStackTrace();
    }
  }
}
[inline-code-end]