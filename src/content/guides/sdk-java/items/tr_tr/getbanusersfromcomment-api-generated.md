## Parameters

| İsim | Tür | Konum | Gerekli | Açıklama |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| commentId | string | path | Yes |  |
| sso | string | query | No |  |

## Response

Döndürür: [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetBannedUsersFromCommentResponse.java)

## Örnek

[inline-code-attrs-start title = 'getBanUsersFromComment Örneği'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Sınıfları içe aktar:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.ModerationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    ModerationApi apiInstance = new ModerationApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      GetBannedUsersFromCommentResponse result = apiInstance.getBanUsersFromComment(tenantId, commentId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("ModerationApi#getBanUsersFromComment çağrılırken istisna");
      System.err.println("Durum kodu: " + e.getCode());
      System.err.println("Sebep: " + e.getResponseBody());
      System.err.println("Yanıt üstbilgileri: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]