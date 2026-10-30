Recent comments in exactly the shape webhook deliveries use, for building integrations (for example Zapier sample data). Every event delivers the same comment object, so `event` only has to be valid.

## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Yes |  |
| event | string | query | No |  |
| limit | number | query | No |  |

## Response

Vraća: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetWebhookSamplePayloadsResponse.java)

## Primer

[inline-code-attrs-start title = 'getWebhookSamplePayloads Primer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Odkomentarišite sledeću liniju da postavite prefiks za API ključ, npr. "Token" (podrazumevano null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String |
    WebhookEventName event = WebhookEventName.fromValue("comment-created"); // WebhookEventName |
    Double limit = 3.4D; // Double |
    try {
      GetWebhookSamplePayloadsResponse result = apiInstance.getWebhookSamplePayloads(tenantId)
            .event(event)
            .limit(limit)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Izuzetak prilikom poziva DefaultApi#getWebhookSamplePayloads");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]