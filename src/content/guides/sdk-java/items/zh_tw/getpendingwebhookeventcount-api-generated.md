## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| commentId | string | query | 否 |  |
| externalId | string | query | 否 |  |
| eventType | string | query | 否 |  |
| type | string | query | 否 |  |
| domain | string | query | 否 |  |
| attemptCountGT | number | query | 否 |  |

## 回應

Returns: [`GetPendingWebhookEventCountResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPendingWebhookEventCountResponse.java)

## 範例

[inline-code-attrs-start title = 'getPendingWebhookEventCount 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // 設定 API 金鑰授權: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // 取消註解以下行以設定 API 金鑰的前綴，例如 "Token"（預設為 null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    String externalId = "externalId_example"; // String | 
    String eventType = "eventType_example"; // String | 
    String type = "type_example"; // String | 
    String domain = "domain_example"; // String | 
    Double attemptCountGT = 3.4D; // Double | 
    try {
      GetPendingWebhookEventCountResponse result = apiInstance.getPendingWebhookEventCount(tenantId)
            .commentId(commentId)
            .externalId(externalId)
            .eventType(eventType)
            .type(type)
            .domain(domain)
            .attemptCountGT(attemptCountGT)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getPendingWebhookEventCount");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]