## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| urlIdWS | string | query | Так |  |
| userIds | string | query | Так |  |

## Відповідь

Повертає: [`GetUserPresenceStatuses200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetUserPresenceStatuses200Response.java)

## Приклад

[inline-code-attrs-start title = 'Приклад getUserPresenceStatuses'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlIdWS = "urlIdWS_example"; // String | 
    String userIds = "userIds_example"; // String | 
    try {
      GetUserPresenceStatuses200Response result = apiInstance.getUserPresenceStatuses(tenantId, urlIdWS, userIds)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getUserPresenceStatuses");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---