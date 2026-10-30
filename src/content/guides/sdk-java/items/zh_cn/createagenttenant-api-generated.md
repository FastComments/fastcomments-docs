创建一个新的 AI 代理试用账户，无需人工注册。调用此接口不需要 API 密钥。

响应中包含租户 ID、可立即用于 REST API 和 MCP 服务器的 API 密钥，以及一个声明 URL。将声明 URL 提供给您所服务的用户：在登录 FastComments 后打开该链接即可将账户关联到该用户。未声明的账户及其密钥将在创建后 72 小时内被删除。在声明之前，账户遵循标准试用限制。

## 响应

返回: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## 示例

[inline-code-attrs-start title = 'createAgentTenant 示例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 导入类:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    CreateAgentTenantBody createAgentTenantBody = new CreateAgentTenantBody(); // CreateAgentTenantBody | 
    try {
      CreateAgentTenantResponse result = apiInstance.createAgentTenant(createAgentTenantBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createAgentTenant");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]