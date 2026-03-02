```bash
composer require fastcomments/laravel
```

Publicer konfigurationsfilen:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Tilføj dine legitimationsoplysninger til `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

For EU-regionen:

```env
FASTCOMMENTS_REGION=eu
```