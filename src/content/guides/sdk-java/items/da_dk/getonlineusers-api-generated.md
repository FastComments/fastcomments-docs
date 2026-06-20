Aktuelt online-seere af en side: personer hvis websocket-session er abonneret på siden lige nu.
Returnerer anonCount + totalCount (abonnenter på tværs af rummet, inklusive anonyme seere, som vi ikke optæller).

## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes | Page URL identifier (renses server-side). |
| afterName | string | query | No | Cursor: angiv nextAfterName fra det forrige svar. |
| afterUserId | string | query | No | Cursor tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så ligheder i navne ikke medfører tab af poster. |

## Svar

Returnerer: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Eksempel

[inline-code-attrs-start title = 'getOnlineUsers Eksempel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer klasser:
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
    String urlId = "urlId_example"; // String | Page URL identifier (renses server-side).
    String afterName = "afterName_example"; // String | Cursor: angiv nextAfterName fra det forrige svar.
    String afterUserId = "afterUserId_example"; // String | Cursor tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er angivet, så ligheder i navne ikke medfører tab af poster.
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