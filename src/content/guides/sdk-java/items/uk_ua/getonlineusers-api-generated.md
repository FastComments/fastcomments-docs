Користувачі, які наразі онлайн на сторінці: люди, чиї websocket session підписані на сторінку прямо зараз.
Повертає anonCount + totalCount (підписники кімнати в цілому, включно з анонімними глядачами, яких ми не перелічуємо).

## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Ідентифікатор URL сторінки (очищено на сервері). |
| afterName | string | query | No | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | No | Тібрейкер курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язково, коли встановлено afterName, щоб однакові імена не призводили до пропуску записів. |

## Відповідь

Повертає: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Приклад

[inline-code-attrs-start title = 'Приклад getOnlineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Імпорт класів:
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
    String urlId = "urlId_example"; // String | Ідентифікатор URL сторінки (очищено на сервері).
    String afterName = "afterName_example"; // String | Курсор: передайте nextAfterName з попередньої відповіді.
    String afterUserId = "afterUserId_example"; // String | Тібрейкер курсора: передайте nextAfterUserId з попередньої відповіді. Обов'язково, коли встановлено afterName, щоб однакові імена не призводили до пропуску записів.
    try {
      PageUsersOnlineResponse result = apiInstance.getOnlineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOnlineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---