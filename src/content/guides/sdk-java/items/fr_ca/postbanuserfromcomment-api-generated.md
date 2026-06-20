## Paramètres

| Nom | Type | Emplacement | Obligatoire | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Oui |  |
| banEmail | boolean | query | Non |  |
| banEmailDomain | boolean | query | Non |  |
| banIP | boolean | query | Non |  |
| deleteAllUsersComments | boolean | query | Non |  |
| bannedUntil | string | query | Non |  |
| isShadowBan | boolean | query | Non |  |
| updateId | string | query | Non |  |
| banReason | string | query | Non |  |
| sso | string | query | Non |  |

## Réponse

Renvoie : [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/BanUserFromCommentResult.java)

## Exemple

[inline-code-attrs-start title = 'Exemple de postBanUserFromComment'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importer des classes:
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
    String commentId = "commentId_example"; // String | 
    Boolean banEmail = true; // Boolean | 
    Boolean banEmailDomain = true; // Boolean | 
    Boolean banIP = true; // Boolean | 
    Boolean deleteAllUsersComments = true; // Boolean | 
    String bannedUntil = "bannedUntil_example"; // String | 
    Boolean isShadowBan = true; // Boolean | 
    String updateId = "updateId_example"; // String | 
    String banReason = "banReason_example"; // String | 
    String sso = "sso_example"; // String | 
    try {
      BanUserFromCommentResult result = apiInstance.postBanUserFromComment(commentId)
            .banEmail(banEmail)
            .banEmailDomain(banEmailDomain)
            .banIP(banIP)
            .deleteAllUsersComments(deleteAllUsersComments)
            .bannedUntil(bannedUntil)
            .isShadowBan(isShadowBan)
            .updateId(updateId)
            .banReason(banReason)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#postBanUserFromComment");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---