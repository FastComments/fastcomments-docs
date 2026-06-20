Список страниц для арендатора. Используется настольным клиентом FChat для заполнения списка комнат.
Требует, чтобы `enableFChat` было true в разрешённой пользовательской конфигурации для каждой страницы.
Страницы, требующие SSO, фильтруются по доступу групп запрашивающего пользователя.

## Параметры

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`. |
| limit | integer | query | No | 1..200, по умолчанию 50 |
| q | string | query | No | Необязательный регистронезависимый фильтр по префиксу заголовка. |
| sortBy | string | query | No | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала страницы с наибольшим количеством комментариев), или `title` (по алфавиту). |
| hasComments | boolean | query | No | Если true, возвращаются только страницы с хотя бы одним комментарием. |

## Ответ

Возвращает: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Пример

[inline-code-attrs-start title = 'Пример getPagesPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Импорт классов:
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
    String cursor = "cursor_example"; // String | Непрозрачный курсор пагинации, возвращаемый как `nextCursor` в предыдущем запросе. Привязан к тому же `sortBy`.
    Integer limit = 56; // Integer | 1..200, по умолчанию 50
    String q = "q_example"; // String | Необязательный регистронезависимый фильтр по префиксу заголовка.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Порядок сортировки. `updatedAt` (по умолчанию, сначала новые), `commentCount` (сначала страницы с наибольшим количеством комментариев) или `title` (по алфавиту).
    Boolean hasComments = true; // Boolean | Если true, возвращаются только страницы с хотя бы одним комментарием.
    try {
      GetPublicPagesResponse result = apiInstance.getPagesPublic(tenantId)
            .cursor(cursor)
            .limit(limit)
            .q(q)
            .sortBy(sortBy)
            .hasComments(hasComments)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getPagesPublic");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]