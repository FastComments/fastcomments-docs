---
### Regenerar el cliente

Para regenerar el cliente de la API a partir de la última especificación de OpenAPI:

1. Asegúrate de tener el servidor FastComments ejecutándose localmente en `http://localhost:3001`
2. Ejecuta el script de actualización:

```bash
./update.sh
```

Esto hará:
- Descargar la última especificación de OpenAPI
- Generar el código cliente en Swift (con la documentación de la API en client/docs)
- Compilar el paquete para verificar que todo funciona
---