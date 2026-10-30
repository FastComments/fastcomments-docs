Ustvari nov preizkusni račun za AI agenta brez človeške registracije. Za klic tega ni potrebnega API ključa.

Odgovor vsebuje ID najemnika, API ključ, ki takoj deluje proti REST API-ju in strežniku MCP, ter URL za prevzem. URL za prevzem dajte osebi, za katero delate: odpiranje tega URL-ja med prijavo v FastComments poveže račun z njo. Neprevzeti računi in njihovi ključi se izbrišejo 72 ur po ustvarjanju. Dokler ni prevzet, ima račun standardne preizkusne omejitve.

## Response

Returns: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Example

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