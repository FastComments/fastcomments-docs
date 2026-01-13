## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | No |  |

## レスポンス

返却値: [`AddHashTagsBulk200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/AddHashTagsBulk200Response.java)

## 例

[inline-code-attrs-start title = 'addHashTagsBulk の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // APIキー認証の設定: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // APIキーにプレフィックスを設定するには次の行のコメントを外してください。例: "Token"（デフォルトは null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    BulkCreateHashTagsBody bulkCreateHashTagsBody = new BulkCreateHashTagsBody(); // BulkCreateHashTagsBody | 
    try {
      AddHashTagsBulk200Response result = apiInstance.addHashTagsBulk()
            .tenantId(tenantId)
            .bulkCreateHashTagsBody(bulkCreateHashTagsBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // DefaultApi#addHashTagsBulk を呼び出した際の例外
      System.err.println("Exception when calling DefaultApi#addHashTagsBulk");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]