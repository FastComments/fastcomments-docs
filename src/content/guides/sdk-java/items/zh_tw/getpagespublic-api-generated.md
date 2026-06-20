列出租戶的頁面。由 FChat 桌面用戶端用來填充其房間清單。要求每個頁面的解析後自訂設定中的 `enableFChat` 必須為 true。需要 SSO 的頁面會根據請求使用者的群組存取進行過濾。

## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 不透明的分頁游標，從先前請求回傳的 `nextCursor`。與相同的 `sortBy` 綁定。 |
| limit | integer | query | No | 1..200，預設 50 |
| q | string | query | No | 可選的大小寫不敏感的標題前綴篩選。 |
| sortBy | string | query | No | 排序方式。`updatedAt`（預設，最新者在前）、`commentCount`（留言數多者在前），或 `title`（依字母順序）。 |
| hasComments | boolean | query | No | 如果為 true，僅回傳至少有一則留言的頁面。 |

## 回應

回傳: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## 範例

[inline-code-attrs-start title = 'getPagesPublic 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 載入類別：
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
    String cursor = "cursor_example"; // String | 不透明的分頁游標，從先前請求回傳的 `nextCursor`。與相同的 `sortBy` 綁定。
    Integer limit = 56; // Integer | 1..200，預設 50
    String q = "q_example"; // String | 可選的大小寫不敏感的標題前綴篩選。
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | 排序方式。`updatedAt`（預設，最新者在前）、`commentCount`（留言數多者在前），或 `title`（依字母順序）。
    Boolean hasComments = true; // Boolean | 如果為 true，僅回傳至少有一則留言的頁面。
    try {
      GetPublicPagesResponse result = apiInstance.getPagesPublic(tenantId)
            .cursor(cursor)
            .limit(limit)
            .q(q)
            .sortBy(sortBy)
            .hasComments(hasComments)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getPagesPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]