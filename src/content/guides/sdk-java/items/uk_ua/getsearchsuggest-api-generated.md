## Параметри

| Назва | Тип | Розташування | Обов'язково | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | query | Так |  |
| text-search | string | query | Ні |  |
| sso | string | query | Ні |  |

## Відповідь

Повертає: [`ModerationSuggestResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationSuggestResponse.java)

## Приклад

[inline-code-attrs-start title = 'Приклад getSearchSuggest'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.ModerationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    ModerationApi apiInstance = new ModerationApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String textSearch = "textSearch_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      ModerationSuggestResponse result = apiInstance.getSearchSuggest(tenantId)
            .textSearch(textSearch)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Виключення при виклику ModerationApi#getSearchSuggest");
      System.err.println("Код статусу: " + e.getCode());
      System.err.println("Причина: " + e.getResponseBody());
      System.err.println("Заголовки відповіді: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]