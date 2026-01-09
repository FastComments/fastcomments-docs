## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| commentId | string | path | はい |  |
| voteId | string | path | はい |  |
| urlId | string | query | はい |  |
| broadcastId | string | query | はい |  |
| editKey | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`DeleteCommentVote200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/DeleteCommentVote200Response.java)

## 例

[inline-code-attrs-start title = 'deleteCommentVote の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// クラスをインポート
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
    String tenantId = "tenantId_example"; // 文字列 | 
    String commentId = "commentId_example"; // 文字列 | 
    String voteId = "voteId_example"; // 文字列 | 
    String urlId = "urlId_example"; // 文字列 | 
    String broadcastId = "broadcastId_example"; // 文字列 | 
    String editKey = "editKey_example"; // 文字列 | 
    String sso = "sso_example"; // 文字列 | 
    try {
      DeleteCommentVote200Response result = apiInstance.deleteCommentVote(tenantId, commentId, voteId, urlId, broadcastId)
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

---