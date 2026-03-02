```bash
composer require fastcomments/laravel
```

פרסם את קובץ התצורה:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

הוסף את האישורים שלך ל-`.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

לאזור האיחוד האירופי:

```env
FASTCOMMENTS_REGION=eu
```