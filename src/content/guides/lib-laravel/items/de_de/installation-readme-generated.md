```bash
composer require fastcomments/laravel
```

Veröffentlichen Sie die Konfigurationsdatei:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Fügen Sie Ihre Zugangsdaten zur Datei `.env` hinzu:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

Für die EU-Region:

```env
FASTCOMMENTS_REGION=eu
```