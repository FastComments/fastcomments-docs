### Usando APIs Autenticadas (DefaultApi)

**Importante:** Você deve definir sua chave de API no ApiClient antes de fazer requisições autenticadas. Se não fizer isso, as requisições falharão com um erro 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Create and configure the API client
        ApiClient apiClient = new ApiClient();

        // REQUIRED: Set your API key (get this from your FastComments dashboard)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Create the API instance with the configured client
        DefaultApi api = new DefaultApi(apiClient);

        // Now you can make authenticated API calls
        try {
            // Example: Add an SSO user
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Common errors:
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

### Usando APIs de Moderação (ModerationApi)

A `ModerationApi` alimenta o painel do moderador. Cada método aceita um parâmetro `sso` identificando o moderador autenticado via SSO em cujo nome a requisição é feita:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // List comments awaiting moderation
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Problemas Comuns

1. **401 "missing-api-key" error**: Certifique-se de chamar `apiClient.setApiKey("YOUR_KEY")` antes de criar a instância do DefaultApi.
2. **Wrong API class**: Use `DefaultApi` para requisições autenticadas no lado do servidor, `PublicApi` para requisições do lado do cliente/públicas.
3. **Null API key**: O SDK ignorará a autenticação silenciosamente se a chave de API for nula, resultando em erros 401.