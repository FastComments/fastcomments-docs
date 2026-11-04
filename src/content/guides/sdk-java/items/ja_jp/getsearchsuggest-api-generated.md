## Parameters

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| text-search | string | query | No |  |
| sso | string | query | No |  |

## Response

返却: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationSuggestResponse.java)

## Example

[inline-code-attrs-start title = 'getSearchSuggest の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
// クラスをインポート:
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
    String tenantId = "tenantId_example"; // String | 
    String textSearch = "textSearch_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      ModerationSuggestResponse result = apiInstance.getSearchSuggest(tenantId)
            .textSearch(textSearch)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("ModerationApi#getSearchSuggest の呼び出し時の例外");
      System.err.println("ステータスコード: " + e.getCode());
      System.err.println("理由: " + e.getResponseBody());
      System.err.println("レスポンスヘッダー: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]