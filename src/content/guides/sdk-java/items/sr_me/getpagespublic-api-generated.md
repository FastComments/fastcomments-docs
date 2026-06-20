Листа страница за тенанта. Користи се од стране FChat десктоп клијента за попуњавање његове листе соба.
Потребно је да `enableFChat` буде true у решеном прилагођеном конфигу за сваку страницу.
Странице које захтијевају SSO биће филтриране у складу са приступом групе корисника који прави захтев.

## Параметри

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозирни курсор за пагинацију враћен као `nextCursor` из претходног захтева. Повезан са истим `sortBy`. |
| limit | integer | query | No | 1..200, подразумевано 50 |
| q | string | query | No | Опциони филтер префикса наслова, неосетљив на регистар. |
| sortBy | string | query | No | Редослед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (азбучно). |
| hasComments | boolean | query | No | Ако је true, враћају се само странице које имају бар један коментар. |

## Одговор

Враћа: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Пример

[inline-code-attrs-start title = 'getPagesPublic Пример'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Увоз класа:
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
    String cursor = "cursor_example"; // String | Непрозирни курсор за пагинацију враћен као `nextCursor` из претходног захтева. Повезан је са истим `sortBy`.
    Integer limit = 56; // Integer | 1..200, подразумевано 50
    String q = "q_example"; // String | Опциони филтер префикса наслова, неосетљив на регистар.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Редослед сортирања. `updatedAt` (подразумевано, најновије прво), `commentCount` (највише коментара прво), или `title` (азбучно).
    Boolean hasComments = true; // Boolean | Ако је true, враћају се само странице које имају бар један коментар.
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