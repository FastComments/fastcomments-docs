Prikazuje stranice za tenant. Koristi ga FChat desktop klijent za popunjavanje popisa soba.
Zahtijeva da `enableFChat` bude `true` u razriješenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtijevaju SSO filtriraju se prema grupnom pristupu korisnika koji šalje zahtjev.

## Parametri

| Naziv | Tip | Lokacija | Obavezno | Opis |
|------|------|----------|----------|-------------|
| tenantId | string | putanja | Da |  |
| cursor | string | upit | Ne | Neprozirni pokazivač paginacije koji se vraća kao `nextCursor` iz prethodnog zahtjeva. Povezan je s istim `sortBy`. |
| limit | integer | upit | Ne | 1..200, zadano 50 |
| q | string | upit | Ne | Opcionalni prefiks naslova, neosjetljiv na velika/mala slova. |
| sortBy | string | upit | Ne | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). |
| hasComments | boolean | upit | Ne | Ako je true, vraćaju se samo stranice s najmanje jednim komentarom. |

## Odgovor

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Primjer

[inline-code-attrs-start title = 'Primjer getPagesPublic'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String cursor = "cursor_example"; // String | Neprozirni pokazivač paginacije koji se vraća kao `nextCursor` iz prethodnog zahtjeva. Povezan je s istim `sortBy`.
    Integer limit = 56; // Integer | 1..200, zadano 50
    String q = "q_example"; // String | Opcionalni prefiks naslova, neosjetljiv na velika/mala slova.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno).
    Boolean hasComments = true; // Boolean | Ako je true, vraćaju se samo stranice s najmanje jednim komentarom.
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