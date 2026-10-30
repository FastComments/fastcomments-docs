## Параметры

| Имя | Тип | Расположение | Обязательно | Описание |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes | Тенант, для которого читаются журналы аудита. |
| limit | integer | query | No | Сколько журналов вернуть. Максимум 10000, по умолчанию 1000. |
| skip | integer | query | No | Сколько журналов пропустить, для постраничного вывода. |
| order | string | query | No | Направление сортировки по времени события. По умолчанию DESC (сначала новые). |
| after | integer | query | No |  |
| before | integer | query | No |  |
| username | string | query | No | Возвращать только события, выполненные этим именем пользователя. |
| ip | string | query | No | Возвращать только события с этого IP-адреса. |
| crudType | string | query | No | Возвращать только события этого типа: c, r, u, d или login. |
| resourceName | string | query | No | Возвращать только события для этого ресурса, например User или Moderator. |
| targetId | string | query | No | Возвращать только события, у которых затронутый объект имеет этот идентификатор. |
| target | string | query | No | Поиск подстроки без учёта регистра в метке затронутого объекта, например имя пользователя или email. |
| includeManagedTenants | boolean | query | No | Также возвращать события от тенантов, которыми управляет данный тенант. |

## Ответ

Возвращает: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример getAuditLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импортировать классы:
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
    
    // Настроить авторизацию API-ключа: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Раскомментировать следующую строку, чтобы установить префикс для API-ключа, например "Token" (по умолчанию null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | Тенант, для которого читаются журналы аудита.
    Integer limit = 56; // Integer | Сколько журналов вернуть. Максимум 10000, по умолчанию 1000.
    Integer skip = 56; // Integer | Сколько журналов пропустить, для постраничного вывода.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Направление сортировки по времени события. По умолчанию DESC (сначала новые).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Возвращать только события, выполненные этим именем пользователя.
    String ip = "ip_example"; // String | Возвращать только события с этого IP-адреса.
    String crudType = "crudType_example"; // String | Возвращать только события этого типа: c, r, u, d или login.
    String resourceName = "resourceName_example"; // String | Возвращать только события для этого ресурса, например User или Moderator.
    String targetId = "targetId_example"; // String | Возвращать только события, у которых затронутый объект имеет этот идентификатор.
    String target = "target_example"; // String | Поиск подстроки без учёта регистра в метке затронутого объекта, например имя пользователя или email.
    Boolean includeManagedTenants = true; // Boolean | Также возвращать события от тенантов, которыми управляет данный тенант.
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