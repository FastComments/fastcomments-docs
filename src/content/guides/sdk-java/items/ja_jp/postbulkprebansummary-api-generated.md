## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| includeByUserIdAndEmail | boolean | query | いいえ |  |
| includeByIP | boolean | query | いいえ |  |
| includeByEmailDomain | boolean | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`BulkPreBanSummary`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/BulkPreBanSummary.java)

## 例

[inline-code-attrs-start title = 'postBulkPreBanSummary の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// クラスのインポート:
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
    BulkPreBanParams bulkPreBanParams = new BulkPreBanParams(); // BulkPreBanParams | 
    Boolean includeByUserIdAndEmail = true; // Boolean | 
    Boolean includeByIP = true; // Boolean | 
    Boolean includeByEmailDomain = true; // Boolean | 
    String sso = "sso_example"; // String | 
    try {
      BulkPreBanSummary result = apiInstance.postBulkPreBanSummary(bulkPreBanParams)
            .includeByUserIdAndEmail(includeByUserIdAndEmail)
            .includeByIP(includeByIP)
            .includeByEmailDomain(includeByEmailDomain)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#postBulkPreBanSummary");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---