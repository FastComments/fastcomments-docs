---
Visualizadores atualmente online de uma página: pessoas cuja sessão websocket está inscrita na página agora.
Retorna anonCount + totalCount (assinantes da sala, incluindo visualizadores anônimos que não enumeramos).

## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|------------|-----------|
| tenantId | string | path | Sim |  |
| urlId | string | query | Sim | Identificador da URL da página (limpo no servidor). |
| afterName | string | query | Não | Cursor: passe nextAfterName da resposta anterior. |
| afterUserId | string | query | Não | Desempate do cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName estiver definido para que empates de nome não descartem entradas. |

## Resposta

Retorna: [`PageUsersOnlineResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/PageUsersOnlineResponse.java)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de getOnlineUsers'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | Identificador da URL da página (limpo no servidor).
    String afterName = "afterName_example"; // String | Cursor: passe nextAfterName da resposta anterior.
    String afterUserId = "afterUserId_example"; // String | Desempate do cursor: passe nextAfterUserId da resposta anterior. Obrigatório quando afterName estiver definido para que empates de nome não descartem entradas.
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

---