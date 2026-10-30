## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| commentId | string | path | Ja |  |

## Respons

Retourneert: [`GetPollResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPollResponse.java)

## Voorbeeld

[inline-code-attrs-start title = 'getPoll Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Configureer API-sleutel autorisatie: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Verwijder de commentaar op de volgende regel om een prefix voor de API-sleutel in te stellen, bijv. "Token" (standaard null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String commentId = "commentId_example"; // String | 
    try {
      GetPollResponse result = apiInstance.getPoll(tenantId, commentId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Uitzondering bij het aanroepen van DefaultApi#getPoll");
      System.err.println("Statuscode: " + e.getCode());
      System.err.println("Reden: " + e.getResponseBody());
      System.err.println("Responsheaders: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]