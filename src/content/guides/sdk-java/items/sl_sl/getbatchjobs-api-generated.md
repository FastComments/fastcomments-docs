Skupinski posli najemnika (uvozi, izvozi, migracije strani...), najnovejši najprej.

## Parametri

| Ime | Vrsta | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| type | string | query | No | Samo posli te vrste, na primer migrate-pages. |
| status | string | query | No | Samo posli v tem statusu: requested, running, success ali failed. |
| skip | integer | query | No | Koliko poslov naj se preskoči, za paginacijo. |

## Odgovor

Vrne: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetBatchJobsAPIResponse.java)

## Primer

[inline-code-attrs-start title = 'Primer getBatchJobs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvozi razrede:
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
    
    // Nastavi avtorizacijo API ključa: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Odkomentirajte naslednjo vrstico, da nastavite predpono za API ključ, npr. "Token" (privzeto null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String type = "type_example"; // String | Samo posli te vrste, na primer migrate-pages.
    String status = "status_example"; // String | Samo posli v tem statusu: requested, running, success ali failed.
    Integer skip = 56; // Integer | Koliko poslov naj se preskoči, za paginacijo.
    try {
      GetBatchJobsAPIResponse result = apiInstance.getBatchJobs(tenantId)
            .type(type)
            .status(status)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getBatchJobs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]