## Parametri

| Ime | Tip | Lokacija | Zahtevano | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| postId | string | path | Da |  |
| broadcastId | string | query | Ne |  |
| sso | string | query | Ne |  |

## Odziv

Vrača: [`DeleteFeedPostPublicResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/DeleteFeedPostPublicResponse.java)

## Primer

[inline-code-attrs-start title = 'Primer deleteFeedPostPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvozi razrede:
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
    String tenantId = "tenantId_example"; // Niz | 
    String postId = "postId_example"; // Niz | 
    String broadcastId = "broadcastId_example"; // Niz | 
    String sso = "sso_example"; // Niz | 
    try {
      DeleteFeedPostPublicResponse result = apiInstance.deleteFeedPostPublic(tenantId, postId)
            .broadcastId(broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#deleteFeedPostPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]