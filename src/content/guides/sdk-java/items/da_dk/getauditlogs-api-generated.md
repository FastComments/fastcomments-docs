## Parametre

| Navn | Type | Placering | Påkrævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Lejeren, hvis revisionslogfiler skal læses. |
| limit | integer | query | No | Hvor mange logfiler der skal returneres. Max 10000, standard er 1000. |
| skip | integer | query | No | Hvor mange logfiler der skal springes over, til paginering. |
| order | string | query | No | Sorteringsretning på hændelsestiden. Standard er DESC (nyeste først). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Returner kun hændelser udført af dette brugernavn. |
| ip | string | query | No | Returner kun hændelser fra denne IP-adresse. |
| crudType | string | query | No | Returner kun hændelser af denne type: c, r, u, d eller login. |
| resourceName | string | query | No | Returner kun hændelser for denne ressource, f.eks. Bruger eller Moderator. |
| targetId | string | query | No | Returner kun hændelser, hvor det berørte objekt har dette id. |
| target | string | query | No | Case‑insensitiv delstrengssøgning på det berørte objekts label, f.eks. et brugernavn eller en e‑mail. |
| includeManagedTenants | boolean | query | No | Returner også hændelser fra lejere, som denne lejer administrerer. |

## Svar

Returns: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Eksempel

[inline-code-attrs-start title = 'getAuditLogs Eksempel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer klasser:
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
    
    // Konfigurer API-nøgleautorisation: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Fjern kommentaren på følgende linje for at angive et præfiks til API-nøglen, f.eks. "Token" (standard er null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | Lejeren, hvis revisionslogfiler skal læses.
    Integer limit = 56; // Integer | Hvor mange logfiler der skal returneres. Max 10000, standard er 1000.
    Integer skip = 56; // Integer | Hvor mange logfiler der skal springes over, til paginering.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Sorteringsretning på hændelsestiden. Standard er DESC (nyeste først).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Returner kun hændelser udført af dette brugernavn.
    String ip = "ip_example"; // String | Returner kun hændelser fra denne IP-adresse.
    String crudType = "crudType_example"; // String | Returner kun hændelser af denne type: c, r, u, d eller login.
    String resourceName = "resourceName_example"; // String | Returner kun hændelser for denne ressource, f.eks. Bruger eller Moderator.
    String targetId = "targetId_example"; // String | Returner kun hændelser, hvor det berørte objekt har dette id.
    String target = "target_example"; // String | Case‑insensitiv delstrengssøgning på det berørte objekts label, f.eks. et brugernavn eller en e‑mail.
    Boolean includeManagedTenants = true; // Boolean | Returner også hændelser fra lejere, som denne lejer administrerer.
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