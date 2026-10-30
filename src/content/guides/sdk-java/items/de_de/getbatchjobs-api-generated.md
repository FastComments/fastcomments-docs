Die Batch‑Jobs des Mandanten (Importe, Exporte, Seitenmigrationen…), neueste zuerst.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|-----|--------------|--------------|
| tenantId | string | query | Ja |  |
| type | string | query | Nein | Nur Jobs dieses Typs, zum Beispiel migrate-pages. |
| status | string | query | Nein | Nur Jobs mit diesem Status: requested, running, success oder failed. |
| skip | integer | query | Nein | Wie viele Jobs übersprungen werden sollen, für Paging. |

## Antwort

Rückgabe: [`GetBatchJobsAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetBatchJobsAPIResponse.java)

## Beispiel

[inline-code-attrs-start title = 'getBatchJobs Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // API‑Schlüssel‑Authentifizierung konfigurieren: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Entfernen Sie das Kommentarzeichen in der folgenden Zeile, um ein Präfix für den API‑Schlüssel festzulegen, z. B. "Token" (Standardwert ist null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String type = "type_example"; // String | Nur Jobs dieses Typs, zum Beispiel migrate-pages.
    String status = "status_example"; // String | Nur Jobs mit diesem Status: requested, running, success oder failed.
    Integer skip = 56; // Integer | Wie viele Jobs übersprungen werden sollen, für Paging.
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