```bash
composer require fastcomments/laravel
```

Yapılandırma dosyasını yayımla:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Kimlik bilgilerinizi `.env` dosyasına ekleyin:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

AB bölgesi için:

```env
FASTCOMMENTS_REGION=eu
```