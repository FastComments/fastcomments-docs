## 参数

| 名称 | 类型 | 所在位置 | 必需 | 描述 |
|------|------|----------|------|-------------|
| tenantId | string | 路径 | 是 |  |
| postId | string | 路径 | 是 |  |
| broadcastId | string | 查询 | 否 |  |
| sso | string | 查询 | 否 |  |

## 响应

返回：[`DeleteFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/DeleteFeedPostPublic200Response.java)

## 示例

[inline-code-attrs-start title = 'deleteFeedPostPublic 示例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String postId = "postId_example"; // String | 
    String broadcastId = "broadcastId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      DeleteFeedPostPublic200Response result = apiInstance.deleteFeedPostPublic(tenantId, postId)
            .broadcastId(broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#deleteFeedPostPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---