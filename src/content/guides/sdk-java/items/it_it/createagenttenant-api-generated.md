---
Crea un nuovo account di prova per un agente AI senza la registrazione da parte di un umano. Non è necessaria alcuna chiave API per chiamare questo.

La risposta contiene l'ID del tenant, una chiave API che funziona immediatamente contro l'API REST e il server MCP, e un URL di reclamo. Fornisci l'URL di reclamo all'umano per cui stai lavorando: aprirlo mentre sei connesso a FastComments associa l'account a loro. Gli account non reclamati, e le loro chiavi, vengono eliminati 72 ore dopo la creazione. Fino a quando non vengono reclamati, l'account ha i limiti standard di prova.

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Example

[inline-code-attrs-start title = 'Esempio createAgentTenant'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    CreateAgentTenantBody createAgentTenantBody = new CreateAgentTenantBody(); // CreateAgentTenantBody | 
    try {
      CreateAgentTenantResponse result = apiInstance.createAgentTenant(createAgentTenantBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createAgentTenant");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---