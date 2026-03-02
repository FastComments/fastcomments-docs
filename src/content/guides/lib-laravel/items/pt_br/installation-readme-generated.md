```bash
composer require fastcomments/laravel
```

Publique o arquivo de configuração:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

Adicione suas credenciais ao `.env`:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

Para a região da UE:

```env
FASTCOMMENTS_REGION=eu
```