## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| commentId | string | path | Sim |  |
| includeEmail | boolean | query | Não |  |
| includeIP | boolean | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`ModerationAPICommentResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/ModerationAPICommentResponse.java)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getModerationComment'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importar classes:
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
    Boolean includeEmail = true; // Boolean | 
    Boolean includeIP = true; // Boolean | 
    String sso = "sso_example"; // String | 
    try {
      ModerationAPICommentResponse result = apiInstance.getModerationComment(commentId)
            .includeEmail(includeEmail)
            .includeIP(includeIP)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling ModerationApi#getModerationComment");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---