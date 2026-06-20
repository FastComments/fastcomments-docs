## パラメータ

| Name | 型 | 送信先 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| text-search | string | query | いいえ |  |
| byIPFromComment | string | query | いいえ |  |
| filters | string | query | いいえ |  |
| searchFilters | string | query | いいえ |  |
| sorts | string | query | いいえ |  |
| sso | string | query | いいえ |  |

## レスポンス

戻り値: [`ModerationExportResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationExportResponse.java)

## 例

[inline-code-attrs-start title = 'postApiExport の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String textSearch = "textSearch_example"; // String | 
    String byIPFromComment = "byIPFromComment_example"; // String | 
    String filters = "filters_example"; // String | 
    String searchFilters = "searchFilters_example"; // String | 
    String sorts = "sorts_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      ModerationExportResponse result = apiInstance.postApiExport()
            .textSearch(textSearch)
            .byIPFromComment(byIPFromComment)
            .filters(filters)
            .searchFilters(searchFilters)
            .sorts(sorts)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#postApiExport");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]