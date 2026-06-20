---
## Parameter

| Name | Typ | Ort | Erforderlich | Beschreibung |
|------|------|----------|----------|-------------|
| commentId | string | path | Ja |  |
| banEmail | boolean | query | Nein |  |
| banEmailDomain | boolean | query | Nein |  |
| banIP | boolean | query | Nein |  |
| deleteAllUsersComments | boolean | query | Nein |  |
| bannedUntil | string | query | Nein |  |
| isShadowBan | boolean | query | Nein |  |
| updateId | string | query | Nein |  |
| banReason | string | query | Nein |  |
| sso | string | query | Nein |  |

## Antwort

Gibt zurück: [`BanUserFromCommentResult`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/BanUserFromCommentResult.java)

## Beispiel

[inline-code-attrs-start title = 'Beispiel für postBanUserFromComment'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Klassen importieren:
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