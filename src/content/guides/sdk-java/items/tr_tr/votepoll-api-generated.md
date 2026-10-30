## Parametreler

| Ad | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | path | Evet |  |
| commentId | string | path | Evet |  |
| urlId | string | query | Hayır |  |
| broadcastId | string | query | Hayır |  |
| sso | string | query | Hayır |  |

## Yanıt

Döndürür: [`VotePollResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/VotePollResponse.java)

## Örnek

[inline-code-attrs-start title = 'votePoll Örneği'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sınıfları içe aktar:
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
    String commentId = "commentId_example"; // String | 
    VotePollBody votePollBody = new VotePollBody(); // VotePollBody | 
    String urlId = "urlId_example"; // String | 
    String broadcastId = "broadcastId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      VotePollResponse result = apiInstance.votePoll(tenantId, commentId, votePollBody)
            .urlId(urlId)
            .broadcastId(broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // PublicApi#votePoll çağrılırken istisna
      System.err.println("Exception when calling PublicApi#votePoll");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---