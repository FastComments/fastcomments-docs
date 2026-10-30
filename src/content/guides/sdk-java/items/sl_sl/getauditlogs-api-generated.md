## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Najemnik, za katerega se berejo revizijski dnevniki. |
| limit | integer | query | No | Koliko dnevnikov naj se vrne. Največ 10000, privzeto 1000. |
| skip | integer | query | No | Koliko dnevnikov naj se preskoči, za paginacijo. |
| order | string | query | No | Smer razvrščanja po času dogodka. Privzeto DESC (najprej najnovejše). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Vrni le dogodke, ki jih je izvedel to uporabniško ime. |
| ip | string | query | No | Vrni le dogodke s tega IP naslova. |
| crudType | string | query | No | Vrni le dogodke tega tipa: c, r, u, d ali login. |
| resourceName | string | query | No | Vrni le dogodke za ta vir, npr. Uporabnik ali Moderator. |
| targetId | string | query | No | Vrni le dogodke, katerih prizadeti objekt ima ta ID. |
| target | string | query | No | Neobčutljivo na velikost črk iskanje podniza v oznaki prizadetega objekta, npr. uporabniško ime ali e-pošto. |
| includeManagedTenants | boolean | query | No | Vrni tudi dogodke od najemnikov, ki jih ta najemnik upravlja. |

## Odziv

Vrne: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Primer

[inline-code-attrs-start title = 'Primer getAuditLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvozi razrede:
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
    
    // Nastavi avtorizacijo API ključa: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Odkomentirajte naslednjo vrstico, da nastavite predpono za API ključ, npr. "Token" (privzeto null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | Najemnik, za katerega se berejo revizijski dnevniki.
    Integer limit = 56; // Integer | Koliko dnevnikov naj se vrne. Največ 10000, privzeto 1000.
    Integer skip = 56; // Integer | Koliko dnevnikov naj se preskoči, za paginacijo.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Smer razvrščanja po času dogodka. Privzeto DESC (najprej najnovejše).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Vrni le dogodke, ki jih je izvedel to uporabniško ime.
    String ip = "ip_example"; // String | Vrni le dogodke s tega IP naslova.
    String crudType = "crudType_example"; // String | Vrni le dogodke tega tipa: c, r, u, d ali login.
    String resourceName = "resourceName_example"; // String | Vrni le dogodke za ta vir, npr. Uporabnik ali Moderator.
    String targetId = "targetId_example"; // String | Vrni le dogodke, katerih prizadeti objekt ima ta ID.
    String target = "target_example"; // String | Neobčutljivo na velikost črk iskanje podniza v oznaki prizadetega objekta, npr. uporabniško ime ali e-pošto.
    Boolean includeManagedTenants = true; // Boolean | Vrni tudi dogodke od najemnikov, ki jih ta najemnik upravlja.
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