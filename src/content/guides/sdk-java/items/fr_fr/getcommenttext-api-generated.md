## Paramètres

| Name | Type | Location | Requis | Description |
|------|------|----------|--------|-------------|
| tenantId | string | path | Oui |  |
| commentId | string | path | Oui |  |
| editKey | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`PublicAPIGetCommentTextResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PublicAPIGetCommentTextResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentText'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes :
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
    String tenantId = "tenantId_example"; // Chaîne | 
    String commentId = "commentId_example"; // Chaîne | 
    String editKey = "editKey_example"; // Chaîne | 
    String sso = "sso_example"; // Chaîne | 
    try {
      PublicAPIGetCommentTextResponse result = apiInstance.getCommentText(tenantId, commentId)
            .editKey(editKey)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getCommentText");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---