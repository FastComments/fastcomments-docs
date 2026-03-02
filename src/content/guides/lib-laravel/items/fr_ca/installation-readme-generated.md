```bash
composer require fastcomments/laravel
```

Publiez le fichier de configuration :

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Ajoutez vos identifiants dans `.env` :

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

Pour la région UE :

```env
FASTCOMMENTS_REGION=eu
```