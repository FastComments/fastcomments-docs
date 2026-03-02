```bash
composer require fastcomments/laravel
```

구성 파일을 게시하세요:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

자격 증명을 `.env`에 추가하세요:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

EU 리전의 경우:

```env
FASTCOMMENTS_REGION=eu
```