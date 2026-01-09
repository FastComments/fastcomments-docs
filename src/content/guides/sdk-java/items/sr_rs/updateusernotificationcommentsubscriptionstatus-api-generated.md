---
Омогућите или онемогућите обавештења за одређени коментар.

## Параметри

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| notificationId | string | path | Yes |  |
| optedInOrOut | string | path | Yes |  |
| commentId | string | query | Yes |  |
| sso | string | query | No |  |

## Одговор

Враћа: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UpdateUserNotificationStatus200Response.java)

## Пример

[inline-code-attrs-start title = 'updateUserNotificationCommentSubscriptionStatus Пример'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String notificationId = "notificationId_example"; // String | 
    String optedInOrOut = "in"; // String | 
    String commentId = "commentId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      UpdateUserNotificationStatus200Response result = apiInstance.updateUserNotificationCommentSubscriptionStatus(tenantId, notificationId, optedInOrOut, commentId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#updateUserNotificationCommentSubscriptionStatus");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---