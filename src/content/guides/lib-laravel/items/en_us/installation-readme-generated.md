```bash
composer require fastcomments/laravel
```

Publish the configuration file:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Add your credentials to `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

For the EU region:

```env
FASTCOMMENTS_REGION=eu
```