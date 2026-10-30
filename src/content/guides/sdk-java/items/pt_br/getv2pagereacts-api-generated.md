## Parâmetros

| Nome | Tipo | Localização | Obrigatório | Descrição |
|------|------|-------------|-------------|-----------|
| tenantId | string | path | Yes |  |
| urlId | string | query | Yes |  |
| sso | string | query | No |  |
| includeUserInfo | string | query | No | Quando definido, retorna os usuários para cada id de reação (avatares e nomes de usuário). Apenas "public" é aceito, o que omite perfis privados. |

## Resposta

Retorna: [`GetV2PageReacts`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetV2PageReacts.java)

## Exemplo

[inline-code-attrs-start title = 'Exemplo getV2PageReacts'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    String urlId = "urlId_example"; // String | 
    String sso = "sso_example"; // String | 
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | Quando definido, retorna os usuários para cada id de reação (avatares e nomes de usuário). Apenas "public" é aceito, o que omite perfis privados.
    try {
      GetV2PageReacts result = apiInstance.getV2PageReacts(tenantId, urlId)
            .sso(sho)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      System.err.println("Exceção ao chamar PublicApi#getV2PageReacts");
      System.err.println("Código de status: " + e.getCode());
      System.err.println("Motivo: " + e.getResponseBody());
      System.err.println("Cabeçalhos da resposta: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]