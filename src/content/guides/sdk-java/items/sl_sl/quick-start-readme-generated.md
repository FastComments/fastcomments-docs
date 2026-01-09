### Uporaba avtenticiranih API-jev (DefaultApi)

**Pomembno:** Pred izvajanjem avtenticiranih zahtev morate nastaviti svoj API ključ na ApiClient. Če tega ne storite, bodo zahtevki neuspešni s 401 napako.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Ustvarite in konfigurirajte API odjemalca
        ApiClient apiClient = new ApiClient();

        // OBVEZNO: Nastavite svoj API ključ (pridobite ga na nadzorni plošči FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Ustvarite instanco API-ja s konfiguriranim odjemalcem
        DefaultApi api = new DefaultApi(apiClient);

        // Zdaj lahko izvajate avtenticirane klice API-ja
        try {
            // Primer: Dodajanje SSO uporabnika
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Pogoste napake:
            // - 401: API ključ manjka ali je neveljaven
            // - 400: Preverjanje zahtevka ni uspelo
        }
    }
}
```

### Uporaba javnih API-jev (PublicApi)

Javne končne točke ne zahtevajo overjanja:

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

### Pogoste težave

1. **401 "missing-api-key" error**: Prepričajte se, da pokličete `apiClient.setApiKey("YOUR_KEY")` pred ustvarjanjem instance DefaultApi.
2. **Napačen razred API-ja**: Uporabite `DefaultApi` za strežniške avtenticirane zahteve, `PublicApi` za odjemalske/javne zahteve.
3. **Null API key**: SDK bo tiho preskočil avtentikacijo, če je API ključ null, kar bo povzročilo 401 napake.