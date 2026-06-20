Предыдущие комментаторы на странице, которые в настоящий момент НЕ в сети. Отсортировано по displayName.
Используйте это после исчерпания /users/online, чтобы отобразить секцию "Members".
Курсорная пагинация по commenterName: сервер проходит по частичному {tenantId, urlId, commenterName}
индекс начиная с afterName вперёд через $gt, без затрат $skip.

## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL страницы (очищается на стороне сервера). |
| afterName | string | query | Нет | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | Нет | Критерий разрешения ничьих курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда afterName установлен, чтобы записи с одинаковыми именами не терялись. |

## Ответ

Возвращает: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импорт классов:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | Идентификатор URL страницы (очищается на стороне сервера).
    String afterName = "afterName_example"; // String | Курсор: передайте nextAfterName из предыдущего ответа.
    String afterUserId = "afterUserId_example"; // String | Критерий разрешения ничьих курсора: передайте nextAfterUserId из предыдущего ответа. Требуется, когда afterName установлен, чтобы записи с одинаковыми именами не терялись.
    try {
      PageUsersOfflineResponse result = apiInstance.getOfflineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOfflineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]