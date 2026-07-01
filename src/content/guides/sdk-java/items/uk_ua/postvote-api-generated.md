## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|--------------|--------------|------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| direction | string | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Відповідь

Повертає: [`VoteResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/VoteResponse.java)

## Приклад

[inline-code-attrs-start title = 'postVote Приклад'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Імпорт класів:
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
    String direction = "direction_example"; // String |
    String broadcastId = "broadcastId_example"; // String |
    String sso = "sso_example"; // String |
    try {
      VoteResponse result = apiInstance.postVote(tenantId, commentId)
            .direction(direction)
            .broadcastId(broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#postVote");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]