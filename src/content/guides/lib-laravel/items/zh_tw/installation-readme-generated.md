```bash
composer require fastcomments/laravel
```

發佈設定檔：

```bash
php artisan vendor:publish --tag=fastcomments-config
```

將您的憑證加入 `.env`：

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

如需 EU 區域：

```env
FASTCOMMENTS_REGION=eu
```