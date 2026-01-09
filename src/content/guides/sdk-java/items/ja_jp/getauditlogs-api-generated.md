## パラメータ

| Name | Type | Location | 必須 | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| limit | number | query | いいえ |  |
| skip | number | query | いいえ |  |
| order | string | query | いいえ |  |
| after | number | query | いいえ |  |
| before | number | query | いいえ |  |

## レスポンス

返却値: [`GetAuditLogs200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogs200Response.java)

## 例

[inline-code-attrs-start title = 'getAuditLogs の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // API キー認証を構成: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // 次の行のコメントを外すと、API キーのプレフィックスを設定できます。例: "Token"（デフォルトは null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    Double limit = 3.4D; // Double | 
    Double skip = 3.4D; // Double | 
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | 
    Double after = 3.4D; // Double | 
    Double before = 3.4D; // Double | 
    try {
      GetAuditLogs200Response result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---