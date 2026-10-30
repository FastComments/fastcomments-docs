訂閱一個 URL 以接收評論事件（REST hook 訂閱）。再次將相同的 URL 訂閱到相同的  
事件與網域時，會返回現有的訂閱。傳遞內容使用 HMAC 簽名，請參閱  
webhooks 指南；舊版的 `token` 標頭不會傳送給 API 訂閱。

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |

## Response

Returns: [`CreateWebhookResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateWebhookResponse.java)

## Example

[inline-code-attrs-start title = 'createWebhook 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 匯入類別:
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
    
    // 設定 API 金鑰授權：api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // 取消註解以下行以設定 API 金鑰的前綴，例如 "Token"（預設為 null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    CreateWebhookParams createWebhookParams = new CreateWebhookParams(); // CreateWebhookParams | 
    try {
      CreateWebhookResponse result = apiInstance.createWebhook(tenantId, createWebhookParams)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createWebhook");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---