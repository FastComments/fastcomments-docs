### Ejecutar pruebas

```bash
# Configurar variables de entorno
export FASTCOMMENTS_API_KEY="your-api-key"
export FASTCOMMENTS_TENANT_ID="your-tenant-id"

# Ejecutar pruebas
pytest tests/
```

### Regenerar el cliente

Para regenerar el cliente de la API a partir de la especificación OpenAPI más reciente:

```bash
./update.sh
```

Esto hará:
1. Descargar la última especificación OpenAPI desde un servidor FastComments en ejecución (o usar openapi.yaml local)
2. Generar el código del cliente de Python
3. Aplanar la estructura de directorios
4. Limpiar archivos de configuración redundantes