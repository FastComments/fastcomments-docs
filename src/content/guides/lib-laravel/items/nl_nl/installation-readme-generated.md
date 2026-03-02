```bash
composer require fastcomments/laravel
```

Publiceer het configuratiebestand:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Voeg uw referenties toe aan `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

Voor de EU-regio:

```env
FASTCOMMENTS_REGION=eu
```