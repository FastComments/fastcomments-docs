Aktuell online befindliche Betrachter einer Seite: Personen, deren WebSocket-Sitzung derzeit auf die Seite abonniert ist.
Gibt anonCount + totalCount zurück (raumweite Abonnenten, einschließlich anonymer Betrachter, die wir nicht einzeln auflisten).

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | Pfad | Ja |  |
| urlId | string | Abfrage | Ja | Seiten-URL-Kennung (serverseitig bereinigt). |
| afterName | string | Abfrage | Nein | Cursor: übergebe nextAfterName aus der vorherigen Antwort. |
| afterUserId | string | Abfrage | Nein | Cursor-Tiebreaker: übergebe nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit bei gleichen Namen keine Einträge ausgelassen werden. |

## Antwort

Gibt zurück: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für getOnlineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String afterName = "afterName_example"; // String | Cursor: übergebe nextAfterName aus der vorherigen Antwort.
    String afterUserId = "afterUserId_example"; // String | Cursor-Tiebreaker: übergebe nextAfterUserId aus der vorherigen Antwort. Erforderlich, wenn afterName gesetzt ist, damit bei gleichen Namen keine Einträge ausgelassen werden.
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