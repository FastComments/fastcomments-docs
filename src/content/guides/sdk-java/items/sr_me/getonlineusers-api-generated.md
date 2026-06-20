Тренутно онлајн посматрачи странице: особе чија је websocket сесија тренутно претплаћена на страницу.
Враћа anonCount + totalCount (сви претплатници у простору, укључујући анонимне посматраче које не набрајамо).

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL-а странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: пошаљите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Курсор за решавање изједначења: пошаљите nextAfterUserId из претходног одговора. Обавезно када је afterName постављено да би везе по имену не изоставиле уносе. |

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
    String urlId = "urlId_example"; // String | Идентификатор URL-а странице (очишћен на серверу).
    String afterName = "afterName_example"; // String | Курсор: пошаљите nextAfterName из претходног одговора.
    String afterUserId = "afterUserId_example"; // String | Курсор за решавање изједначења: пошаљите nextAfterUserId из претходног одговора. Обавезно када је afterName постављено да би везе по имену не изоставиле уносе.
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