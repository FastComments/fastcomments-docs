### Usando APIs Autenticadas (DefaultApi)

**Importante:** Você deve configurar sua chave de API no ApiClient antes de fazer requisições autenticadas. Se não o fizer, as requisições falharão com um erro 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Crie e configure o cliente da API
        ApiClient apiClient = new ApiClient();

        // OBRIGATÓRIO: Defina sua chave de API (obtenha-a no painel do FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Crie a instância da API com o cliente configurado
        DefaultApi api = new DefaultApi(apiClient);

        // Agora você pode fazer chamadas de API autenticadas
        try {
            // Exemplo: Adicionar um usuário SSO
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Erros comuns:
            // - 401: API key is missing or invalid
            // - 400: Request validation failed
        }
    }
}
```

### Usando APIs Públicas (PublicApi)

Endpoints públicos não requerem autenticação:

```java
import com.fastcomments.api.PublicApi;
import com.fastcomments.invoker.ApiException;

PublicApi publicApi = new PublicApi();

try {
    var response = publicApi.getCommentsPublic("YOUR_TENANT_ID", "page-url-id")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Problemas Comuns

1. **Erro 401 "missing-api-key"**: Certifique-se de chamar `apiClient.setApiKey("YOUR_KEY")` antes de criar a instância DefaultApi.
2. **Classe de API incorreta**: Use `DefaultApi` para requisições autenticadas no servidor, `PublicApi` para requisições no cliente/públicas.
3. **Chave de API nula**: O SDK ignorará silenciosamente a autenticação se a chave de API for nula, levando a erros 401.