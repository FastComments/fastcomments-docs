## Parameters

| Naam | Type | Locatie | Vereist | Beschrijving |
|------|------|----------|----------|---------------|
| tenantId | string | query | Ja | De tenant waarvoor auditlogs worden gelezen. |
| limit | integer | query | Nee | Hoeveel logs moeten worden geretourneerd. Max 10000, standaard 1000. |
| skip | integer | query | Nee | Hoeveel logs moeten worden overgeslagen, voor paginering. |
| order | string | query | Nee | Sorteerrichting op de gebeurtenistijd. Standaard DESC (nieuwste eerst). |
| after | integer | query | Nee |  |
| before | integer | query | Nee |  |
| username | string | query | Nee | Alleen gebeurtenissen teruggeven die door deze gebruikersnaam zijn uitgevoerd. |
| ip | string | query | Nee | Alleen gebeurtenissen teruggeven van dit IP-adres. |
| crudType | string | query | Nee | Alleen gebeurtenissen van dit type teruggeven: c, r, u, d, of login. |
| resourceName | string | query | Nee | Alleen gebeurtenissen voor deze resource teruggeven, bijv. User of Moderator. |
| targetId | string | query | Nee | Alleen gebeurtenissen waarvan het getroffen object dit id heeft teruggeven. |
| target | string | query | Nee | Hoofdletterongevoelige substring-match op het label van het getroffen object, bijv. een gebruikersnaam of e‑mail. |
| includeManagedTenants | boolean | query | Nee | Ook gebeurtenissen teruggeven van tenants die deze tenant beheert. |

## Respons

Retourneert: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Voorbeeld

[inline-code-attrs-start title = 'getAuditLogs Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Klassen importeren:
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
    
    // API-sleutel autorisatie configureren: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Haal het commentaar weg van de volgende regel om een prefix voor de API-sleutel in te stellen, bijv. "Token" (standaard null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | De tenant waarvoor auditlogs worden gelezen.
    Integer limit = 56; // Integer | Hoeveel logs moeten worden geretourneerd. Max 10000, standaard 1000.
    Integer skip = 56; // Integer | Hoeveel logs moeten worden overgeslagen, voor paginering.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Sorteerrichting op de gebeurtenistijd. Standaard DESC (nieuwste eerst).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Alleen gebeurtenissen teruggeven die door deze gebruikersnaam zijn uitgevoerd.
    String ip = "ip_example"; // String | Alleen gebeurtenissen teruggeven van dit IP-adres.
    String crudType = "crudType_example"; // String | Alleen gebeurtenissen van dit type teruggeven: c, r, u, d, of login.
    String resourceName = "resourceName_example"; // String | Alleen gebeurtenissen voor deze resource teruggeven, bijv. User of Moderator.
    String targetId = "targetId_example"; // String | Alleen gebeurtenissen waarvan het getroffen object dit id heeft teruggeven.
    String target = "target_example"; // String | Hoofdletterongevoelige substring-match op het label van het getroffen object, bijv. een gebruikersnaam of e‑mail.
    Boolean includeManagedTenants = true; // Boolean | Ook gebeurtenissen teruggeven van tenants die deze tenant beheert.
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