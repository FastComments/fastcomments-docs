## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|-------------|--------------|-------------|
| tenantId | string | query | Oui |  |
| commentId | string | path | Oui |  |
| direction | string | query | Non |  |
| broadcastId | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`VoteResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/VoteResponse.java)

## Exemple

[inline-code-attrs-start title = 'Exemple postVote'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer les classes :
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.ModerationApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    ModerationApi apiInstance = new ModerationApi(defaultClient);
    String tenantId = "tenantId_example"; // Chaîne |
    String commentId = "commentId_example"; // Chaîne |
    String direction = "direction_example"; // Chaîne |
    String broadcastId = "broadcastId_example"; // Chaîne |
    String sso = "sso_example"; // Chaîne |
    try {
      VoteResponse result = apiInstance.postVote(tenantId, commentId)
            .direction(direction)
            .broadcastId(broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#postVote");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---