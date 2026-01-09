### Korišćenje autentifikovanih API-ja (DefaultApi)

**Važno:** Morate postaviti svoj API ključ na ApiClient pre nego što napravite autentifikovane zahteve. Ako to ne uradite, zahtevi će završiti sa 401 greškom.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Kreirajte i konfigurišite ApiClient
        ApiClient apiClient = new ApiClient();

        // OBAVEZNO: Postavite svoj API ključ (preuzmite ga sa FastComments kontrolne table)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Kreirajte instancu DefaultApi sa konfigurisanim ApiClient
        DefaultApi api = new DefaultApi(apiClient);

        // Sada možete napraviti autentifikovane pozive API-ju
        try {
            // Primer: Dodavanje SSO korisnika
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Uobičajene greške:
            // - 401: Nedostaje ili je neispravan API ključ
            // - 400: Validacija zahteva nije uspela
        }
    }
}
```

### Korišćenje javnih API-ja (PublicApi)

Javni endpointi ne zahtevaju autentifikaciju:

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

### Uobičajeni problemi

1. **401 "missing-api-key" error**: Uverite se da pozovete `apiClient.setApiKey("YOUR_KEY")` pre kreiranja instance DefaultApi.
2. **Wrong API class**: Koristite `DefaultApi` za serverske autentifikovane zahteve, `PublicApi` za klijentske/javne zahteve.
3. **Null API key**: SDK će ćutke preskočiti autentifikaciju ako je API ključ null, što dovodi do 401 grešaka.