## 参数

| 名称 | 类型 | 位置 | 必需 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | path | 是 |  |
| sso | string | query | 否 |  |

## 响应

返回: [`GetCommentTextResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetCommentTextResponse.java)

## 示例

[inline-code-attrs-start title = 'getModerationCommentText 示例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 导入类:
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
    String commentId = "commentId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      GetCommentTextResponse result = apiInstance.getModerationCommentText(tenantId, commentId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("调用 ModerationApi#getModerationCommentText 时的异常");
      System.err.println("状态码: " + e.getCode());
      System.err.println("原因: " + e.getResponseBody());
      System.err.println("响应头: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---