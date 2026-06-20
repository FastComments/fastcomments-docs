Претходни коментатори на страници који тренутно НИСУ онлајн. Сортирано по displayName.
Користите ово након што истрошите /users/online да прикажете секцију "Чланови".
Пагинација курсора по commenterName: сервер пролази делимични индекс {tenantId, urlId, commenterName} од afterName унапред помоћу $gt, без трошка $skip.

## Параметри

| Назив | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | Не | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | Не | Курсор за разрешење изједначења: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен да се уноси са истим именима не изгубе. |

## Одговор

Враћа: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример getOfflineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | Идентификатор URL странице (очишћен на серверу).
    String afterName = "afterName_example"; // String | Курсор: проследите nextAfterName из претходног одговора.
    String afterUserId = "afterUserId_example"; // String | Курсор за разрешење изједначења: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен да се уноси са истим именима не изгубе.
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

---