## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| commentId | string | path | Da |  |
| dir | integer | query | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetCommentVoteUserNames200Response.java)

## Primer

[inline-code-attrs-start title = 'getCommentVoteUserNames Primer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvezi klase:
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
    Integer dir = 56; // Integer | 
    String sso = "sso_example"; // String | 
    try {
      GetCommentVoteUserNames200Response result = apiInstance.getCommentVoteUserNames(tenantId, commentId, dir)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Izuzetak pri pozivu PublicApi#getCommentVoteUserNames");
      System.err.println("Statusni kod: " + e.getCode());
      System.err.println("Razlog: " + e.getResponseBody());
      System.err.println("Zaglavlja odgovora: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---