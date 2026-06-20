## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | chemin | Oui |  |
| commentId | string | chemin | Oui |  |
| dir | integer | query | Oui |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`GetCommentVoteUserNamesSuccessResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetCommentVoteUserNamesSuccessResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple de getCommentVoteUserNames'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes:
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
    Integer dir = 56; // Entier | 
    String sso = "sso_example"; // Chaîne | 
    try {
      GetCommentVoteUserNamesSuccessResponse result = apiInstance.getCommentVoteUserNames(tenantId, commentId, dir)
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