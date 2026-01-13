---
## パラメータ

| 名前 | 型 | Location | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 任意 |  |

## レスポンス

戻り値: [`AddHashTag200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/AddHashTag200Response.java)

## 例

[inline-code-attrs-start title = 'addHashTag の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// クラスをインポート:
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
    // APIキーにプレフィックスを設定するには、次の行のコメントを解除してください。例: "Token"（デフォルトは null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    CreateHashTagBody createHashTagBody = new CreateHashTagBody(); // CreateHashTagBody | 
    try {
      AddHashTag200Response result = apiInstance.addHashTag()
            .tenantId(tenantId)
            .createHashTagBody(createHashTagBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#addHashTag");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---