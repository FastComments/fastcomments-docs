### Uso delle API Autenticate (DefaultApi)

**Importante:** Devi impostare la tua API key su ApiClient prima di effettuare richieste autenticate. Se non lo fai, le richieste restituiranno un errore 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Crea e configura il client API
        ApiClient apiClient = new ApiClient();

        // OBBLIGATORIO: Imposta la tua API key (prendila dalla dashboard di FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Crea l'istanza dell'API con il client configurato
        DefaultApi api = new DefaultApi(apiClient);

        // Ora puoi effettuare chiamate API autenticate
        try {
            // Esempio: Aggiungi un utente SSO
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Errori comuni:
            // - 401: la API key manca o non è valida
            // - 400: la validazione della richiesta è fallita
        }
    }
}
```

### Uso delle API Pubbliche (PublicApi)

Gli endpoint pubblici non richiedono autenticazione:

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

### Problemi Comuni

1. **401 "missing-api-key" error**: Assicurati di chiamare `apiClient.setApiKey("YOUR_KEY")` prima di creare l'istanza DefaultApi.
2. **Wrong API class**: Usa `DefaultApi` per richieste autenticate lato server, `PublicApi` per richieste lato client/pubbliche.
3. **Null API key**: L'SDK salterà silenziosamente l'autenticazione se l'API key è null, portando a errori 401.