---
### Uso de Nimble

```bash
nimble install fastcomments
```

### Compilar desde el código fuente

```bash
nimble build
```

### Contenido de la biblioteca

Esta biblioteca contiene el cliente de API generado y las utilidades SSO para facilitar el trabajo con la API.

- [Documentación de la biblioteca del cliente API](https://github.com/FastComments/fastcomments-nim/blob/master/client/README.md)

### APIs públicas vs protegidas

Para el cliente de API, hay dos módulos de API, `api_default` y `api_public`. El `api_default` contiene métodos que requieren tu clave de API, y `api_public` contiene llamadas de API
que se pueden realizar directamente desde un navegador/dispositivo móvil/etc sin autenticación.
---