```bash
composer require fastcomments/laravel
```

Pubblica il file di configurazione:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Aggiungi le tue credenziali in `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

Per la regione UE:

```env
FASTCOMMENTS_REGION=eu
```