## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| commentId | string | path | Oui |  |
| dir | integer | query | Oui |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`GetCommentVoteUserNames200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetCommentVoteUserNames200Response.java)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentVoteUserNames'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
      System.err.println("Exception when calling PublicApi#getCommentVoteUserNames");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]