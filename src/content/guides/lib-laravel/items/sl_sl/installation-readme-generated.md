```bash
composer require fastcomments/laravel
```

Objavite konfiguracijsko datoteko:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Dodajte svoje poverilnice v `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

Za regijo EU:

```env
FASTCOMMENTS_REGION=eu
```