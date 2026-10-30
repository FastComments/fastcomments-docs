Stvara novi probni račun za AI agenta bez ljudske registracije. Nije potreban API ključ za pozivanje ove funkcije.

Odgovor sadrži ID najmodavca, API ključ koji odmah funkcionira protiv REST API‑ja i
MCP poslužitelja, te URL za preuzimanje. Dajte URL za preuzimanje čovjeku za kojeg radite: otvaranjem dok ste
prijavljeni u FastComments, račun se povezuje s njima. Nepreuzeti računi i njihovi ključevi
brišu se 72 sata nakon stvaranja. Dok se ne preuzme, račun ima standardna probna ograničenja.

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Example

[inline-code-attrs-start title = 'createAgentTenant Primjer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvezi klase:
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