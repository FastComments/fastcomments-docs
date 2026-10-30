Unsubscribes (REST hook unsubscribe). Only subscriptions created through this API can be
deleted here; dashboard-managed webhooks are edited in the dashboard.

## 参数

| 名称 | 类型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| id | string | path | 是 |  |

## 响应

返回: [`APIEmptyResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/APIEmptyResponse.java)

## 示例

[inline-code-attrs-start title = 'deleteWebhook 示例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // 取消注释以下行以为 API 密钥设置前缀，例如 "Token"（默认 null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String id = "id_example"; // String | 
    try {
      APIEmptyResponse result = apiInstance.deleteWebhook(tenantId, id)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // 调用 DefaultApi#deleteWebhook 时的异常
      System.err.println("Exception when calling DefaultApi#deleteWebhook");
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