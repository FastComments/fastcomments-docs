## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так | Тенант, для якого читаються журнали аудиту. |
| limit | integer | query | Ні | Кількість журналів, які потрібно повернути. Макс 10000, за замовчуванням 1000. |
| skip | integer | query | Ні | Кількість журналів, які слід пропустити, для пагінації. |
| order | string | query | Ні | Напрямок сортування за часом події. За замовчуванням DESC (найновіші спочатку). |
| after | integer | query | Ні |  |
| before | integer | query | Ні |  |
| username | string | query | Ні | Повернути лише події, виконані цим ім'ям користувача. |
| ip | string | query | Ні | Повернути лише події з цієї IP-адреси. |
| crudType | string | query | Ні | Повернути лише події цього типу: c, r, u, d або login. |
| resourceName | string | query | Ні | Повернути лише події для цього ресурсу, напр., User або Moderator. |
| targetId | string | query | Ні | Повернути лише події, у яких у ураженого об’єкта є цей ідентифікатор. |
| target | string | query | Ні | Пошук підрядка без урахування регістру в мітці ураженого об’єкта, напр., ім'я користувача або електронна пошта. |
| includeManagedTenants | boolean | query | Ні | Також повернути події від тенантів, якими керує цей тенант. |

## Відповідь

Повертає: [`GetAuditLogsResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetAuditLogsResponse.java)

## Приклад

[inline-code-attrs-start title = 'Приклад getAuditLogs'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Імпорт класів:
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
    
    // Налаштування авторизації API-ключа: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Розкоментуйте наступний рядок, щоб встановити префікс для API-ключа, напр., "Token" (за замовчуванням null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | Тенант, для якого читаються журнали аудиту.
    Integer limit = 56; // Integer | Кількість журналів, які потрібно повернути. Макс 10000, за замовчуванням 1000.
    Integer skip = 56; // Integer | Кількість журналів, які слід пропустити, для пагінації.
    SORTDIR order = SORTDIR.fromValue("ASC"); // SORTDIR | Напрямок сортування за часом події. За замовчуванням DESC (найновіші спочатку).
    Long after = 56L; // Long | 
    Long before = 56L; // Long | 
    String username = "username_example"; // String | Повернути лише події, виконані цим ім'ям користувача.
    String ip = "ip_example"; // String | Повернути лише події з цієї IP-адреси.
    String crudType = "crudType_example"; // String | Повернути лише події цього типу: c, r, u, d або login.
    String resourceName = "resourceName_example"; // String | Повернути лише події для цього ресурсу, напр., User або Moderator.
    String targetId = "targetId_example"; // String | Повернути лише події, у яких у ураженого об’єкта є цей ідентифікатор.
    String target = "target_example"; // String | Пошук підрядка без урахування регістру в мітці ураженого об’єкта, напр., ім'я користувача або електронна пошта.
    Boolean includeManagedTenants = true; // Boolean | Також повернути події від тенантів, якими керує цей тенант.
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