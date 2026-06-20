---
列出租户的页面。由 FChat 桌面客户端用于填充其房间列表。
要求每个页面的已解析自定义配置中的 `enableFChat` 为 true。
需要 SSO 的页面将根据请求用户的组访问权限进行过滤。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | 来自先前请求的作为 `nextCursor` 返回的不透明分页游标。与相同的 `sortBy` 关联。 |
| limit | integer | query | No | 1..200，默认 50 |
| q | string | query | No | 可选的不区分大小写的标题前缀过滤器。 |
| sortBy | string | query | No | 排序顺序。`updatedAt`（默认，最新优先）、`commentCount`（评论最多优先）或 `title`（按字母顺序）。 |
| hasComments | boolean | query | No | 如果为 true，则只返回至少有一条评论的页面。 |

## Response

返回: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## 示例

[inline-code-attrs-start title = 'getPagesPublic 示例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 导入类：
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
    String cursor = "cursor_example"; // String | 来自先前请求的作为 `nextCursor` 返回的不透明分页游标。与相同的 `sortBy` 关联。
    Integer limit = 56; // Integer | 1..200，默认 50
    String q = "q_example"; // String | 可选的不区分大小写的标题前缀过滤器。
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | 排序顺序。`updatedAt`（默认，最新优先），`commentCount`（评论最多优先），或 `title`（按字母顺序）。
    Boolean hasComments = true; // Boolean | 如果为 true，则只返回至少有一条评论的页面。
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

---