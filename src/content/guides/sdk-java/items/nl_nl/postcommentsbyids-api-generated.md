## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| sso | string | query | Nee |  |

## Response

Returns: [`ModerationAPIChildCommentsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationAPIChildCommentsResponse.java)

## Example

[inline-code-attrs-start title = 'postCommentsByIds Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importeer klassen:
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
    CommentsByIdsParams commentsByIdsParams = new CommentsByIdsParams(); // CommentsByIdsParams | 
    String sso = "sso_example"; // String | 
    try {
      ModerationAPIChildCommentsResponse result = apiInstance.postCommentsByIds(tenantId, commentsByIdsParams)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exceptie bij het aanroepen van ModerationApi#postCommentsByIds");
      System.err.println("Statuscode: " + e.getCode());
      System.err.println("Reden: " + e.getResponseBody());
      System.err.println("Responsheaders: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]