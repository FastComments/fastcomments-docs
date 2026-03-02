```bash
composer require fastcomments/laravel
```

发布配置文件：

```bash
php artisan vendor:publish --tag=fastcomments-config
```

将凭据添加到 `.env`：

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

对于欧盟区域：

```env
FASTCOMMENTS_REGION=eu
```