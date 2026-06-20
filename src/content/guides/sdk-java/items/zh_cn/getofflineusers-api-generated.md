页面上过去发表评论但当前不在线的用户。按 displayName 排序。
在耗尽 /users/online 后使用此方法以渲染 "Members" 部分。
在 commenterName 上的游标分页：服务器在部分 {tenantId, urlId, commenterName} 索引上，从 afterName 向前通过 $gt 遍历，不产生 $skip 成本。

## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 路径 | 是 |  |
| urlId | string | 查询 | 是 | 页面 URL 标识符（由服务器端清理）。 |
| afterName | string | 查询 | 否 | 游标：传入上一次响应中的 nextAfterName。 |
| afterUserId | string | 查询 | 否 | 游标决胜项：传入上一次响应中的 nextAfterUserId。当设置了 afterName 时此项为必需，以防止同名情况下条目丢失。 |

## 响应

返回：[`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## 示例

[inline-code-attrs-start title = 'getOfflineUsers 示例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | 页面 URL 标识符（由服务器端清理）。
    String afterName = "afterName_example"; // String | 游标：传入上一次响应中的 nextAfterName。
    String afterUserId = "afterUserId_example"; // String | 游标决胜项：传入上一次响应中的 nextAfterUserId。当设置了 afterName 时此项为必需，以防止同名情况下条目丢失。
    try {
      PageUsersOfflineResponse result = apiInstance.getOfflineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOfflineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]