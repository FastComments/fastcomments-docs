Aggregates documents by grouping them (if groupBy is provided) and applying multiple operations.
Different operations (e.g. sum, countDistinct, avg, etc.) are supported.

## Parameter

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Ja |  |
| parentTenantId | string | query | Nein |  |
| includeStats | boolean | query | Nein |  |

## Antwort

Gibt zurück: [`AggregationResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/AggregationResponse.java)

## Beispiel

[inline-code-attrs-start title = 'aggregate Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Klassen importieren:
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
    
    // API-Schlüssel-Authentifizierung konfigurieren: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Kommentieren Sie die folgende Zeile aus, um ein Präfix für den API-Schlüssel zu setzen, z.B. "Token" (Standard ist null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    AggregationRequest aggregationRequest = new AggregationRequest(); // AggregationRequest | 
    String parentTenantId = "parentTenantId_example"; // String | 
    Boolean includeStats = true; // Boolean | 
    try {
      AggregationResponse result = apiInstance.aggregate(tenantId, aggregationRequest)
            .parentTenantId(parentTenantId)
            .includeStats(includeStats)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#aggregate");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]