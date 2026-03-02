---
```bash
composer require fastcomments/laravel
```

Objavite konfiguracioni fajl:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Dodajte svoje podatke za pristup u `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

Za region EU:

```env
FASTCOMMENTS_REGION=eu
```
---