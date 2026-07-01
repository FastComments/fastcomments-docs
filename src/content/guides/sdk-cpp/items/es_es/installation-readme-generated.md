### Instalar dependencias

```bash
sudo apt install libcpprest-dev libboost-all-dev
```

### Compilar desde el código fuente

```bash
mkdir build
cd build
cmake ..
make
```

### Instalación

```bash
sudo make install
```

### Contenido de la biblioteca

Esta biblioteca contiene el cliente API generado y las utilidades SSO para facilitar el trabajo con la API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### APIs públicas vs seguras

Para el cliente API, existen tres clases, `DefaultApi`, `PublicApi` y `ModerationApi`. El `DefaultApi` contiene métodos que requieren su clave API, y el `PublicApi` contiene métodos que pueden ejecutarse directamente desde un navegador, dispositivo móvil, etc., sin autenticación. El `ModerationApi` ofrece un conjunto extenso de APIs de moderación en tiempo real y rápidas. Cada método del `ModerationApi` acepta un parámetro `sso` y puede autenticarse mediante SSO o una cookie de sesión de FastComments.com.