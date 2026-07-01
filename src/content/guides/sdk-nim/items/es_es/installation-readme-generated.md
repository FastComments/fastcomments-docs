### Usando Nimble

```bash
nimble install fastcomments
```

### Compilando desde el código fuente

```bash
nimble build
```

### Contenido de la biblioteca

Esta biblioteca contiene el cliente API generado y las utilidades SSO para facilitar el trabajo con la API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### APIs públicas vs seguras

Para el cliente API, existen tres módulos API, `api_default`, `api_public` y `api_moderation`. El módulo `api_default` contiene métodos que requieren tu clave API, y `api_public` contiene llamadas API que pueden realizarse directamente desde un navegador/dispositivo móvil/etc. sin autenticación. El módulo `api_moderation` contiene métodos para el panel de moderador.

El módulo `api_moderation` ofrece una amplia suite de APIs de moderación en tiempo real y rápidas. Cada método `api_moderation` acepta un parámetro `sso` y puede autenticarse mediante SSO o una cookie de sesión de FastComments.com.