---
Cria uma nova conta de teste para um agente de IA sem um cadastro humano. Nenhuma chave de API é necessária para chamar isso.

A resposta contém o ID do locatário, uma chave de API que funciona imediatamente contra a API REST e o
servidor MCP, e uma URL de reivindicação. Forneça a URL de reivindicação ao humano para quem você está trabalhando: abri‑la enquanto
está conectado ao FastComments associa a conta a ele. Contas não reivindicadas, e suas chaves, são
excluídas 72 horas após a criação. Até serem reivindicadas, a conta tem os limites padrão de teste.

## Resposta

Retorna: [`CreateAgentTenantResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/CreateAgentTenantResponse.java)

## Exemplo

[inline-code-attrs-start title = 'createAgentTenant Exemplo'; type = 'java'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
// Import classes:
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.invoker.Configuration;
import com.fastcomments.invoker.models.*;
import com.fastcomments.api.DefaultApi;

public class Example {
  public static void main(String[] args) {
    ApiClient defaultClient = Configuration.getDefaultApiClient();
    defaultClient.setBasePath("https://fastcomments.com");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    CreateAgentTenantBody createAgentTenantBody = new CreateAgentTenantBody(); // CreateAgentTenantBody | 
    try {
      CreateAgentTenantResponse result = apiInstance.createAgentTenant(createAgentTenantBody)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exception when calling DefaultApi#createAgentTenant");
      System.err.println("Status code: " + e.getCode());
      System.err.println("Reason: " + e.getResponseBody());
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]

---