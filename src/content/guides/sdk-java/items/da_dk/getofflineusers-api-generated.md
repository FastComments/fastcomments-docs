Tidligere kommentatorer på siden, som IKKE er online lige nu. Sorteret efter displayName.
Brug dette efter at have udtømt /users/online for at vise en "Medlemmer"-sektion.
Cursor-paginering på commenterName: serveren går igennem den delvise {tenantId, urlId, commenterName}
indeks fra afterName fremad via $gt, uden $skip-omkostning.

## Parametre

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Ja |  |
| urlId | string | query | Ja | Side-URL-identifikator (renset på serversiden). |
| afterName | string | query | Nej | Cursor: angiv nextAfterName fra det forrige svar. |
| afterUserId | string | query | Nej | Cursor tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er sat, så navnekonflikter ikke medfører tab af poster. |

## Svar

Returnerer: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Eksempel

[inline-code-attrs-start title = 'getOfflineUsers Eksempel'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | Side-URL-identifikator (renset på serversiden).
    String afterName = "afterName_example"; // String | Cursor: angiv nextAfterName fra det forrige svar.
    String afterUserId = "afterUserId_example"; // String | Cursor tiebreaker: angiv nextAfterUserId fra det forrige svar. Påkrævet når afterName er sat, så navnekonflikter ikke medfører tab af poster.
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