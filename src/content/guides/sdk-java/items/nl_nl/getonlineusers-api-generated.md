Momenteel online kijkers van een pagina: mensen wiens websocket-sessie op dit moment op die pagina geabonneerd is.
Geeft anonCount + totalCount terug (abonnees in de hele ruimte, inclusief anonieme kijkers die we niet afzonderlijk opsommen).

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Pagina-URL-identificator (schoongemaakt aan de serverzijde). |
| afterName | string | query | Nee | Cursor: geef nextAfterName uit de vorige respons door. |
| afterUserId | string | query | Nee | Cursor tiebreaker: geef nextAfterUserId uit de vorige respons door. Verplicht wanneer afterName is ingesteld zodat bij gelijke namen geen items verloren gaan. |

## Respons

Geeft terug: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Voorbeeld

[inline-code-attrs-start title = 'getOnlineUsers Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importeer klassen:
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
    String urlId = "urlId_example"; // String | Pagina-URL-identificator (schoongemaakt aan de serverzijde).
    String afterName = "afterName_example"; // String | Cursor: geef nextAfterName uit de vorige respons door.
    String afterUserId = "afterUserId_example"; // String | Cursor tiebreaker: geef nextAfterUserId uit de vorige respons door. Verplicht wanneer afterName is ingesteld zodat bij gelijke namen geen items verloren gaan.
    try {
      PageUsersOnlineResponse result = apiInstance.getOnlineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOnlineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---