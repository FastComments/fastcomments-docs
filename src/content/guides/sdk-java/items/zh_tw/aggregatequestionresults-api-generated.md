## 參數

| 名稱 | 類型 | 位置 | 必填 | 描述 |
|------|------|----------|----------|-------------|
| tenantId | string | query | 是 |  |
| questionId | string | query | 否 |  |
| questionIds | array | query | 否 |  |
| urlId | string | query | 否 |  |
| timeBucket | string | query | 否 |  |
| startDate | string | query | 否 |  |
| forceRecalculate | boolean | query | 否 |  |

## 回應

回傳: [`AggregateQuestionResults200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/AggregateQuestionResults200Response.java)

## 範例

[inline-code-attrs-start title = 'aggregateQuestionResults 範例'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // 取消註解下列行以設定 API 金鑰的前綴，例如 "Token"（預設為 null）
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String questionId = "questionId_example"; // String | 
    List<String> questionIds = Arrays.asList(); // List<String> | 
    String urlId = "urlId_example"; // String | 
    AggregateTimeBucket timeBucket = AggregateTimeBucket.fromValue("day"); // AggregateTimeBucket | 
    OffsetDateTime startDate = OffsetDateTime.now(); // OffsetDateTime | 
    Boolean forceRecalculate = true; // Boolean | 
    try {
      AggregateQuestionResults200Response result = apiInstance.aggregateQuestionResults(tenantId)
            .questionId(questionId)
            .questionIds(questionIds)
            .urlId(urlId)
            .timeBucket(timeBucket)
            .startDate(startDate)
            .forceRecalculate(forceRecalculate)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#aggregateQuestionResults");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]