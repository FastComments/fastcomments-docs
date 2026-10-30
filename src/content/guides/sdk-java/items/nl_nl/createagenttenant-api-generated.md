---
Maakt een nieuw proefaccount aan voor een AI‑agent zonder menselijke registratie. Er is geen API‑sleutel nodig om dit aan te roepen.

De respons bevat de tenant‑id, een API‑sleutel die onmiddellijk werkt tegen de REST‑API en de MCP‑server, en een claim‑URL. Geef de claim‑URL aan de persoon waarvoor je werkt: het openen ervan terwijl je bent ingelogd op FastComments koppelt het account aan hen. Niet‑geclaimde accounts, en hun sleutels, worden 72 uur na creatie verwijderd. Totdat ze geclaimd zijn, heeft het account de standaard proeflimieten.

## Respons

Retourneert: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Voorbeeld

[inline-code-attrs-start title = 'createAgentTenant Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importeer klassen:
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