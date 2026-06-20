## 參數

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | 查詢 | 是 |  |
| userId | string | 查詢 | 否 |  |
| urlId | string | 查詢 | 否 |  |
| fromCommentId | string | 查詢 | 否 |  |
| viewed | boolean | 查詢 | 否 |  |
| type | string | 查詢 | 否 |  |

## 回應

回傳： [`GetNotificationCountResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetNotificationCountResponse.java)

## 範例

[inline-code-attrs-start title = 'getNotificationCount 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // 取消註解下列行以設定 API 金鑰的前綴，例如 "Token"（預設為 null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String userId = "userId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    String fromCommentId = "fromCommentId_example"; // String | 
    Boolean viewed = true; // Boolean | 
    String type = "type_example"; // String | 
    try {
      GetNotificationCountResponse result = apiInstance.getNotificationCount(tenantId)
            .userId(userId)
            .urlId(urlId)
            .fromCommentId(fromCommentId)
            .viewed(viewed)
            .type(type)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getNotificationCount");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]