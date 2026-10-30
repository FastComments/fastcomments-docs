## Parameters

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|------|
| tenantId | string | query | Da | Najam za kojeg se čitaju revizijski zapisi. |
| limit | integer | query | Ne | Koliko zapisa vratiti. Maks 10000, zadano 1000. |
| skip | integer | query | Ne | Koliko zapisa preskočiti, za paginaciju. |
| order | string | query | Ne | Smjer sortiranja po vremenu događaja. Zadano DESC (najnoviji prvi). |
| after | integer | query | Ne |  |
| before | integer | query | Ne |  |
| username | string | query | Ne | Vraća samo događaje izvršene od strane ovog korisničkog imena. |
| ip | string | query | Ne | Vraća samo događaje s ove IP adrese. |
| crudType | string | query | Ne | Vraća samo događaje ovog tipa: c, r, u, d ili login. |
| resourceName | string | query | Ne | Vraća samo događaje za ovaj resurs, npr. User ili Moderator. |
| targetId | string | query | Ne | Vraća samo događaje čiji je zahvaćeni objekt s ovim ID-om. |
| target | string | query | Ne | Podudaranje podstringa neosjetljivo na veličinu slova na oznaci zahvaćenog objekta, npr. korisničko ime ili email. |
| includeManagedTenants | boolean | query | Ne | Također vraća događaje od najma koje ovaj najam upravlja. |

## Response

Returns: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Example

[inline-code-attrs-start title = 'Primjer getAuditLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    
    // Configure API key authorization: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Uncomment the following line to set a prefix for the API key, e.g. "Token" (defaults to null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | Najam za kojeg se čitaju revizijski zapisi.
    Integer limit = 56; // Integer | Koliko zapisa vratiti. Maks 10000, zadano 1000.
    Integer skip = 56; // Integer | Koliko zapisa preskočiti, za paginaciju.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Smjer sortiranja po vremenu događaja. Zadano DESC (najnoviji prvi).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Vraća samo događaje izvršene od strane ovog korisničkog imena.
    String ip = "ip_example"; // String | Vraća samo događaje s ove IP adrese.
    String crudType = "crudType_example"; // String | Vraća samo događaje ovog tipa: c, r, u, d ili login.
    String resourceName = "resourceName_example"; // String | Vraća samo događaje za ovaj resurs, npr. User ili Moderator.
    String targetId = "targetId_example"; // String | Vraća samo događaje čiji je zahvaćeni objekt s ovim ID-om.
    String target = "target_example"; // String | Podudaranje podstringa neosjetljivo na veličinu slova na oznaci zahvaćenog objekta, npr. korisničko ime ili email.
    Boolean includeManagedTenants = true; // Boolean | Također vraća događaje od najma koje ovaj najam upravlja.
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