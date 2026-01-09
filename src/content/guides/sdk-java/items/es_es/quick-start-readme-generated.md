### Uso de APIs autenticadas (DefaultApi)

**Importante:** Debes configurar tu clave API en el ApiClient antes de realizar solicitudes autenticadas. Si no lo haces, las solicitudes fallarán con un error 401.

```java
import com.fastcomments.invoker.ApiClient;
import com.fastcomments.invoker.ApiException;
import com.fastcomments.api.DefaultApi;
import com.fastcomments.model.*;

public class Example {
    public static void main(String[] args) {
        // Crear y configurar el cliente de la API
        ApiClient apiClient = new ApiClient();

        // REQUERIDO: Establece tu clave API (obténla desde tu panel de FastComments)
        apiClient.setApiKey("YOUR_API_KEY_HERE");

        // Crear la instancia de la API con el cliente configurado
        DefaultApi api = new DefaultApi(apiClient);

        // Ahora puedes hacer llamadas a la API autenticadas
        try {
            // Ejemplo: Añadir un usuario SSO
            CreateAPISSOUserData userData = new CreateAPISSOUserData();
            userData.setId("user-123");
            userData.setEmail("user@example.com");
            userData.setDisplayName("John Doe");

            AddSSOUserAPIResponse response = api.addSSOUser("YOUR_TENANT_ID", userData)
                .execute();
            System.out.println("User created: " + response);

        } catch (ApiException e) {
            System.err.println("Error: " + e.getResponseBody());
            // Errores comunes:
            // - 401: La clave API falta o no es válida
            // - 400: La validación de la solicitud falló
        }
    }
}
```

### Uso de APIs públicas (PublicApi)

Los endpoints públicos no requieren autenticación:

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

### Problemas comunes

1. **401 "missing-api-key" error**: Asegúrate de llamar a `apiClient.setApiKey("YOUR_KEY")` antes de crear la instancia DefaultApi.
2. **Wrong API class**: Usa `DefaultApi` para solicitudes autenticadas del lado del servidor, `PublicApi` para solicitudes del lado del cliente/públicas.
3. **Null API key**: El SDK omitirá la autenticación silenciosamente si la clave API es null, lo que llevará a errores 401.