## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | path | Oui |  |
| sso | string | query | Non |  |

## Réponse

Retourne : [`GetBannedUsersFromCommentResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetBannedUsersFromCommentResponse.java)

## Exemple

[inline-code-attrs-start title = 'getBanUsersFromComment Exemple'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes:
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
      System.err.println("Exception lors de l'appel de ModerationApi#getBanUsersFromComment");
      System.err.println("Code d'état: " + e.getCode());
      System.err.println("Raison: " + e.getResponseBody());
      System.err.println("En-têtes de réponse: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---