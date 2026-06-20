Prejšnji komentatorji na strani, ki trenutno NISO na spletu. Razvrščeno po displayName.
Uporabite to po izčrpanju /users/online za upodobitev razdelka "Members".
Pomnilniška paginacija (cursor pagination) na commenterName: strežnik hodi po delnem indeksu {tenantId, urlId, commenterName}
od afterName naprej preko $gt, brez stroška $skip.

## Parametri

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | path | Da |  |
| urlId | string | query | Da | Page URL identifier (cleaned server-side). |
| afterName | string | query | Ne | Kazalec: posredujte nextAfterName iz prejšnjega odgovora. |
| afterUserId | string | query | Ne | Kazalec za prelom: posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, ko je afterName nastavljena, da vezi pri enakih imenih ne izpustijo vnosov. |

## Odgovor

Vrne: [`PageUsersOfflineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOfflineResponse.java)

## Primer

[inline-code-attrs-start title = 'getOfflineUsers Primer'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | Identifikator URL strani (očiščen na strežniški strani).
    String afterName = "afterName_example"; // String | Kazalec: posredujte nextAfterName iz prejšnjega odgovora.
    String afterUserId = "afterUserId_example"; // String | Kazalec za prelom: posredujte nextAfterUserId iz prejšnjega odgovora. Zahtevano, ko je afterName nastavljena, da vezi pri enakih imenih ne izpustijo vnosov.
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