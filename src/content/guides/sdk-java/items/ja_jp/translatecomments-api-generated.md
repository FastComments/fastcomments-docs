指定されたコメントをウィジェットのロケールに翻訳します。対応するウィジェットのカスタマイズルールで「Comment Translation」が「Automatic」または「Manual」に設定されている場合にのみ利用可能です。

## パラメーター

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | はい |  |
| urlId | string | query | はい |  |
| sso | string | query | いいえ |  |
| locale | string | query | いいえ |  |

## レスポンス

戻り値: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CommentTranslationsResponse.java)

## 例

[inline-code-attrs-start title = 'translateComments の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// クラスをインポート:
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
    String urlId = "urlId_example"; // String | 
    CommentTranslationsRequest commentTranslationsRequest = new CommentTranslationsRequest(); // CommentTranslationsRequest | 
    String sso = "sso_example"; // String | 
    String locale = "locale_example"; // String | 
    try {
      CommentTranslationsResponse result = apiInstance.translateComments(tenantId, urlId, commentTranslationsRequest)
            .sso(sso)
            .locale(locale)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#translateComments");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]