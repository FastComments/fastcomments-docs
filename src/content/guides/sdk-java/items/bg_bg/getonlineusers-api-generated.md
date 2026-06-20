Понастоящем онлайн зрители на страница: хора, чиито websocket сесии са абонирани за страницата в момента.
Връща anonCount + totalCount (абонати в рамките на стаята, включително анонимните зрители, които не изброяваме).

## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | path | Да |  |
| urlId | string | query | Да | Идентификатор на URL на страницата (почистен от страна на сървъра). |
| afterName | string | query | Не | Курсор: предайте nextAfterName от предишния отговор. |
| afterUserId | string | query | Не | Курсор за разрешаване на равенства: предайте nextAfterUserId от предишния отговор. Задължително, когато afterName е зададено, за да не се изпускат записи при равни имена. |

## Отговор

Връща: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример за getOnlineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импортиране на класове:
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
    String urlId = "urlId_example"; // String | Идентификатор на URL на страницата (почистен от страна на сървъра).
    String afterName = "afterName_example"; // String | Курсор: предайте nextAfterName от предишния отговор.
    String afterUserId = "afterUserId_example"; // String | Курсор за разрешаване на равенства: предайте nextAfterUserId от предишния отговор. Задължително, когато afterName е зададено, за да не се изпускат записи при равни имена.
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