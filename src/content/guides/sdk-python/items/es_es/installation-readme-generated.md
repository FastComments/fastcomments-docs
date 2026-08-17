### Install from GitHub

Instalar directamente desde una etiqueta de lanzamiento (recomendado, totalmente reproducible):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Fije la etiqueta en lugar de una rama para que las compilaciones sean determinísticas. La misma forma funciona en `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Cada lanzamiento etiquetado de [GitHub Release](https://github.com/FastComments/fastcomments-python/releases) también tiene una rueda construida adjunta si prefieres instalar un artefacto binario directamente.

### Library Contents

Esta biblioteca contiene dos módulos: el cliente API generado y la biblioteca central de Python que contiene utilidades escritas a mano para facilitar el trabajo con la API, incluyendo soporte SSO.

- [Documentación de la Biblioteca del Cliente API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentación de la Biblioteca Central, Incluyendo Ejemplos de SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### Public vs Secured APIs

Para el cliente API, hay tres clases, `DefaultApi`, `PublicApi` y `ModerationApi`. `DefaultApi` contiene métodos que requieren su clave API, y `PublicApi` contiene métodos que pueden ejecutarse directamente desde un navegador/dispositivo móvil/etc sin autenticación. `ModerationApi` ofrece una amplia suite de APIs de moderación en vivo y rápidas. Cada método de `ModerationApi` acepta un parámetro `sso` y puede autenticarse mediante SSO o una cookie de sesión de FastComments.com.