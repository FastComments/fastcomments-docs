```bash
composer require fastcomments/laravel
```

Objavite konfiguracijsku datoteku:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Dodajte svoje vjerodajnice u `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

Za EU regiju:

```env
FASTCOMMENTS_REGION=eu
```