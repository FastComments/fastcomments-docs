The SDK は3つの主要な API クラスを提供します:

- **`DefaultApi`** - 認証に API キーを必要とする保護されたエンドポイント。サーバー側の操作に使用してください。
- **`PublicApi`** - API キーなしでアクセスできる公開エンドポイント。ブラウザやモバイルデバイスなどから直接呼び出せます。
- **`HiddenApi`** - 高度なユースケース向けの内部／管理者用エンドポイント。

### 例: Public API の使用（ブラウザ対応）

```typescript
import { PublicApi } from 'fastcomments-sdk/browser';

const publicApi = new PublicApi();

// ページのコメントを取得（API キー不要）
const response = await publicApi.getCommentsPublic({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 例: Default API の使用（サーバー側のみ）

```typescript
import { DefaultApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key' // これは秘密にしてください！
});
const defaultApi = new DefaultApi(config);

// 管理者権限でコメントを取得
const response = await defaultApi.getComments({
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```