Recent comments in exactly the shape webhook deliveries use, for building integrations (for example Zapier
sample data). Every event delivers the same comment object, so `event` only has to be valid.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| event | string | query | No |  |
| limit | number | query | No |  |

## Response

Returns: [`GetWebhookSamplePayloadsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetWebhookSamplePayloadsResponse.java)

## Example

[inline-code-attrs-start title = 'Beispiel für getWebhookSamplePayloads'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Entfernen Sie das Kommentarzeichen in der folgenden Zeile, um ein Präfix für den API-Schlüssel festzulegen, z. B. "Token" (Standardwert ist null)
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
      // Ausnahme beim Aufruf von DefaultApi#getWebhookSamplePayloads
      System.err.println("Exception when calling DefaultApi#getWebhookSamplePayloads");
      System.err.println("Statuscode: " + e.getCode());
      System.err.println("Grund: " + e.getResponseBody());
      System.err.println("Antwort-Header: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]