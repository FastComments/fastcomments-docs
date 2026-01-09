## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| commentId | string | path | Da |  |
| broadcastId | string | query | Da |  |
| sso | string | query | Ne |  |

## Odgovor

Vraća: [`LockComment200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/LockComment200Response.java)

## Primjer

[inline-code-attrs-start title = 'unLockComment Primjer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvoz klasa:
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
    String broadcastId = "broadcastId_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      LockComment200Response result = apiInstance.unLockComment(tenantId, commentId, broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#unLockComment");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]