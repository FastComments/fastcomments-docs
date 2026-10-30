Identifiziert das verwendete Anmeldecredential: den Mandanten, zu dem es gehört, und bei OAuth‑Token den Benutzer, der es autorisiert hat.  
Integrationen verwenden dies, um eine Verbindung zu testen und zu kennzeichnen.

## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|-----|-----|--------------|--------------|
| tenantId | string | query | Ja |  |

## Antwort

Returns: [`GetMeResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetMeResponse.java)

## Beispiel

[inline-code-attrs-start title = 'getMe Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Entfernen Sie das Kommentarzeichen in der folgenden Zeile, um ein Präfix für den API-Schlüssel festzulegen, z. B. "Token" (Standard ist null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    try {
      GetMeResponse result = apiInstance.getMe(tenantId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getMe");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---