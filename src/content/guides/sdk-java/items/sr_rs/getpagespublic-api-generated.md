Листа страница за tenant. Користи га FChat десктоп клијент да попуни своју листу соба.
Захтева да `enableFChat` буде true у решеној прилагођеној конфигурацији за сваку страницу.
Странице које захтевају SSO се филтрирају у складу са приступним правима група корисника који шаље захтев.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозирни курсор пагинације који је враћен као `nextCursor` из претходног захтева. Везан за исти `sortBy`. |
| limit | integer | query | No | 1..200, подразумевано 50 |
| q | string | query | No | Опционални филтер префикса наслова који не разликује велика/мала слова. |
| sortBy | string | query | No | Редослед сортирања. `updatedAt` (подразумевано, најновији први), `commentCount` (највише коментара први), или `title` (алфабетски). |
| hasComments | boolean | query | No | Ако је true, враћа само странице са најмање једним коментаром. |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Пример

[inline-code-attrs-start title = 'getPagesPublic Пример'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Увези класе:
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
    String cursor = "cursor_example"; // String | Непрозирни курсор пагинације враћен као `nextCursor` из претходног захтева. Везан за исти `sortBy`.
    Integer limit = 56; // Integer | 1..200, подразумевано 50
    String q = "q_example"; // String | Опционални филтер префикса наслова који не разликује велика/мала слова.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Редослед сортирања. `updatedAt` (подразумевано, најновији први), `commentCount` (највише коментара први), или `title` (алфабетски).
    Boolean hasComments = true; // Boolean | Ако је true, враћа само странице са најмање једним коментаром.
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