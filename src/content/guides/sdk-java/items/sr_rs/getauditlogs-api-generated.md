## Parameters

| Име | Тип | Локација | Обавезно | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Тенант за који се читају дневници ревизије. |
| limit | integer | query | No | Колико дневника да се врати. Максимум 10000, подразумевано 1000. |
| skip | integer | query | No | Колико дневника да се прескочи, за пагинацију. |
| order | string | query | No | Смер сортирања по времену догађаја. Подразумевано DESC (најновије прво). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Враћа само догађаје извршене од стране овог корисничког имена. |
| ip | string | query | No | Враћа само догађаје са ове IP адресе. |
| crudType | string | query | No | Враћа само догађаје овог типа: c, r, u, d, или login. |
| resourceName | string | query | No | Враћа само догађаје за овај ресурс, нпр. User или Moderator. |
| targetId | string | query | No | Враћа само догађаје чији је утицани објекат има овај ID. |
| target | string | query | No | Поклапање подстринга без разликовања величине слова на ознаци утицаног објекта, нпр. корисничко име или е‑mail. |
| includeManagedTenants | boolean | query | No | Такође враћа догађаје из тенаната које овај тенант управља. |

## Response

Враћа: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Example

[inline-code-attrs-start title = 'Primer getAuditLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Увези класе:
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
    
    // Подеси ауторизацију API кључа: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Одкоментариши следећи ред да поставиш префикс за API кључ, нпр. "Token" (подразумевано null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | Тенант за који се читају дневници ревизије.
    Integer limit = 56; // Integer | Колико дневника да се врати. Максимум 10000, подразумевано 1000.
    Integer skip = 56; // Integer | Колико дневника да се прескочи, за пагинацију.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Смер сортирања по времену догађаја. Подразумевано DESC (најновије прво).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Враћа само догађаје извршене од стране овог корисничког имена.
    String ip = "ip_example"; // String | Враћа само догађаје са ове IP адресе.
    String crudType = "crudType_example"; // String | Враћа само догађаје овог типа: c, r, u, d, или login.
    String resourceName = "resourceName_example"; // String | Враћа само догађаје за овај ресурс, нпр. User или Moderator.
    String targetId = "targetId_example"; // String | Враћа само догађаје чији је утицани објекат има овај ID.
    String target = "target_example"; // String | Поклапање подстринга без разликовања величине слова на ознаци утицаног објекта, нпр. корисничко име или е‑mail.
    Boolean includeManagedTenants = true; // Boolean | Такође враћа догађаје из тенаната које овај тенант управља.
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