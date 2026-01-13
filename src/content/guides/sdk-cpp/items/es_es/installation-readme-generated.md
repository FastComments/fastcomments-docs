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

Esta biblioteca contiene el cliente de API generado y las utilidades SSO para facilitar el trabajo con la API.

- [API Client Library Docs](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### APIs públicas frente a protegidas

Para el cliente de API, hay dos clases, `DefaultAPI` y `PublicAPI`. `DefaultAPI` contiene métodos que requieren su clave de la API, y `PublicAPI` contiene llamadas a la API
que pueden realizarse directamente desde un navegador/dispositivo móvil/etc. sin autenticación.