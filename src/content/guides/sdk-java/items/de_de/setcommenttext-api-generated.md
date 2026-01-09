## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| commentId | string | path | Ja |  |
| broadcastId | string | query | Ja |  |
| editKey | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`SetCommentText200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SetCommentText200Response.java)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für setCommentText'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Klassen importieren:
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
    String commentId = "commentId_example"; // String | 
    String broadcastId = "broadcastId_example"; // String | 
    CommentTextUpdateRequest commentTextUpdateRequest = new CommentTextUpdateRequest(); // CommentTextUpdateRequest | 
    String editKey = "editKey_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      SetCommentText200Response result = apiInstance.setCommentText(tenantId, commentId, broadcastId, commentTextUpdateRequest)
            .editKey(editKey)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#setCommentText");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---