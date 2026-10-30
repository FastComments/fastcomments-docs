Chi ha votato per quale opzione. I sondaggi anonimi non rispondono mai, i sondaggi solo per amministratori rispondono solo per un amministratore o moderatore.

## Parameters

| Nome | Tipo | Posizione | Obbligatorio | Descrizione |
|------|------|-----------|--------------|-------------|
| tenantId | string | path | Yes |  |
| commentId | string | path | Yes |  |
| urlId | string | query | No |  |
| page | integer | query | No |  |
| search | string | query | No |  |
| sso | string | query | No |  |

## Response

Restituisce: [`GetPollVoterNamesSuccessResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPollVoterNamesSuccessResponse.java)

## Example

[inline-code-attrs-start title = 'Esempio getPollVoters'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importa classi:
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
    String tenantId = "tenantId_example"; // Stringa | 
    String commentId = "commentId_example"; // Stringa | 
    String urlId = "urlId_example"; // Stringa | 
    Integer page = 56; // Intero | 
    String search = "search_example"; // Stringa | 
    String sso = "sso_example"; // Stringa | 
    try {
      GetPollVoterNamesSuccessResponse result = apiInstance.getPollVoters(tenantId, commentId)
            .urlId(urlId)
            .page(page)
            .search(search)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling PublicApi#getPollVoters");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]