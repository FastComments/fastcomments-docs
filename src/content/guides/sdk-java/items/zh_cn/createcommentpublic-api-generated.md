## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| broadcastId | string | query | Yes |  |
| sessionId | string | query | No |  |
| sso | string | query | No |  |
| locale | string | query | No |  |

## 响应

返回：[`SaveCommentsResponseWithPresence`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/SaveCommentsResponseWithPresence.java)

## 示例

[inline-code-attrs-start title = 'createCommentPublic 示例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 导入类:
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
    String broadcastId = "broadcastId_example"; // String | 
    CommentData commentData = new CommentData(); // CommentData | 
    String sessionId = "sessionId_example"; // String | 
    String sso = "sso_example"; // String | 
    String locale = "locale_example"; // String | 
    try {
      SaveCommentsResponseWithPresence result = apiInstance.createCommentPublic(tenantId, urlId, broadcastId, commentData)
            .sessionId(sessionId)
            .sso(soup)
            .locale(locale)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("调用 PublicApi#createCommentPublic 时出现异常");
      System.err.println("状态码: " + e.getCode());
      System.err.println("原因: " + e.getResponseBody());
      System.err.println("响应头: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]