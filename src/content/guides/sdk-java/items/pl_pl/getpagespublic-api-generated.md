Lista stron dla tenanta. Używana przez aplikację desktopową FChat do wypełniania jej listy pokoi.
Wymaga, aby `enableFChat` było ustawione na true w rozwiązywanej konfiguracji niestandardowej dla każdej strony.
Strony wymagające SSO są filtrowane względem dostępu grupowego użytkownika wysyłającego żądanie.

## Parametry

| Nazwa | Typ | Lokalizacja | Wymagane | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Niejawny kursor paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`. |
| limit | integer | query | No | 1..200, domyślnie 50 |
| q | string | query | No | Opcjonalny filtr prefiksu tytułu nieczuły na wielkość liter. |
| sortBy | string | query | No | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze jako pierwsze), `commentCount` (najwięcej komentarzy jako pierwsze), lub `title` (alfabetycznie). |
| hasComments | boolean | query | No | Jeśli true, zwróć tylko strony z co najmniej jednym komentarzem. |

## Odpowiedź

Zwraca: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Przykład

[inline-code-attrs-start title = 'Przykład getPagesPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importowanie klas:
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
    String cursor = "cursor_example"; // String | Niejawny kursor paginacji zwrócony jako `nextCursor` z poprzedniego żądania. Powiązany z tym samym `sortBy`.
    Integer limit = 56; // Integer | 1..200, domyślnie 50
    String q = "q_example"; // String | Opcjonalny, nieczuły na wielkość liter filtr prefiksu tytułu.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Kolejność sortowania. `updatedAt` (domyślnie, najnowsze jako pierwsze), `commentCount` (najwięcej komentarzy jako pierwsze) lub `title` (alfabetycznie).
    Boolean hasComments = true; // Boolean | Jeśli true, zwróć tylko strony z co najmniej jednym komentarzem.
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