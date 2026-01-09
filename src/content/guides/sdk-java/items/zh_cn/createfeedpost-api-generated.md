## 参数

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| broadcastId | string | query | 否 |  |
| isLive | boolean | query | 否 |  |
| doSpamCheck | boolean | query | 否 |  |
| skipDupCheck | boolean | query | 否 |  |

## 响应

返回: [`CreateFeedPost200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateFeedPost200Response.java)

## 示例

[inline-code-attrs-start title = 'createFeedPost 示例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // 取消注释以下行以为 API 密钥设置前缀，例如 "Token" (defaults to null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    CreateFeedPostParams createFeedPostParams = new CreateFeedPostParams(); // CreateFeedPostParams | 
    String broadcastId = "broadcastId_example"; // String | 
    Boolean isLive = true; // Boolean | 
    Boolean doSpamCheck = true; // Boolean | 
    Boolean skipDupCheck = true; // Boolean | 
    try {
      CreateFeedPost200Response result = apiInstance.createFeedPost(tenantId, createFeedPostParams)
            .broadcastId(broadcastId)
            .isLive(isLive)
            .doSpamCheck(doSpamCheck)
            .skipDupCheck(skipDupCheck)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createFeedPost");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---