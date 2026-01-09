## 參數

| 名稱 | 類型 | Location | 必填 | 說明 |
|------|------|----------|----------|-------------|
| tenantId | string | path | 是 |  |
| postId | string | path | 是 |  |
| isUndo | boolean | query | 否 |  |
| broadcastId | string | query | 否 |  |
| sso | string | query | 否 |  |

## 回應

回傳: [`ReactFeedPostPublic200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ReactFeedPostPublic200Response.java)

## 範例

[inline-code-attrs-start title = 'reactFeedPostPublic 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// 匯入類別：
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
    String postId = "postId_example"; // String | 
    ReactBodyParams reactBodyParams = new ReactBodyParams(); // ReactBodyParams | 
    Boolean isUndo = true; // Boolean | 
    String broadcastId = "broadcastId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      ReactFeedPostPublic200Response result = apiInstance.reactFeedPostPublic(tenantId, postId, reactBodyParams)
            .isUndo(isUndo)
            .broadcastId(broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#reactFeedPostPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]