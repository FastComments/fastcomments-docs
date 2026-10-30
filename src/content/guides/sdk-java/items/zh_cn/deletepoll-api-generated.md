Remove a poll from its comment, along with every vote cast on it. The comment itself is left alone.

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIEmptyResponse.java)

## 示例

[inline-code-attrs-start title = 'deletePoll 示例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 导入类:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // 配置 API 密钥授权: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // 取消注释以下行以为 API 密钥设置前缀，例如 "Token"（defaults to null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    try {
      APIEmptyResponse result = apiInstance.deletePoll(tenantId, commentId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // 调用 DefaultApi#deletePoll 时的异常
      System.err.println("Exception when calling DefaultApi#deletePoll");
      // 状态码: " + e.getCode()
      System.err.println("Status code: " + e.getCode());
      // 原因: " + e.getResponseBody()
      System.err.println("Reason: " + e.getResponseBody());
      // 响应头: " + e.getResponseHeaders()
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---