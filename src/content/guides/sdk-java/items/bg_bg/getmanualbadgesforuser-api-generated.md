## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| badgesUserId | string | query | No |  |
| commentId | string | query | No |  |
| sso | string | query | No |  |

## Отговор

Връща: [`GetUserManualBadgesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetUserManualBadgesResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример за getManualBadgesForUser'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импортиране на класове:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.ModerationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    ModerationApi apiInstance = new ModerationApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String badgesUserId = "badgesUserId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      GetUserManualBadgesResponse result = apiInstance.getManualBadgesForUser(tenantId)
            .badgesUserId(badgesUserId)
            .commentId(commentId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Изключение при извикване на ModerationApi#getManualBadgesForUser");
      System.err.println("Код на състоянието: " + e.getCode());
      System.err.println("Причина: " + e.getResponseBody());
      System.err.println("Хедъри на отговора: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]