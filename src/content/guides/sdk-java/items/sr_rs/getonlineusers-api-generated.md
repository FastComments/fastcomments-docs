Тренутно-онлајн гледаоци странице: особе чија је websocket сесија управо претплаћена на страницу.
Враћа anonCount + totalCount (претплатнике у оквиру просторије, укључујући анонимне гледаоце које не набрајавамо).

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Идентификатор URL странице (очишћен на серверу). |
| afterName | string | query | No | Курсор: проследите nextAfterName из претходног одговора. |
| afterUserId | string | query | No | Резервни критеријум курсора: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен да се у случају истих имена не би изгубили уноси. |

## Одговор

Враћа: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример getOnlineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Увоз класа:
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
    String afterUserId = "afterUserId_example"; // String | Резервни критеријум курсора: проследите nextAfterUserId из претходног одговора. Обавезно када је afterName подешен да се у случају истих имена не би изгубили уноси.
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