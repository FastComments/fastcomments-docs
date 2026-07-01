## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| spam | boolean | query | No |  |
| permNotSpam | boolean | query | No |  |
| broadcastId | string | query | No |  |
| sso | string | query | No |  |

## Antwort

Rückgabe: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIEmptyResponse.java)

## Beispiel

[inline-code-attrs-start title = 'postSetCommentSpamStatus Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Klassen importieren:
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
      System.err.println("Ausnahme beim Aufruf von ModerationApi#postSetCommentSpamStatus");
      System.err.println("Statuscode: " + e.getCode());
      System.err.println("Grund: " + e.getResponseBody());
      System.err.println("Antwort-Header: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]