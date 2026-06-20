Предыдущие комментаторы на странице, которые в настоящее время НЕ в сети. Отсортированы по displayName.
Используйте это после обращения к /users/online, чтобы отобразить раздел «Members».
Курсорная пагинация по commenterName: сервер проходит по частичному индексу {tenantId, urlId, commenterName}
индекс от afterName вперёд с использованием $gt — без затрат на $skip.

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL страницы (очищается на стороне сервера). |
| afterName | string | query | No | Курсор: передайте nextAfterName из предыдущего ответа. |
| afterUserId | string | query | No | Тайбрейкер курсора: передайте nextAfterUserId из предыдущего ответа. Обязательно, когда afterName задан, чтобы при совпадении имён записи не терялись. |

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
    String afterUserId = "afterUserId_example"; // String | Тайбрейкер курсора: передайте nextAfterUserId из предыдущего ответа. Обязательно, когда afterName задан, чтобы при совпадении имён записи не терялись.
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