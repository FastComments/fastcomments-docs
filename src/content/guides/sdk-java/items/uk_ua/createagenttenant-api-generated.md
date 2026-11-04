Створює новий пробний обліковий запис для AI-агента без реєстрації людиною. Для виклику цього методу API-ключ не потрібен.

Відповідь містить ідентифікатор тенанта, API-ключ, який одразу працює з REST API та
MCP-сервером, а також URL для підтвердження. Передайте URL підтвердження людині, на яку ви працюєте: відкриття його,
будучи залогіненим у FastComments, приєднує обліковий запис до неї. Непідтверджені облікові записи та їхні ключі
видаляються через 72 години після створення. До підтвердження обліковий запис має стандартні обмеження пробного періоду.

## Відповідь

Повертає: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Приклад

[inline-code-attrs-start title = 'Приклад createAgentTenant'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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