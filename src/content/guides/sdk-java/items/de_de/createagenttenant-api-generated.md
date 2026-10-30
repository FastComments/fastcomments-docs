Erstellt ein neues Testkonto für einen KI‑Agenten ohne menschliche Anmeldung. Es wird kein API‑Schlüssel benötigt, um dies aufzurufen.

Die Antwort enthält die Mandanten‑ID, einen API‑Schlüssel, der sofort gegen die REST‑API und den MCP‑Server funktioniert, sowie eine Claim‑URL. Geben Sie die Claim‑URL an die Person weiter, für die Sie arbeiten: Das Öffnen der URL, während Sie bei FastComments angemeldet sind, verbindet das Konto mit ihr. Nicht beanspruchte Konten und deren Schlüssel werden 72 Stunden nach Erstellung gelöscht. Bis zur Beanspruchung hat das Konto die üblichen Testlimits.

## Antwort

Rückgabe: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Beispiel

[inline-code-attrs-start title = 'createAgentTenant Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    CreateAgentTenantBody createAgentTenantBody = new CreateAgentTenantBody(); // CreateAgentTenantBody | 
    try {
      CreateAgentTenantResponse result = apiInstance.createAgentTenant(createAgentTenantBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createAgentTenant");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]