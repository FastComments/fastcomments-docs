### Korištenje autentifikovanih API-ja (DefaultApi)

**Važno:** Morate postaviti vaš API ključ na ApiClient prije nego što napravite autentifikovane zahtjeve. Ako to ne uradite, zahtjevi će završiti sa 401 greškom.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Kreiraj i konfiguriraj API klijent
        ApiClient apiClient = new ApiClient();

        // OBAVEZNO: Postavite vaš API ključ (preuzmite ga sa FastComments kontrolne ploče)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Kreiraj instancu API-ja sa konfiguriranim klijentom
        DefaultApi api = new DefaultApi(apiClient);

        // Sada možete praviti autentifikovane API pozive
        try {
            // Primjer: Dodavanje SSO korisnika
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
            // - 401: API key nedostaje ili je nevažeći
            // - 400: Validacija zahtjeva nije uspjela
        }
    }
}
```

### Korištenje javnih API-ja (PublicApi)

Javni endpointi ne zahtijevaju autentifikaciju:

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

1. **401 "missing-api-key" greška**: Provjerite da li pozivate `apiClient.setApiKey("YOUR_KEY")` prije kreiranja instance DefaultApi.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za autentifikovane zahtjeve na serverskoj strani, a `PublicApi` za zahtjeve na klijentskoj strani/javne zahtjeve.
3. **Null API key**: SDK će tiho preskočiti autentifikaciju ako je API key null, što će dovesti do 401 grešaka.