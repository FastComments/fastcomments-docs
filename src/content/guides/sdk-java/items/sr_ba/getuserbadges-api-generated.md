## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| userId | string | query | No |  |
| badgeId | string | query | No |  |
| type | number | query | No |  |
| displayedOnComments | boolean | query | No |  |
| limit | number | query | No |  |
| skip | number | query | No |  |

## Odgovor

Vraća: [`GetUserBadges200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetUserBadges200Response.java)

## Primer

[inline-code-attrs-start title = 'Primer getUserBadges'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvezi klase:
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
    
    // Konfiguriši autorizaciju API ključa: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Otkomentariši sledeću liniju da postaviš prefiks za API ključ, npr. "Token" (zadano: null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String userId = "userId_example"; // String | 
    String badgeId = "badgeId_example"; // String | 
    Double type = 3.4D; // Double | 
    Boolean displayedOnComments = true; // Boolean | 
    Double limit = 3.4D; // Double | 
    Double skip = 3.4D; // Double | 
    try {
      GetUserBadges200Response result = apiInstance.getUserBadges(tenantId)
            .userId(userId)
            .badgeId(badgeId)
            .type(type)
            .displayedOnComments(displayedOnComments)
            .limit(limit)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getUserBadges");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]