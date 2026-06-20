Претходни коментатори на страници који тренутно нису на мрежи. Сортирано по displayName.
Користите ово након што исцрпите /users/online да бисте приказали секцију "Чланови".
Cursor пагинација по commenterName: сервер користи делимични индекс {tenantId, urlId, commenterName} и пролази од afterName унапред преко $gt, без трошкова $skip.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL странице (очишћен на серверској страни). |
| afterName | string | query | Не | Курсор: прослиједите nextAfterName из претходног одговора. |
| afterUserId | string | query | Не | Курсор - разрешење изједначавања: прослиједите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен да би везе по имену не изгубиле уносе. |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Пример

[inline-code-attrs-start title = 'getOfflineUsers Пример'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String afterUserId = "afterUserId_example"; // String | Курсор - разрешење изједначавања: прослиједите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен да би везе по имену не изгубиле уносе.
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