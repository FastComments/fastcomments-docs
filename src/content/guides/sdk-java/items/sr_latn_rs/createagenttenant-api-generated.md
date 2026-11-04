Kreira novi probni nalog za AI agenta bez ljudske registracije. Za pozivanje ovoga nije potreban API ključ.

Odgovor sadrži ID tenanta, API ključ koji odmah funkcioniše sa REST API-jem i MCP serverom, kao i URL za preuzimanje naloga. Dajte URL za preuzimanje osobi za koju radite: otvaranje tog URL-a dok je prijavljen na FastComments povezuje nalog sa njom. Nepreuzeti nalozi, kao i njihovi ključevi, brišu se 72 sata nakon kreiranja. Dok se ne preuzme, nalog ima standardna ograničenja probnog perioda.

## Odgovor

Vraća: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Primer

[inline-code-attrs-start title = 'createAgentTenant Primer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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