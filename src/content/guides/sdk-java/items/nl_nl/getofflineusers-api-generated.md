Vorige commentatoren op de pagina die NIET momenteel online zijn. Gesorteerd op displayName.
Gebruik dit nadat u /users/online hebt uitgeput om een "Leden" sectie weer te geven.
Cursor-paginering op commenterName: de server loopt de gedeeltelijke index {tenantId, urlId, commenterName} vanaf afterName vooruit via $gt, zonder $skip-kosten.

## Parameters

| Naam | Type | Locatie | Verplicht | Beschrijving |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Pagina-URL-identificator (schoongemaakt aan de serverzijde). |
| afterName | string | query | No | Cursor: geef nextAfterName door van het vorige antwoord. |
| afterUserId | string | query | No | Cursor tiebreaker: geef nextAfterUserId door van het vorige antwoord. Vereist wanneer afterName is ingesteld zodat naamgelijkheden geen vermeldingen wegvallen. |

## Antwoord

Retourneert: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Voorbeeld

[inline-code-attrs-start title = 'getOfflineUsers Voorbeeld'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String afterName = "afterName_example"; // String | Cursor: geef nextAfterName door van het vorige antwoord.
    String afterUserId = "afterUserId_example"; // String | Cursor tiebreaker: geef nextAfterUserId door van het vorige antwoord. Vereist wanneer afterName is ingesteld zodat naamgelijkheden geen vermeldingen wegvallen.
    try {
      PageUsersOfflineResponse result = apiInstance.getOfflineUsers(tenantId, urlId)
            .afterName(afterName)
            .afterUserId(afterUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getOfflineUsers");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]