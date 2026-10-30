## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Der Mandant, für den die Audit-Logs gelesen werden sollen. |
| limit | integer | query | No | Wie viele Logs zurückgegeben werden sollen. Maximal 10000, Standardwert 1000. |
| skip | integer | query | No | Wie viele Logs übersprungen werden sollen, für die Seitennavigation. |
| order | string | query | No | Sortierrichtung nach der Ereigniszeit. Standard ist DESC (neueste zuerst). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Nur Ereignisse zurückgeben, die von diesem Benutzernamen durchgeführt wurden. |
| ip | string | query | No | Nur Ereignisse von dieser IP-Adresse zurückgeben. |
| crudType | string | query | No | Nur Ereignisse dieses Typs zurückgeben: c, r, u, d oder login. |
| resourceName | string | query | No | Nur Ereignisse für diese Ressource zurückgeben, z. B. User oder Moderator. |
| targetId | string | query | No | Nur Ereignisse zurückgeben, deren betroffene Objekt diese ID hat. |
| target | string | query | No | Fallunabhängige Teilzeichenkettenübereinstimmung mit dem Label des betroffenen Objekts, z. B. ein Benutzername oder eine E‑Mail. |
| includeManagedTenants | boolean | query | No | Auch Ereignisse von Mandanten zurückgeben, die dieser Mandant verwaltet. |

## Antwort

Rückgabe: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Beispiel

[inline-code-attrs-start title = 'getAuditLogs Beispiel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Kommentieren Sie die folgende Zeile aus, um ein Präfix für den API-Schlüssel festzulegen, z. B. "Token" (Standardwert ist null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | Der Mandant, für den die Audit-Logs gelesen werden sollen.
    Integer limit = 56; // Integer | Wie viele Logs zurückgegeben werden sollen. Maximal 10000, Standardwert 1000.
    Integer skip = 56; // Integer | Wie viele Logs übersprungen werden sollen, für die Seitennavigation.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Sortierrichtung nach der Ereigniszeit. Standard ist DESC (neueste zuerst).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Nur Ereignisse zurückgeben, die von diesem Benutzernamen durchgeführt wurden.
    String ip = "ip_example"; // String | Nur Ereignisse von dieser IP-Adresse zurückgeben.
    String crudType = "crudType_example"; // String | Nur Ereignisse dieses Typs zurückgeben: c, r, u, d oder login.
    String resourceName = "resourceName_example"; // String | Nur Ereignisse für diese Ressource zurückgeben, z. B. User oder Moderator.
    String targetId = "targetId_example"; // String | Nur Ereignisse zurückgeben, deren betroffene Objekt diese ID hat.
    String target = "target_example"; // String | Fallunabhängige Teilzeichenkettenübereinstimmung mit dem Label des betroffenen Objekts, z. B. ein Benutzername oder eine E‑Mail.
    Boolean includeManagedTenants = true; // Boolean | Auch Ereignisse von Mandanten zurückgeben, die dieser Mandant verwaltet.
    try {
      GetAuditLogsResponse result = apiInstance.getAuditLogs(tenantId)
            .limit(limit)
            .skip(skip)
            .order(order)
            .after(after)
            .before(before)
            .username(username)
            .ip(ip)
            .crudType(crudType)
            .resourceName(resourceName)
            .targetId(targetId)
            .target(target)
            .includeManagedTenants(includeManagedTenants)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#getAuditLogs");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]