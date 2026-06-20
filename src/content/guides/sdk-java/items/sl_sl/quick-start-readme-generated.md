### Uporaba avtenticiranih API-jev (DefaultApi)

**Pomembno:** Pred izvajanjem avtenticiranih zahtev morate nastaviti svoj API ključ na ApiClientu. Če tega ne storite, bodo zahteve neuspešne z napako 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Ustvarite in konfigurirajte API odjemalca
        ApiClient apiClient = new ApiClient();

        // OBVEZNO: Nastavite svoj API ključ (pridobite ga iz FastComments nadzorne plošče)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Ustvarite instanco API z konfiguriranim odjemalcem
        DefaultApi api = new DefaultApi(apiClient);

        // Zdaj lahko izvajate avtenticirane klice API-ja
        try {
            // Primer: dodajanje SSO uporabnika
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
            // - 401: API ključ manjka ali ni veljaven
            // - 400: Validacija zahtevka ni uspela
        }
    }
}
```

### Uporaba javnih API-jev (PublicApi)

Javne končne točke ne zahtevajo avtentikacije:

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

### Uporaba moderacijskih API-jev (ModerationApi)

Razred `ModerationApi` poganja nadzorno ploščo moderatorjev. Vsaka metoda sprejme parameter `sso`, ki identificira moderatorja, avtenticiranega z SSO, v čigar imenu je zahteva poslana:

```java
import com.fastcomments.api.ModerationApi;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.model.*;

ModerationApi moderationApi = new ModerationApi();

try {
    // Naštejte komentarje, ki čakajo na moderacijo
    ModerationAPIGetCommentsResponse response = moderationApi.getApiComments()
        .sso("YOUR_SSO_TOKEN")
        .execute();
    System.out.println(response);
} catch (ApiException e) {
    e.printStackTrace();
}
```

### Pogoste težave

1. **401 "missing-api-key" error**: Prepričajte se, da pokličete `apiClient.setApiKey("YOUR_KEY")` pred ustvarjanjem instance DefaultApi.
2. **Napačen razred API**: Uporabite `DefaultApi` za avtenticirane zahtevke na strežniški strani, `PublicApi` za odjemalske/javne zahtevke.
3. **API ključ je null**: SDK bo tiho preskočil avtentikacijo, če je API ključ null, kar bo vodilo do 401 napak.