## Параметри

| Име | Тип | Местоположение | Задължително | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Наемателят, за който да се прочетат журналите за одит. |
| limit | integer | query | No | Колко журнала да се върнат. Максимум 10000, по подразбиране 1000. |
| skip | integer | query | No | Колко журнала да се пропуснат, за странициране. |
| order | string | query | No | Посока на сортиране по времето на събитието. По подразбиране DESC (най-новите първи). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Връща само събития, извършени от това потребителско име. |
| ip | string | query | No | Връща само събития от този IP адрес. |
| crudType | string | query | No | Връща само събития от този тип: c, r, u, d или login. |
| resourceName | string | query | No | Връща само събития за този ресурс, напр. User или Moderator. |
| targetId | string | query | No | Връща само събития, чиито засегнат обект има това ID. |
| target | string | query | No | Съвпадение на подниз без значение на регистъра върху етикета на засегнатия обект, напр. потребителско име или имейл. |
| includeManagedTenants | boolean | query | No | Също така връща събития от наематели, които този наемател управлява. |

## Отговор

Returns: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример за getAuditLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импортиране на класове:
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
    
    // Конфигуриране на упълномощаване с API ключ: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Разкоментирайте следния ред, за да зададете префикс за API ключа, напр. "Token" (по подразбиране null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | Наемателят, за който да се прочетат журналите за одит.
    Integer limit = 56; // Integer | Колко журнала да се върнат. Максимум 10000, по подразбиране 1000.
    Integer skip = 56; // Integer | Колко журнала да се пропуснат, за странициране.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Посока на сортиране по времето на събитието. По подразбиране DESC (най-новите първи).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Връща само събития, извършени от това потребителско име.
    String ip = "ip_example"; // String | Връща само събития от този IP адрес.
    String crudType = "crudType_example"; // String | Връща само събития от този тип: c, r, u, d или login.
    String resourceName = "resourceName_example"; // String | Връща само събития за този ресурс, напр. User или Moderator.
    String targetId = "targetId_example"; // String | Връща само събития, чиито засегнат обект има това ID.
    String target = "target_example"; // String | Съвпадение на подниз без значение на регистъра върху етикета на засегнатия обект, напр. потребителско име или имейл.
    Boolean includeManagedTenants = true; // Boolean | Също така връща събития от наематели, които този наемател управлява.
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