## パラメータ

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | はい |  |
| yearNumber | number | query | いいえ |  |
| monthNumber | number | query | いいえ |  |
| dayNumber | number | query | いいえ |  |
| skip | number | query | いいえ |  |

## レスポンス

戻り値: [`GetTenantDailyUsages200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetTenantDailyUsages200Response.java)

## 例

[inline-code-attrs-start title = 'getTenantDailyUsagesの例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // APIキーのプレフィックスを設定するには、以下の行のコメントを外してください。例: "Token"（デフォルトは null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    Double yearNumber = 3.4D; // Double | 
    Double monthNumber = 3.4D; // Double | 
    Double dayNumber = 3.4D; // Double | 
    Double skip = 3.4D; // Double | 
    try {
      GetTenantDailyUsages200Response result = apiInstance.getTenantDailyUsages(tenantId)
            .yearNumber(yearNumber)
            .monthNumber(monthNumber)
            .dayNumber(dayNumber)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getTenantDailyUsages");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]