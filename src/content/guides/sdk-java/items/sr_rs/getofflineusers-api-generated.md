Прошли коментатори на страници који тренутно нису онлајн. Сортирано по displayName.
Користите ово након исцрпљивања /users/online да би се приказао одељак "Чланови".
Cursor pagination on commenterName: server walks the partial {tenantId, urlId, commenterName}
index from afterName forward via $gt, no $skip cost.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Курсор за решавање нерешености: проследите nextAfterUserId из претходног одговора. Потребно када је afterName постављен како не би дошло до губитка записа при истоветним именима. |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Пример

[inline-code-attrs-start title = 'getOfflineUsers Пример'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Увезите класе:
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
    String urlId = "urlId_example"; // String | Идентификатор URL странице (очишћен на серверу).
    String afterName = "afterName_example"; // String | Курсор: проследите nextAfterName из претходног одговора.
    String afterUserId = "afterUserId_example"; // String | Курсор за решавање нерешености: проследите nextAfterUserId из претходног одговора. Потребно када је afterName постављен како не би дошло до губитка записа при истоветним именима.
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