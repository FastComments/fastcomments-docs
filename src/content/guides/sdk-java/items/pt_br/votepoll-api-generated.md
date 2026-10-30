## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|--------------|--------------|-----------|
| tenantId | string | path | Sim |  |
| commentId | string | path | Sim |  |
| urlId | string | query | Não |  |
| broadcastId | string | query | Não |  |
| sso | string | query | Não |  |

## Resposta

Retorna: [`VotePollResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/VotePollResponse.java)

## Exemplo

[inline-code-attrs-start title = 'Exemplo votePoll'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importar classes:
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
    String commentId = "commentId_example"; // String |
    VotePollBody votePollBody = new VotePollBody(); // VotePollBody |
    String urlId = "urlId_example"; // String |
    String broadcastId = "broadcastId_example"; // String |
    String sso = "sso_example"; // String |
    try {
      VotePollResponse result = apiInstance.votePoll(tenantId, commentId, votePollBody)
            .urlId(urlId)
            .broadcastId(broadcastId)
            .sso(sso)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exceção ao chamar PublicApi#votePoll");
      System.err.println("Código de status: " + e.getCode());
      System.err.println("Motivo: " + e.getResponseBody());
      System.err.println("Cabeçalhos da resposta: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]