## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| spam | boolean | query | No |  |
| permNotSpam | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Одговор

Враћа: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIEmptyResponse.java)

## Пример

[inline-code-attrs-start title = 'postSetCommentSpamStatus Primer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Увези класе:
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
    String commentId = "commentId_example"; // String |
    Boolean spam = true; // Boolean |
    Boolean permNotSpam = true; // Boolean |
    String broadcastId = "broadcastId_example"; // String |
    String sso = "sso_example"; // String |
    try {
      APIEmptyResponse result = apiInstance.postSetCommentSpamStatus(tenantId, commentId)
            .spam(spam)
            .permNotSpam(permNotSpam)
            .broadcastId(broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Изузетак приликом позивања ModerationApi#postSetCommentSpamStatus");
      System.err.println("Статус код: " + e.getCode());
      System.err.println("Разлог: " + e.getResponseBody());
      System.err.println("Заглавља одговора: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]