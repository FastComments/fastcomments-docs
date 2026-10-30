## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|------------|--------------|-------------|
| tenantId | string | query | Yes | Il tenant per cui leggere i log di audit. |
| limit | integer | query | No | Quanti log restituire. Max 10000, valore predefinito 1000. |
| skip | integer | query | No | Quanti log saltare, per paginazione. |
| order | string | query | No | Direzione di ordinamento sul tempo dell'evento. Predefinito DESC (più recenti per primi). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Restituisce solo gli eventi eseguiti da questo nome utente. |
| ip | string | query | No | Restituisce solo gli eventi da questo indirizzo IP. |
| crudType | string | query | No | Restituisce solo gli eventi di questo tipo: c, r, u, d, o login. |
| resourceName | string | query | No | Restituisce solo gli eventi per questa risorsa, ad es. User o Moderator. |
| targetId | string | query | No | Restituisce solo gli eventi il cui oggetto interessato ha questo ID. |
| target | string | query | No | Corrispondenza di sottostringa non sensibile a maiuscole/minuscole sull'etichetta dell'oggetto interessato, ad es. un nome utente o email. |
| includeManagedTenants | boolean | query | No | Restituisce anche gli eventi dai tenant che questo tenant gestisce. |

## Response

Restituisce: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Esempio

[inline-code-attrs-start title = 'Esempio getAuditLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Configura l'autorizzazione della chiave API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Decommenta la riga seguente per impostare un prefisso per la chiave API, ad es. "Token" (predefinito null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | Il tenant per cui leggere i log di audit.
    Integer limit = 56; // Integer | Quanti log restituire. Max 10000, valore predefinito 1000.
    Integer skip = 56; // Integer | Quanti log saltare, per paginazione.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Direzione di ordinamento sul tempo dell'evento. Predefinito DESC (più recenti per primi).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Restituisce solo gli eventi eseguiti da questo nome utente.
    String ip = "ip_example"; // String | Restituisce solo gli eventi da questo indirizzo IP.
    String crudType = "crudType_example"; // String | Restituisce solo gli eventi di questo tipo: c, r, u, d, o login.
    String resourceName = "resourceName_example"; // String | Restituisce solo gli eventi per questa risorsa, ad es. User o Moderator.
    String targetId = "targetId_example"; // String | Restituisce solo gli eventi il cui oggetto interessato ha questo ID.
    String target = "target_example"; // String | Corrispondenza di sottostringa non sensibile a maiuscole/minuscole sull'etichetta dell'oggetto interessato, ad es. un nome utente o email.
    Boolean includeManagedTenants = true; // Boolean | Restituisce anche gli eventi dai tenant che questo tenant gestisce.
    try {
      GetAuditLogsResponse result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .username(username)
            .ip(ip)
            .crudType(crudType)
            .resourceName(resourceName)
            .targetId(targetId)
            .target(target)
            .includeManagedTenants(includeManagedTenants)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]