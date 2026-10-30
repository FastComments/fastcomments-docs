## Параметри

| Назва | Тип | Розташування | Обов’язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | path | Так |  |
| urlId | string | query | Так |  |
| title | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`CreateV1PageReact`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateV1PageReact.java)

## Приклад

[inline-code-attrs-start title = 'Приклад createV1PageReact'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Імпорт класів:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.PublicApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    PublicApi apiInstance = new PublicApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    String title = "title_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      CreateV1PageReact result = apiInstance.createV1PageReact(tenantId, urlId)
            .title(title)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Виняток під час виклику PublicApi#createV1PageReact");
      System.err.println("Код статусу: " + e.getCode());
      System.err.println("Причина: " + e.getResponseBody());
      System.err.println("Заголовки відповіді: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---