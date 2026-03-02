```bash
composer require fastcomments/laravel
```

Опубликуйте файл конфигурации:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Добавьте ваши учетные данные в `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

Для региона ЕС:

```env
FASTCOMMENTS_REGION=eu
```