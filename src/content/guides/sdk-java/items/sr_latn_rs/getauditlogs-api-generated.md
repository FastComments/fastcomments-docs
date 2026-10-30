## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Tenant za čitanje audit logova. |
| limit | integer | query | No | Koliko logova da se vrati. Maksimum 10000, podrazumevano 1000. |
| skip | integer | query | No | Koliko logova da se preskoči, za paginaciju. |
| order | string | query | No | Smer sortiranja po vremenu događaja. Podrazumevano DESC (najnoviji prvi). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Vraća samo događaje izvršene od strane ovog korisničkog imena. |
| ip | string | query | No | Vraća samo događaje sa ove IP adrese. |
| crudType | string | query | No | Vraća samo događaje ovog tipa: c, r, u, d, ili login. |
| resourceName | string | query | No | Vraća samo događaje za ovaj resurs, npr. User ili Moderator. |
| targetId | string | query | No | Vraća samo događaje čiji pogođeni objekat ima ovaj ID. |
| target | string | query | No | Podudaranje podstringa neosetljivo na veličinu slova na labeli pogođenog objekta, npr. korisničko ime ili email. |
| includeManagedTenants | boolean | query | No | Takođe vraća događaje od tenant-a koje ovaj tenant upravlja. |

## Odgovor

Vraća: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Primer

[inline-code-attrs-start title = 'getAuditLogs Primer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Otkomentarišite sledeću liniju da postavite prefiks za API ključ, npr. "Token" (podrazumevano null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | Tenant za čitanje audit logova.
    Integer limit = 56; // Integer | Koliko logova da se vrati. Maksimum 10000, podrazumevano 1000.
    Integer skip = 56; // Integer | Koliko logova da se preskoči, za paginaciju.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Smer sortiranja po vremenu događaja. Podrazumevano DESC (najnoviji prvi).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Vraća samo događaje izvršene od strane ovog korisničkog imena.
    String ip = "ip_example"; // String | Vraća samo događaje sa ove IP adrese.
    String crudType = "crudType_example"; // String | Vraća samo događaje ovog tipa: c, r, u, d, ili login.
    String resourceName = "resourceName_example"; // String | Vraća samo događaje za ovaj resurs, npr. User ili Moderator.
    String targetId = "targetId_example"; // String | Vraća samo događaje čiji pogođeni objekat ima ovaj ID.
    String target = "target_example"; // String | Podudaranje podstringa neosetljivo na veličinu slova na labeli pogođenog objekta, npr. korisničko ime ili email.
    Boolean includeManagedTenants = true; // Boolean | Takođe vraća događaje od tenant-a koje ovaj tenant upravlja.
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