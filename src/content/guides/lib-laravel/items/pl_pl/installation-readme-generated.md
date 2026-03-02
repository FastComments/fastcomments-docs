```bash
composer require fastcomments/laravel
```

Opublikuj plik konfiguracyjny:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Dodaj swoje dane uwierzytelniające do `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

Dla regionu UE:

```env
FASTCOMMENTS_REGION=eu
```