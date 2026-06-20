## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|------|------|-------------|
| tenantId | string | 路徑 | 是 |  |
| search | string | 查詢 | 是 |  |
| locale | string | 查詢 | 否 |  |
| rating | string | 查詢 | 否 |  |
| page | number | 查詢 | 否 |  |

## 回應

Returns: [`GetGifsSearchResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetGifsSearchResponse.java)

## 範例

[inline-code-attrs-start title = 'getGifsSearch 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 匯入類別：
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
    String search = "search_example"; // String | 
    String locale = "locale_example"; // String | 
    String rating = "rating_example"; // String | 
    Double page = 3.4D; // Double | 
    try {
      GetGifsSearchResponse result = apiInstance.getGifsSearch(tenantId, search)
            .locale(locale)
            .rating(rating)
            .page(page)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getGifsSearch");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]