Тренутно онлајн посматрачи странице: људи чија је websocket сесија тренутно претплаћена на страницу.
Враћа anonCount + totalCount (сви претплатници у соби, укључујући анонимне посматраче које не набрајемо).

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверској страни). |
| afterName | string | query | No | Курсор: прослиједите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Курсор за разликовање: прослиједите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен како записи са истим именом не би били изгубљени. |

## Одговор

Враћа: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Пример

[inline-code-attrs-start title = 'getOnlineUsers Пример'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Увези класе:
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
    String urlId = "urlId_example"; // String | Идентификатор URL странице (очишћен на серверској страни).
    String afterName = "afterName_example"; // String | Курсор: прослиједите nextAfterName из претходног одговора.
    String afterUserId = "afterUserId_example"; // String | Курсор за разликовање: прослиједите nextAfterUserId из претходног одговора. Обавезно када је afterName постављен како записи са истим именом не би били изгубљени.
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