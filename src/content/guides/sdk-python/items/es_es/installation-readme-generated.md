### PyPI

```bash
pip install fastcomments
```

### Contenido de la biblioteca

Esta biblioteca contiene dos módulos: el cliente de API generado y la biblioteca principal de Python que contiene utilidades escritas a mano para facilitar el trabajo con la API, incluido el soporte SSO.

- [Documentación de la biblioteca del cliente de la API](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [Documentación de la biblioteca principal, incluidos ejemplos de SSO](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### APIs públicas vs seguras

Para el cliente de la API, hay dos clases, `DefaultApi` y `PublicApi`. La `DefaultApi` contiene métodos que requieren tu clave de API, y la `PublicApi` contiene llamadas a la API que se pueden realizar directamente desde un navegador, dispositivo móvil, etc., sin autenticación.