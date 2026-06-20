Navodi stranice za tenant. Koristi se od strane FChat desktop klijenta za popunjavanje svoje liste soba.
Zahteva da `enableFChat` bude true na razrešenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtevaju SSO se filtriraju u odnosu na pristupne grupe korisnika koji šalje zahtev.

## Parametri

| Name | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Neprozirni kursor za paginaciju koji se vraća kao `nextCursor` iz prethodnog zahteva. Veže se za isti `sortBy`. |
| limit | integer | query | No | 1..200, podrazumevano 50 |
| q | string | query | No | Opcionalan filter prefiksa naslova, neosetljiv na velika/mala slova. |
| sortBy | string | query | No | Redosled sortiranja. `updatedAt` (podrazumevano, najnoviji prvi), `commentCount` (prvo stranice sa najviše komentara), ili `title` (alfabetski). |
| hasComments | boolean | query | No | Ako je true, vraća samo stranice sa najmanje jednim komentarom. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Primer

[inline-code-attrs-start title = 'getPagesPublic Primer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Uvezi klase:
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
    String cursor = "cursor_example"; // String | Neprozirni kursor za paginaciju koji se vraća kao `nextCursor` iz prethodnog zahteva. Veže se za isti `sortBy`.
    Integer limit = 56; // Integer | 1..200, podrazumevano 50
    String q = "q_example"; // String | Opcionalan filter prefiksa naslova, neosetljiv na velika/mala slova.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Redosled sortiranja. `updatedAt` (podrazumevano, najnoviji prvi), `commentCount` (prvo stranice sa najviše komentara), ili `title` (alfabetski).
    Boolean hasComments = true; // Boolean | Ako je true, vraća samo stranice sa najmanje jednim komentarom.
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