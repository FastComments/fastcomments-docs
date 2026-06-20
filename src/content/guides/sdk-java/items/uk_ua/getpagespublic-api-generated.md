Список сторінок для тенанта. Використовується клієнтом FChat для настільних систем, щоб заповнити список кімнат.
Потребує, щоб `enableFChat` було встановлено в true у отриманій користувацькій конфігурації для кожної сторінки.
Сторінки, які вимагають SSO, фільтруються відповідно до доступу групи користувача, що робить запит.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Непрозорий курсор пагінації, повернутий як `nextCursor` у попередньому запиті. Пов'язаний з тим же `sortBy`. |
| limit | integer | query | No | 1..200, за замовчуванням 50 |
| q | string | query | No | Необов'язковий префікс-фільтр заголовка, нечутливий до регістру. |
| sortBy | string | query | No | Порядок сортування. `updatedAt` (за замовчуванням, спочатку найновіші), `commentCount` (спочатку сторінки з найбільшою кількістю коментарів), або `title` (за абеткою). |
| hasComments | boolean | query | No | Якщо true, повернути тільки сторінки з принаймні одним коментарем. |

## Response

Повертає: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Example

[inline-code-attrs-start title = 'Приклад getPagesPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String cursor = "cursor_example"; // String | Непрозорий курсор пагінації, повернутий як `nextCursor` у попередньому запиті. Пов'язаний з тим же `sortBy`.
    Integer limit = 56; // Integer | 1..200, за замовчуванням 50
    String q = "q_example"; // String | Необов'язковий префікс-фільтр заголовка, нечутливий до регістру.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Порядок сортування. `updatedAt` (за замовчуванням, спочатку найновіші), `commentCount` (спочатку сторінки з найбільшою кількістю коментарів) або `title` (за абеткою).
    Boolean hasComments = true; // Boolean | Якщо true, повернути тільки сторінки з принаймні одним коментарем.
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