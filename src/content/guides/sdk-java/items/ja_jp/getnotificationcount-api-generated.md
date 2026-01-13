## パラメーター

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| urlId | string | query | いいえ |  |
| fromCommentId | string | query | いいえ |  |
| viewed | boolean | query | いいえ |  |
| type | string | query | いいえ |  |

## レスポンス

戻り値: [`GetNotificationCount200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetNotificationCount200Response.java)

## 例

[inline-code-attrs-start title = 'getNotificationCount の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// インポートするクラス:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // APIキー認証の設定: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // APIキーのプレフィックスを設定するには、次の行のコメントを外してください。例: "Token"（デフォルトは null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String userId = "userId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    String fromCommentId = "fromCommentId_example"; // String | 
    Boolean viewed = true; // Boolean | 
    String type = "type_example"; // String | 
    try {
      GetNotificationCount200Response result = apiInstance.getNotificationCount(tenantId)
            .userId(userId)
            .urlId(urlId)
            .fromCommentId(fromCommentId)
            .viewed(viewed)
            .type(type)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getNotificationCount");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]