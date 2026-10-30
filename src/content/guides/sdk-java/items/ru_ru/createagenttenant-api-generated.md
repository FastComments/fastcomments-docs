Создает новую пробную учетную запись для AI‑агента без регистрации человеком. Для вызова не требуется API‑ключ.

Ответ содержит идентификатор арендатора, API‑ключ, который сразу же работает с REST API и сервером MCP, а также URL для привязки. Передайте URL для привязки человеку, для которого вы работаете: открытие его, будучи авторизованным в FastComments, привязывает учетную запись к этому человеку. Непривязанные учетные записи и их ключи удаляются через 72 часа после создания. Пока учетная запись не привязана, она имеет стандартные пробные ограничения.

## Ответ

Возвращает: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример createAgentTenant'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    CreateAgentTenantBody createAgentTenantBody = new CreateAgentTenantBody(); // CreateAgentTenantBody | 
    try {
      CreateAgentTenantResponse result = apiInstance.createAgentTenant(createAgentTenantBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createAgentTenant");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]