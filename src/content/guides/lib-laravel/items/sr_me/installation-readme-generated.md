```bash
composer require fastcomments/laravel
```

Објавите конфигурациони фајл:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Додајте своје акредитиве у `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

За ЕУ регион:

```env
FASTCOMMENTS_REGION=eu
```