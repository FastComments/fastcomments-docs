```bash
composer require fastcomments/laravel
```

Δημοσιεύστε το αρχείο ρυθμίσεων:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Προσθέστε τα διαπιστευτήριά σας στο `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

Για την περιοχή ΕΕ:

```env
FASTCOMMENTS_REGION=eu
```