## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| urlId | string | query | いいえ |  |
| fromCommentId | string | query | いいえ |  |
| viewed | boolean | query | いいえ |  |
| type | string | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetNotificationsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetNotificationsResponse.java)

## 例

[inline-code-attrs-start title = 'getNotifications の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// クラスをインポート：
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
    
    // APIキー認証を設定: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // APIキーのプレフィックス（例: "Token"、デフォルトは null）を設定するには、次の行のコメントを外してください
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String userId = "userId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    String fromCommentId = "fromCommentId_example"; // String | 
    Boolean viewed = true; // Boolean | 
    String type = "type_example"; // String | 
    Double skip = 3.4D; // Double | 
    try {
      GetNotificationsResponse result = apiInstance.getNotifications(tenantId)
            .userId(userId)
            .urlId(urlId)
            .fromCommentId(fromCommentId)
            .viewed(viewed)
            .type(type)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // DefaultApi#getNotifications を呼び出したときの例外
      System.err.println("Exception when calling DefaultApi#getNotifications");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]