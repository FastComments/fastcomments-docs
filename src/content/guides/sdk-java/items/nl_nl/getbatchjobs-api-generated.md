The tenant's batch jobs (imports, exports, page migrations...), newest first.

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|-----------|--------------|
| tenantId | string | query | Ja |  |
| type | string | query | Nee | Alleen taken van dit type, bijvoorbeeld migrate-pages. |
| status | string | query | Nee | Alleen taken in deze status: requested, running, success of failed. |
| skip | integer | query | Nee | Hoeveel taken over te slaan, voor paginering. |

## Response

Retourneert: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetBatchJobsAPIResponse.java)

## Example

[inline-code-attrs-start title = 'getBatchJobs Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Verwijder commentaar op de volgende regel om een prefix voor de API-sleutel in te stellen, bijv. "Token" (standaard null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String type = "type_example"; // String | Alleen taken van dit type, bijvoorbeeld migrate-pages.
    String status = "status_example"; // String | Alleen taken in deze status: requested, running, success of failed.
    Integer skip = 56; // Integer | Hoeveel taken over te slaan, voor paginering.
    try {
      GetBatchJobsAPIResponse result = apiInstance.getBatchJobs(tenantId)
            .type(type)
            .status(status)
            .skip(skip)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Uitzondering bij het aanroepen van DefaultApi#getBatchJobs");
      System.err.println("Statuscode: " + e.getCode());
      System.err.println("Reden: " + e.getResponseBody());
      System.err.println("Responsheaders: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]