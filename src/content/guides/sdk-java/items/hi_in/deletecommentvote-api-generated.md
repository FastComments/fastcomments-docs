## पैरामीटर

| नाम | प्रकार | स्थान | आवश्यक | विवरण |
|------|------|----------|----------|-------------|
| tenantId | string | path | हाँ |  |
| commentId | string | path | हाँ |  |
| voteId | string | path | हाँ |  |
| urlId | string | query | हाँ |  |
| broadcastId | string | query | हाँ |  |
| editKey | string | query | नहीं |  |
| sso | string | query | नहीं |  |

## प्रतिक्रिया

रिटर्न: [`VoteDeleteResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/VoteDeleteResponse.java)

## उदाहरण

[inline-code-attrs-start title = 'deleteCommentVote उदाहरण'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// क्लास इम्पोर्ट करें:
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
    String voteId = "voteId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    String broadcastId = "broadcastId_example"; // String | 
    String editKey = "editKey_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      VoteDeleteResponse result = apiInstance.deleteCommentVote(tenantId, commentId, voteId, urlId, broadcastId)
            .editKey(editKey)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#deleteCommentVote");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]