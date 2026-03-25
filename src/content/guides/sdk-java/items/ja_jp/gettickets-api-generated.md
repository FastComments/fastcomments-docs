## パラメータ

| 名前 | 型 | 場所 | 必須 | 説明 |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| userId | string | query | いいえ |  |
| state | number | query | いいえ |  |
| skip | number | query | いいえ |  |
| limit | number | query | いいえ |  |

## レスポンス

戻り値: [`GetTickets200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetTickets200Response.java)

## 例

[inline-code-attrs-start title = 'getTickets の例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// クラスをインポート:
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
    
    // APIキー認証を設定: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Uncomment the following line to set a prefix for the API key, e.g. "Token" (defaults to null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String userId = "userId_example"; // String | 
    Double state = 3.4D; // Double | 
    Double skip = 3.4D; // Double | 
    Double limit = 3.4D; // Double | 
    try {
      GetTickets200Response result = apiInstance.getTickets(tenantId)
            .userId(userId)
            .state(state)
            .skip(skip)
            .limit(limit)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getTickets");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]