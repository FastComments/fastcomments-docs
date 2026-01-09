הפעל או השבת התראות עבור תגובה ספציפית.

## פרמטרים

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | כן |  |
| notificationId | string | path | כן |  |
| optedInOrOut | string | path | כן |  |
| commentId | string | query | כן |  |
| sso | string | query | לא |  |

## תגובה

מחזיר: [`UpdateUserNotificationStatus200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/UpdateUserNotificationStatus200Response.java)

## דוגמה

[inline-code-attrs-start title = 'דוגמה: updateUserNotificationCommentSubscriptionStatus'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// ייבוא מחלקות:
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
    String tenantId = "tenantId_example"; // מחרוזת | 
    String notificationId = "notificationId_example"; // מחרוזת | 
    String optedInOrOut = "in"; // מחרוזת | 
    String commentId = "commentId_example"; // מחרוזת | 
    String sso = "sso_example"; // מחרוזת | 
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