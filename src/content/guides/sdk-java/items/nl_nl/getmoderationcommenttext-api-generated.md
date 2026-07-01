## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|--------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Respons

Retourneert: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetCommentTextResponse.java)

## Voorbeeld

[inline-code-attrs-start title = 'Voorbeeld getModerationCommentText'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Klassen importeren:
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
    String sso = "sso_example"; // String | 
    try {
      GetCommentTextResponse result = apiInstance.getModerationCommentText(tenantId, commentId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Uitzondering bij het aanroepen van ModerationApi#getModerationCommentText");
      System.err.println("Statuscode: " + e.getCode());
      System.err.println("Reden: " + e.getResponseBody());
      System.err.println("Responsheaders: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]