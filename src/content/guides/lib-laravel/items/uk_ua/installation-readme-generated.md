```bash
composer require fastcomments/laravel
```

Опублікуйте файл конфігурації:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Додайте свої облікові дані до `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

Для регіону ЄС:

```env
FASTCOMMENTS_REGION=eu
```