Frühere Kommentierende auf der Seite, die derzeit NICHT online sind. Nach displayName sortiert.
Verwenden Sie dies, nachdem Sie /users/online erschöpft haben, um einen Abschnitt 'Mitglieder' darzustellen.
Cursor-Pagination auf commenterName: der Server durchläuft den partiellen {tenantId, urlId, commenterName}-Index.
Index ab afterName vorwärts mittels $gt, keine $skip-Kosten.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Seiten-URL-Kennung (serverseitig bereinigt). |
| afterName | string | query | Nein | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort. |
| afterUserId | string | query | Nein | Cursor-Tiebreaker: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Einträge bei Namensgleichheit nicht verloren gehen. |

## Antwort

Gibt zurück: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getOfflineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Klassen importieren:
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
    String urlId = "urlId_example"; // String | Seiten-URL-Kennung (serverseitig bereinigt).
    String afterName = "afterName_example"; // String | Cursor: übergeben Sie nextAfterName aus der vorherigen Antwort.
    String afterUserId = "afterUserId_example"; // String | Cursor-Tiebreaker: übergeben Sie nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit Einträge bei Namensgleichheit nicht verloren gehen.
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