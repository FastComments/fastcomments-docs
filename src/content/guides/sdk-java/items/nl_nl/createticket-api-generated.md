## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | Yes |  |

## Response

Retourneert: [`CreateTicket200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateTicket200Response.java)

## Voorbeeld

[inline-code-attrs-start title = 'createTicket Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importeer klassen:
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
    
    // Configureer API-sleutelautorisatie: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Haal de commentaarstreep voor de volgende regel weg om een voorvoegsel voor de API-sleutel in te stellen, bijv. "Token" (standaard is null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String userId = "userId_example"; // String | 
    CreateTicketBody createTicketBody = new CreateTicketBody(); // CreateTicketBody | 
    try {
      CreateTicket200Response result = apiInstance.createTicket(tenantId, userId, createTicketBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createTicket");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]