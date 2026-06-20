Lister sider for en tenant. Bruges af FChat desktop-klienten til at udfylde dens rumsliste.
Kræver, at `enableFChat` er true på den resolved custom config for hver side.
Sider, der kræver SSO, filtreres i forhold til den anmodende brugers gruppeadgang.

## Parametre

| Navn | Type | Placering | Krævet | Beskrivelse |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opak pagineringscursor returneret som `nextCursor` fra en tidligere anmodning. Knyttet til samme `sortBy`. |
| limit | integer | query | No | 1..200, standard 50 |
| q | string | query | No | Valgfrit præfiksfilter for titel, der ikke skelner mellem store og små bogstaver. |
| sortBy | string | query | No | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (flest kommentarer først), eller `title` (alfabetisk). |
| hasComments | boolean | query | No | Hvis true, returner kun sider med mindst én kommentar. |

## Svar

Returnerer: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Eksempel

[inline-code-attrs-start title = 'getPagesPublic Eksempel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
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
    String cursor = "cursor_example"; // String | Opak pagineringscursor returneret som `nextCursor` fra en tidligere forespørgsel. Knyttet til samme `sortBy`.
    Integer limit = 56; // Integer | 1..200, standard 50
    String q = "q_example"; // String | Valgfrit præfiksfilter for titel, der ikke skelner mellem store og små bogstaver.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Sorteringsrækkefølge. `updatedAt` (standard, nyeste først), `commentCount` (flest kommentarer først), eller `title` (alfabetisk).
    Boolean hasComments = true; // Boolean | Hvis true, returner kun sider med mindst én kommentar.
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

---