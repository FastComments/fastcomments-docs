## Parametry

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Tak |  |
| skip | number | query | Nie |  |

## Odpowiedź

Zwraca: [`GetTenantPackages200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetTenantPackages200Response.java)

## Przykład

[inline-code-attrs-start title = 'Przykład getTenantPackages'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importuj klasy:
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
    
    // Skonfiguruj autoryzację klucza API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Odkomentuj następującą linię, aby ustawić prefiks dla klucza API, np. "Token" (domyślnie null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    Double skip = 3.4D; // Double | 
    try {
      GetTenantPackages200Response result = apiInstance.getTenantPackages(tenantId)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getTenantPackages");
      // Kod statusu: 
      System.err.println("Status code: " + e.getCode());
      // Powód: 
      System.err.println("Reason: " + e.getResponseBody());
      // Nagłówki odpowiedzi: 
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]