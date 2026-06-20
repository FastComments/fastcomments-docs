Минулі коментатори на сторінці, які ЗАРАЗ не в мережі. Відсортовано за displayName.
Використовуйте це після вичерпання /users/online, щоб відобразити секцію «Учасники».
Курсорна пагінація за commenterName: сервер проходить частковий індекс {tenantId, urlId, commenterName} починаючи з afterName уперед за допомогою $gt, без витрат на $skip.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так | Page URL identifier (cleaned server-side). |
| afterName | string | query | Ні | Курсор: передайте nextAfterName з попередньої відповіді. |
| afterUserId | string | query | Ні | Курсор для розв'язання спорів: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли afterName встановлено, щоб записи з однаковими іменами не втрачалися. |

## Відповідь

Повертає: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Приклад

[inline-code-attrs-start title = 'Приклад getOfflineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | Ідентифікатор URL сторінки (очищається на сервері).
    String afterName = "afterName_example"; // String | Курсор: передайте nextAfterName з попередньої відповіді.
    String afterUserId = "afterUserId_example"; // String | Курсор-тайбрейкер: передайте nextAfterUserId з попередньої відповіді. Потрібно, коли afterName встановлено, щоб записи з однаковими іменами не випадали.
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