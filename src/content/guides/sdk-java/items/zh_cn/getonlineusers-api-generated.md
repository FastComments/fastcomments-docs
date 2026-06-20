---
当前在线的页面查看者：指当前其 websocket 会话已订阅该页面的人。返回 anonCount + totalCount（房间范围的订阅者，包括我们不逐一列举的匿名查看者）。

## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | 路径 | 是 |  |
| urlId | string | 查询 | 是 | 页面 URL 标识符（服务器端已清理）。 |
| afterName | string | 查询 | 否 | 游标：传递上一响应中的 nextAfterName。 |
| afterUserId | string | 查询 | 否 | 游标平局决胜：传递上一响应中的 nextAfterUserId。当设置 afterName 时需要此参数，以防姓名相同时条目被丢弃。 |

## 响应

返回：[`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## 示例

[inline-code-attrs-start title = 'getOnlineUsers 示例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | 页面 URL 标识符（服务器端已清理）。
    String afterName = "afterName_example"; // String | 游标：传递上一响应中的 nextAfterName。
    String afterUserId = "afterUserId_example"; // String | 游标平局决胜：传递上一响应中的 nextAfterUserId。当设置 afterName 时需要此参数，以防姓名相同时条目被丢弃。
    try {
      PageUsersOnlineResponse result = apiInstance.getOnlineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOnlineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---