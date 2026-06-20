Visualizzatori attualmente online di una pagina: persone la cui sessione websocket è sottoscritta alla pagina in questo momento.
Restituisce anonCount + totalCount (iscritti a livello di stanza, inclusi gli spettatori anonimi che non elenchiamo).

## Parametri

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | path | Sì |  |
| urlId | string | query | Sì | Identificatore dell'URL della pagina (ripulito lato server). |
| afterName | string | query | No | Cursore: passa nextAfterName dalla risposta precedente. |
| afterUserId | string | query | No | Cursore tie-breaker: passa nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i pareggi di nome non facciano perdere voci. |

## Risposta

Restituisce: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Esempio

[inline-code-attrs-start title = 'Esempio di getOnlineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importa classi:
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
    String urlId = "urlId_example"; // String | Identificatore URL della pagina (ripulito lato server).
    String afterName = "afterName_example"; // String | Cursore: passa nextAfterName dalla risposta precedente.
    String afterUserId = "afterUserId_example"; // String | Cursore tie-breaker: passa nextAfterUserId dalla risposta precedente. Richiesto quando afterName è impostato in modo che i pareggi di nome non facciano perdere voci.
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