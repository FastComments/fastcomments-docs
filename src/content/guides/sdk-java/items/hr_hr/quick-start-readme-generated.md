### Korištenje autentificiranih API-ja (DefaultApi)

**Važno:** Morate postaviti svoj API ključ na ApiClient prije slanja autentificiranih zahtjeva. Ako to ne učinite, zahtjevi će vratiti grešku 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Kreirajte i konfigurirajte API klijent
        ApiClient apiClient = new ApiClient();

        // OBAVEZNO: Postavite svoj API ključ (dohvatite ga s FastComments nadzorne ploče)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Kreirajte instancu API-ja s konfiguriranim klijentom
        DefaultApi api = new DefaultApi(apiClient);

        // Sada možete izvoditi autentificirane API pozive
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
            // Uobičajene pogreške:
            // - 401: API ključ nedostaje ili je neispravan
            // - 400: Provjera valjanosti zahtjeva nije uspjela
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

1. **401 "missing-api-key" error**: Pobrinite se da pozovete `apiClient.setApiKey("YOUR_KEY")` prije kreiranja instance DefaultApi.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za poslužiteljske autentificirane zahtjeve, `PublicApi` za klijentske/javne zahtjeve.
3. **Null API ključ**: SDK će tiho preskočiti autentifikaciju ako je API ključ null, što dovodi do 401 pogrešaka.