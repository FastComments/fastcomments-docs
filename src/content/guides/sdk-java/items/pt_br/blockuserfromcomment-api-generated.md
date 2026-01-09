## Parâmetros

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Sim |  |
| id | string | path | Sim |  |
| userId | string | query | Não |  |
| anonUserId | string | query | Não |  |

## Resposta

Retorna: [`BlockFromCommentPublic200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/BlockFromCommentPublic200Response.java)

## Exemplo

[inline-code-attrs-start title = 'Exemplo blockUserFromComment'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Importar classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.auth.*;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");
    
    // Configurar autorização por chave de API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Descomente a linha abaixo para definir um prefixo para a chave de API, ex.: "Token" (padrão: null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String id = "id_example"; // String | 
    BlockFromCommentParams blockFromCommentParams = new BlockFromCommentParams(); // BlockFromCommentParams | 
    String userId = "userId_example"; // String | 
    String anonUserId = "anonUserId_example"; // String | 
    try {
      BlockFromCommentPublic200Response result = apiInstance.blockUserFromComment(tenantId, id, blockFromCommentParams)
            .userId(userId)
            .anonUserId(anonUserId)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#blockUserFromComment");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]