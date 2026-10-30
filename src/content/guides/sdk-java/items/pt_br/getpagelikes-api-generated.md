Like count for a page, optionally with the users who liked it.

## Parameters

| Name | Type | Location | Required | Description |
|------|------|----------|----------|-------------|
| tenantId | string | query | Yes |  |
| urlId | string | query | Yes |  |
| includeUserInfo | string | query | No | "\"public\" omite usuários com perfis privados, \"include-private\" os inclui." |

## Response

Returns: [`GetPageLikesAPIResponse`](https://github.com/FastComments/fastcomments-java/blob/main/client/src/main/java/com/fastcomments/model/GetPageLikesAPIResponse.java)

## Example

[inline-code-attrs-start title = 'Exemplo getPageLikes'; type = 'java'; isFunctional = false; inline-code-attrs-end]
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
    
    // Configurar autorização da chave API: api_key
    ApiKeyAuth api_key = (ApiKeyAuth) defaultClient.getAuthentication("api_key");
    api_key.setApiKey("YOUR API KEY");
    // Descomente a linha a seguir para definir um prefixo para a chave API, por exemplo "Token" (padrão null)
    //api_key.setApiKeyPrefix("Token");

    DefaultApi apiInstance = new DefaultApi(defaultClient);
    String tenantId = "tenantId_example"; // String | 
    String urlId = "urlId_example"; // String | 
    PageReactIncludeUserInfo includeUserInfo = PageReactIncludeUserInfo.fromValue("public"); // PageReactIncludeUserInfo | \"public\" omite usuários com perfis privados, \"include-private\" os inclui.
    try {
      GetPageLikesAPIResponse result = apiInstance.getPageLikes(tenantId, urlId)
            .includeUserInfo(includeUserInfo)
            .execute();
      System.out.println(result);
    } catch (ApiException e) {
      // Exceção ao chamar DefaultApi#getPageLikes
      System.err.println("Exception when calling DefaultApi#getPageLikes");
      // Código de status: " + e.getCode()
      System.err.println("Status code: " + e.getCode());
      // Motivo: " + e.getResponseBody()
      System.err.println("Reason: " + e.getResponseBody());
      // Cabeçalhos da resposta: " + e.getResponseHeaders()
      System.err.println("Response headers: " + e.getResponseHeaders());
      e.printStackTrace();
    }
  }
}
[inline-code-end]