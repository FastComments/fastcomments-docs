### Instalar desde GitHub

Instale directamente desde una etiqueta de lanzamiento (recomendado, totalmente reproducible):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Fije la etiqueta en lugar de una rama para que las compilaciones sean determinísticas. La misma forma funciona en `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.0.0
```

Cada [GitHub Release](https://github.com/fastcomments/fastcomments-python/releases) etiquetado también tiene una rueda construida adjunta si prefiere instalar un artefacto binario directamente.

### Contenido de la Biblioteca

Esta biblioteca contiene dos módulos: el cliente API generado y la biblioteca central de Python que contiene utilidades escritas a mano para facilitar el trabajo con la API, incluido el soporte SSO.

- [Documentación del Cliente API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentación de la Biblioteca Central, Incluyendo Ejemplos SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### APIs Públicas vs Seguras

Para el cliente API, hay tres clases, `DefaultApi`, `PublicApi` y `ModerationApi`. La `DefaultApi` contiene métodos que requieren su clave API, y la `PublicApi` contiene métodos que pueden ejecutarse directamente desde un navegador/dispositivo móvil/etc sin autenticación. La `ModerationApi` proporciona una suite extensa de APIs de moderación en tiempo real y rápidas. Cada método de `ModerationApi` acepta un parámetro `sso` y puede autenticarse vía SSO o mediante una cookie de sesión de FastComments.com.