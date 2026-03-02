```bash
composer require fastcomments/laravel
```

Publica el archivo de configuración:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Añade tus credenciales a `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

Para la región de la UE:

```env
FASTCOMMENTS_REGION=eu
```