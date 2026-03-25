## Parametri

| Nome | Tipo | Posizione | Richiesto | Descrizione |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sì |  |
| id | string | path | Sì |  |
| userId | string | query | No |  |

## Risposta

Restituisce: [`GetTicket200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetTicket200Response.java)

## Esempio

[inline-code-attrs-start title = 'Esempio getTicket'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importa classi:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // Configura l'autorizzazione con chiave API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Decommenta la seguente riga per impostare un prefisso per la chiave API, es. "Token" (predefinito null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String id = "id_example"; // String | 
    String userId = "userId_example"; // String | 
    try {
      GetTicket200Response result = apiInstance.getTicket(tenantId, id)
            .userId(userId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getTicket");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---