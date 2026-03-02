```bash
composer require fastcomments/laravel
```

Публикувайте конфигурационния файл:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Добавете вашите учетни данни в `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

За региона на ЕС:

```env
FASTCOMMENTS_REGION=eu
```