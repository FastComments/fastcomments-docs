Identifica le credenziali in uso: il tenant a cui appartiene e, per i token OAuth, l'utente che le ha autorizzate. Le integrazioni usano questo per testare una connessione e etichettarla.

## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | query | Sì |  |

## Response

Restituisce: [`GetMeResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetMeResponse.java)

## Example

[inline-code-attrs-start title = 'Esempio getMe'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importa le classi:
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
    // Decommenta la riga seguente per impostare un prefisso per la chiave API, ad es. "Token" (predefinito a null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    try {
      GetMeResponse result = apiInstance.getMe(tenantId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getMe");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---