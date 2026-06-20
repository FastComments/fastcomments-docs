## Параметри

| Назва | Тип | Розташування | Обов'язковий | Опис |
|------|------|----------|----------|-------------|
| tenantId | string | шлях | Так |  |
| locale | string | параметр запиту | Ні |  |
| rating | string | параметр запиту | Ні |  |
| page | number | параметр запиту | Ні |  |

## Відповідь

Повертає: [`GetGifsTrendingResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetGifsTrendingResponse.java)

## Приклад

[inline-code-attrs-start title = 'Приклад getGifsTrending'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String locale = "locale_example"; // String | 
    String rating = "rating_example"; // String | 
    Double page = 3.4D; // Double | 
    try {
      GetGifsTrendingResponse result = apiInstance.getGifsTrending(tenantId)
            .locale(locale)
            .rating(rating)
            .page(page)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getGifsTrending");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]