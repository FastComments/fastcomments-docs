Seznam strani za najemnika. Uporablja ga namizni odjemalec FChat za polnitev seznama svojih sob.
Zahteva, da je v rešeni prilagojeni konfiguraciji za vsako stran `enableFChat` nastavljen na true.
Strani, ki zahtevajo SSO, so filtrirane glede na dostop skupin zahtevajočega uporabnika.

## Parametri

| Ime | Tip | Lokacija | Obvezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Nečitljiv kurzor za straničenje, vrnjen kot `nextCursor` iz prejšnjega zahtevka. Povezan z istim `sortBy`. |
| limit | integer | query | No | 1..200, privzeto 50 |
| q | string | query | No | Neobvezen filter za predpono naslova, neobčutljiv na velike/male črke. |
| sortBy | string | query | No | Vrstni red razvrščanja. `updatedAt` (privzeto, najnovejše prve), `commentCount` (največ komentarjev prve), ali `title` (abecedno). |
| hasComments | boolean | query | No | Če je true, vrne samo strani z vsaj enim komentarjem. |

## Odgovor

Vrača: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Primer

[inline-code-attrs-start title = 'Primer getPagesPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvozi razrede:
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
    String cursor = "cursor_example"; // String | Nečitljiv kurzor za straničenje, vrnjen kot `nextCursor` iz prejšnjega zahtevka. Povezan z istim `sortBy`.
    Integer limit = 56; // Integer | 1..200, privzeto 50
    String q = "q_example"; // String | Neobvezen filter za predpono naslova, neobčutljiv na velike/male črke.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Vrstni red razvrščanja. `updatedAt` (privzeto, najnovejše prve), `commentCount` (največ komentarjev prve), ali `title` (abecedno).
    Boolean hasComments = true; // Boolean | Če je true, vrne le strani z vsaj enim komentarjem.
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