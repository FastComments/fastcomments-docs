---
Anciens commentateurs sur la page qui ne sont pas actuellement en ligne. Triés par displayName.
Utilisez ceci après avoir épuisé /users/online pour afficher une section "Members".
Pagination par curseur sur commenterName : le serveur parcourt l'index partiel {tenantId, urlId, commenterName} à partir de afterName vers l'avant via $gt, sans coût $skip.

## Paramètres

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Identifiant d'URL de la page (nettoyé côté serveur). |
| afterName | string | query | No | Curseur : passez nextAfterName depuis la réponse précédente. |
| afterUserId | string | query | No | Séparateur d'égalité du curseur : passez nextAfterUserId depuis la réponse précédente. Obligatoire lorsque afterName est défini afin d'éviter que des égalités de nom ne fassent disparaître des entrées. |

## Réponse

Renvoie : [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple de getOfflineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | Identifiant d'URL de la page (nettoyé côté serveur).
    String afterName = "afterName_example"; // String | Curseur : passez nextAfterName depuis la réponse précédente.
    String afterUserId = "afterUserId_example"; // String | Séparateur d'égalité du curseur : passez nextAfterUserId depuis la réponse précédente. Obligatoire lorsque afterName est défini afin d'éviter que des égalités de nom ne fassent disparaître des entrées.
    try {
      PageUsersOfflineResponse result = apiInstance.getOfflineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOfflineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---