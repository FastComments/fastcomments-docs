Opretter en ny prøvekonto til en AI-agent uden menneskelig tilmelding. Der kræves ingen API-nøgle for at kalde dette.

Svaret indeholder tenant-id'et, en API-nøgle, der fungerer med det samme mod REST API'en og MCP-serveren, samt en claim-URL. Giv claim-URL'en til den person, du arbejder for: når den åbnes, mens man er logget ind på FastComments, tilknyttes kontoen til denne person. Ikke-claimede konti og deres nøgler slettes 72 timer efter oprettelsen. Indtil kontoen claimes, gælder de standard begrænsninger for prøvekonti.

## Response

Returnerer: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Eksempel

[inline-code-attrs-start title = 'createAgentTenant Eksempel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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