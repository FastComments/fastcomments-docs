### Korišćenje autentifikovanih API-ja (DefaultApi)

**Važno:** Morate postaviti vaš API ključ na ApiClient pre nego što izvršite autentifikovane zahteve. Ako to ne uradite, zahtevi će biti odbijeni sa 401 greškom.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Kreirajte i konfigurišite ApiClient
        ApiClient apiClient = new ApiClient();

        // OBAVEZNO: Postavite vaš API ključ (preuzmite ga sa FastComments kontrolne table)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Kreirajte instancu API-a sa konfigurisanim klijentom
        DefaultApi api = new DefaultApi(apiClient);

        // Sada možete praviti autentifikovane API pozive
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
            // - 401: API ključ nedostaje ili nije validan
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

### Korišćenje moderacijskih API-ja (ModerationApi)

`ModerationApi` pokreće moderator dashboard. Svaka metoda prihvata `sso` parametar koji identifikuje SSO-autentifikovanog moderatora u čije ime se zahtev pravi:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Listajte komentare koji čekaju na moderaciju
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Uobičajeni problemi

1. **401 "missing-api-key" error**: Uverite se da pozovete `apiClient.setApiKey("YOUR_KEY")` pre nego što kreirate DefaultApi instancu.
2. **Pogrešna API klasa**: Koristite `DefaultApi` za serverske autentifikovane zahteve, `PublicApi` za klijentske/javne zahteve.
3. **Null API ključ**: SDK će tiho preskočiti autentifikaciju ako je API ključ null, što će dovesti do 401 grešaka.