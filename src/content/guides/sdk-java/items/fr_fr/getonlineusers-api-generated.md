Actuellement, les visiteurs en ligne d'une page : personnes dont la session websocket est abonnée à la page en ce moment.
Renvoie anonCount + totalCount (abonnés de la salle, y compris les visiteurs anonymes que nous n'énumérons pas).

## Paramètres

| Nom | Type | Emplacement | Requis | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Oui |  |
| urlId | string | query | Oui | Identifiant d'URL de la page (nettoyé côté serveur). |
| afterName | string | query | Non | Curseur : passer nextAfterName depuis la réponse précédente. |
| afterUserId | string | query | Non | Briseur d'égalité du curseur : passer nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de noms n'entraînent pas la suppression d'entrées. |

## Réponse

Renvoie: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

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
    String urlId = "urlId_example"; // String | Identifiant d'URL de la page (nettoyé côté serveur).
    String afterName = "afterName_example"; // String | Curseur : passer nextAfterName depuis la réponse précédente.
    String afterUserId = "afterUserId_example"; // String | Briseur d'égalité du curseur : passer nextAfterUserId depuis la réponse précédente. Requis lorsque afterName est défini afin que les égalités de noms n'entraînent pas la suppression d'entrées.
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