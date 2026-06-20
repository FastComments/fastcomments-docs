### Utilizzo delle API Autenticate (DefaultApi)

**Importante:** Devi impostare la tua API key su ApiClient prima di effettuare richieste autenticate. Se non lo fai, le richieste falliranno con un errore 401.

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

        // Crea l'istanza API con il client configurato
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
            // - 401: Mancante o invalida API key
            // - 400: La validazione della richiesta è fallita
        }
    }
}
```

### Utilizzo delle API Pubbliche (PublicApi)

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

### Utilizzo delle API di Moderazione (ModerationApi)

La `ModerationApi` gestisce la dashboard dei moderatori. Ogni metodo accetta un parametro `sso` che identifica il moderatore autenticato via SSO per conto del quale viene effettuata la richiesta:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Elenca i commenti in attesa di moderazione
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Problemi comuni

1. **401 "missing-api-key" error**: Assicurati di chiamare `apiClient.setApiKey("YOUR_KEY")` prima di creare l'istanza DefaultApi.
2. **Classe API errata**: Usa `DefaultApi` per richieste autenticate lato server, `PublicApi` per richieste client-side/pubbliche.
3. **Null API key**: L'SDK salterà silenziosamente l'autenticazione se la API key è null, causando errori 401.