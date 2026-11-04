### Instalar desde GitHub

Instala directamente desde una etiqueta de release (recomendado, totalmente reproducible):

```bash
pip install git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Fija la etiqueta en lugar de una rama para que las compilaciones sean deterministas. La misma forma funciona en `requirements.txt`:

```
fastcomments @ git+https://github.com/fastcomments/fastcomments-python.git@v3.1.0
```

Cada [GitHub Release](https://github.com/FastComments/fastcomments-python/releases) etiquetado también tiene un wheel compilado adjunto si prefieres instalar un artefacto binario directamente.

### Contenido de la Librería

Esta librería contiene dos módulos: el cliente de API generado y la librería principal de Python que contiene utilidades escritas a mano para facilitar el trabajo con la API, incluyendo soporte para SSO.

- [Documentación de la Librería Cliente de API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentación de la Librería Principal, Incluyendo Ejemplos de SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### APIs Públicas vs Aseguradas

Para el cliente de API, hay tres clases: `DefaultApi`, `PublicApi` y `ModerationApi`. La `DefaultApi` contiene métodos que requieren tu clave de API, y la `PublicApi` contiene métodos que pueden realizarse directamente desde un navegador/dispositivo móvil/etc sin autenticación. La `ModerationApi` proporciona un extenso conjunto de APIs de moderación en vivo y rápidas. Cada método de `ModerationApi` acepta un parámetro `sso` y puede autenticarse mediante SSO o una cookie de sesión de FastComments.com.