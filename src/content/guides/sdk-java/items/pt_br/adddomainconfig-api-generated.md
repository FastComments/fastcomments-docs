---
## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|------------|------------|-----------|
| tenantId | string | query | Sim |  |

## Resposta

Retorna: [`AddDomainConfig200Response`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/AddDomainConfig200Response.java)

## Exemplo

[inline-code-attrs-start title = 'Exemplo de addDomainConfig'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    // Descomente a linha abaixo para definir um prefixo para a chave da API, por exemplo "Token" (padrão: null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    AddDomainConfigParams addDomainConfigParams = new AddDomainConfigParams(); // AddDomainConfigParams | 
    try {
      AddDomainConfig200Response result = apiInstance.addDomainConfig(tenantId, addDomainConfigParams)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#addDomainConfig");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---