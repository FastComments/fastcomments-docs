將給定的評論翻譯成小工具的語系。僅在相應的小工具自訂規則將「評論翻譯」設定為自動或手動時可用。

## 參數

| 名稱 | 類型 | 位置 | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| locale | string | query | No |  |

## 回應

回傳: [`CommentTranslationsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CommentTranslationsResponse.java)

## 範例

[inline-code-attrs-start title = 'translateComments 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 匯入類別:
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
      System.err.println("呼叫 PublicApi#translateComments 時的例外");
      System.err.println("狀態碼: " + e.getCode());
      System.err.println("原因: " + e.getResponseBody());
      System.err.println("回應標頭: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]