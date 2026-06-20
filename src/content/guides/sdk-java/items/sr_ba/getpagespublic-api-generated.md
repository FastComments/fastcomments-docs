Lista stranica za tenant. Koristi se od FChat desktop klijenta za popunjavanje njegove liste soba.
Zahtijeva da `enableFChat` bude true u razriješenoj prilagođenoj konfiguraciji za svaku stranicu.
Stranice koje zahtijevaju SSO filtriraju se prema grupnom pristupu korisnika koji šalje zahtjev.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| cursor | string | query | No | Opaque paginacijski kursor vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan sa istim `sortBy`. |
| limit | integer | query | No | 1..200, zadano 50 |
| q | string | query | No | Opcionalni filter prefiksa naslova koji nije osjetljiv na velika/mala slova. |
| sortBy | string | query | No | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno). |
| hasComments | boolean | query | No | Ako je true, vrati samo stranice sa najmanje jednim komentarom. |

## Response

Vraća: [`GetPublicPagesResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPublicPagesResponse.java)

## Example

[inline-code-attrs-start title = 'getPagesPublic Primjer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String cursor = "cursor_example"; // String | Opaque paginacijski kursor vraćen kao `nextCursor` iz prethodnog zahtjeva. Povezan sa istim `sortBy`.
    Integer limit = 56; // Integer | 1..200, zadano 50
    String q = "q_example"; // String | Opcionalni filter prefiksa naslova koji nije osjetljiv na velika/mala slova.
    PagesSortBy sortBy = PagesSortBy.fromValue("updatedAt"); // PagesSortBy | Redoslijed sortiranja. `updatedAt` (zadano, najnovije prvo), `commentCount` (najviše komentara prvo), ili `title` (abecedno).
    Boolean hasComments = true; // Boolean | Ako je true, vrati samo stranice sa najmanje jednim komentarom.
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