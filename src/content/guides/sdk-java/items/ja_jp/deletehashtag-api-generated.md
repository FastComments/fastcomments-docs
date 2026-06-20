## パラメーター

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tag | string | path | はい |  |
| tenantId | string | query | いいえ |  |

## レスポンス

戻り値: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIEmptyResponse.java)

## 例

[inline-code-attrs-start title = 'deleteHashTag の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// クラスのインポート:
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
    // 次の行のコメントを外して、APIキーの接頭辞（例: "Token"）を設定します（デフォルトは null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tag = "tag_example"; // String | 
    String tenantId = "tenantId_example"; // String | 
    DeleteHashTagRequestBody deleteHashTagRequestBody = new DeleteHashTagRequestBody(); // DeleteHashTagRequestBody | 
    try {
      APIEmptyResponse result = apiInstance.deleteHashTag(tag)
            .tenantId(tenantId)
            .deleteHashTagRequestBody(deleteHashTagRequestBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#deleteHashTag");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]