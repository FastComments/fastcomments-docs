為 AI 代理建立新的試用帳戶，無需人工註冊。呼叫此 API 不需要 API 金鑰。

回應包含租戶 ID、一個可立即用於 REST API 與 MCP 伺服器的 API 金鑰，以及一個認領 URL。將認領 URL 提供給您所服務的人員：在登入 FastComments 的狀態下開啟該連結，即可將帳戶綁定至該人員。未認領的帳戶及其金鑰將在建立後 72 小時被刪除。在認領之前，該帳戶受標準試用限制約束。

## 回應

返回：[`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## 範例

[inline-code-attrs-start title = 'createAgentTenant 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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