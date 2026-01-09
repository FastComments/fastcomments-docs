### Verwendung authentifizierter APIs (DefaultApi)

**Wichtig:** Sie müssen Ihren API-Schlüssel im ApiClient setzen, bevor Sie authentifizierte Anfragen durchführen. Wenn Sie dies nicht tun, schlagen die Anfragen mit einem 401-Fehler fehl.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Erstelle und konfiguriere den API-Client
        ApiClient apiClient = new ApiClient();

        // ERFORDERLICH: Setzen Sie Ihren API-Schlüssel (erhalten Sie ihn von Ihrem FastComments-Dashboard)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Erstelle die API-Instanz mit dem konfigurierten Client
        DefaultApi api = new DefaultApi(apiClient);

        // Jetzt können Sie authentifizierte API-Aufrufe durchführen
        try {
            // Beispiel: Füge einen SSO-Benutzer hinzu
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Häufige Fehler:
            // - 401: API-Schlüssel fehlt oder ist ungültig
            // - 400: Anfragevalidierung fehlgeschlagen
        }
    }
}
```

### Verwendung öffentlicher APIs (PublicApi)

Öffentliche Endpunkte erfordern keine Authentifizierung:

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

### Häufige Probleme

1. **401 "missing-api-key" error**: Stellen Sie sicher, dass Sie `apiClient.setApiKey("YOUR_KEY")` aufrufen, bevor Sie die DefaultApi-Instanz erstellen.
2. **Falsche API-Klasse**: Verwenden Sie `DefaultApi` für serverseitige authentifizierte Anfragen, `PublicApi` für clientseitige/öffentliche Anfragen.
3. **API-Schlüssel ist null**: Das SDK überspringt stillschweigend die Authentifizierung, wenn der API-Schlüssel null ist, was zu 401-Fehlern führt.