---
Utilisateurs actuellement en ligne sur une page : personnes dont la session websocket est abonnée à la page en ce moment.
Retourne anonCount + totalCount (abonnés de la salle, y compris les spectateurs anonymes que nous n'énumérons pas).

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (cleaned server-side). |
| afterName | string | query | No | Curseur : transmettre nextAfterName depuis la réponse précédente. |
| afterUserId | string | query | No | Séparateur de cas d'égalité pour le curseur : transmettre nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom n'entraînent pas la suppression d'entrées. |

## Réponse

Retourne : [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple de getOnlineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | Identifiant de l'URL de la page (nettoyé côté serveur).
    String afterName = "afterName_example"; // String | Curseur : transmettre nextAfterName depuis la réponse précédente.
    String afterUserId = "afterUserId_example"; // String | Séparateur de cas d'égalité pour le curseur : transmettre nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de nom n'entraînent pas la suppression d'entrées.
    try {
      PageUsersOnlineResponse result = apiInstance.getOnlineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOnlineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---