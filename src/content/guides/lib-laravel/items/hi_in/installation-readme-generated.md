```bash
composer require fastcomments/laravel
```

कॉन्फ़िगरेशन फ़ाइल प्रकाशित करें:

```bash
php artisan vendor:publish --tag=fastcomments-config
```

अपने क्रेडेंशियल्स `.env` में जोड़ें:

```env
FASTCOMMENTS_TENANT_ID=your-tenant-id
FASTCOMMENTS_API_KEY=your-api-key
```

EU क्षेत्र के लिए:

```env
FASTCOMMENTS_REGION=eu
```