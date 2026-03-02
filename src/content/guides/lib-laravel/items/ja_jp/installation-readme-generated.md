```bash
composer require fastcomments/laravel
```

設定ファイルを公開してください:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

認証情報を `.env` に追加してください:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

EUリージョンの場合:

```env
FASTCOMMENTS_REGION=eu
```