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

- [Documentación de la biblioteca del cliente de la API](https://github.com/FastComments/fastcomments-cpp/blob/master/client/README.md)

### APIs públicas vs seguras

Para el cliente de API, hay tres clases, `DefaultApi`, `PublicApi`, and `ModerationApi`. The `DefaultApi` contains methods that require your API key, and `PublicApi` contiene
métodos que se pueden realizar directamente desde un navegador/dispositivo móvil/etc sin autenticación. La `ModerationApi` contiene métodos que impulsan el panel del moderador - listado,
conteo, búsqueda, exportación y obtención de registros para comentarios, acciones de moderación (eliminar/restaurar, marcar, establecer estado de revisión/spam/aprobación, ajustar votos, reabrir/cerrar hilos),
prohibiciones (banear desde un comentario, deshacer prohibiciones, resúmenes previos a la prohibición, estado y preferencias de la prohibición, recuentos de usuarios prohibidos), y insignias y confianza (otorgar/quitar insignias, insignias manuales, obtener/establecer confianza
factor, perfil interno del usuario). Cada método de `ModerationApi` acepta un parámetro `sso` para que la llamada se realice en nombre de un moderador autenticado por SSO.