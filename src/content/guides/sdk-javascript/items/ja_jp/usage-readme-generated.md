この SDK は、最適な互換性とセキュリティを確保するために、ブラウザとサーバー環境それぞれに対して別々のエントリーポイントを提供します:

### ブラウザでの使用（クライアント側）

ブラウザ／フロントエンドのアプリケーションでは、Node.js の依存関係を含まないブラウザ用の安全なエクスポートを使用してください:

```typescript
// ブラウザ安全なインポート（Node.js の依存関係なし）
import { createFastCommentsBrowserSDK } from 'fastcomments-sdk/browser';

// ブラウザ用 SDK インスタンスを作成
const sdk = createFastCommentsBrowserSDK({
  basePath: 'https://fastcomments.com' // オプション、デフォルトは https://fastcomments.com
});

// Use public APIs (no API key needed - safe for browsers)
const comments = await sdk.publicApi.getCommentsPublic({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

公開 API を使用します（API キーは不要 - ブラウザで安全です）

### サーバーでの使用（Node.js）

サーバー／バックエンドのアプリケーションでは、SSO や認証機能を含むフル SDK を使用してください:

```typescript
// サーバー側インポート（SSO を含み、NodeJS で動作するよう設計されています）
import { createFastCommentsSDK } from 'fastcomments-sdk/server';

// サーバー用 SDK インスタンスを作成
const sdk = createFastCommentsSDK({
  apiKey: 'your-api-key', // これはサーバー側で秘密にしてください！
  basePath: 'https://fastcomments.com' // オプション、デフォルトは https://fastcomments.com
});

// API キーを使って保護された API を使用します
const comments = await sdk.defaultApi.getComments({ 
  tenantId: 'your-tenant-id',
  urlId: 'page-url-id'
});
```

### 型のみのインポート

TypeScript の型のみが必要で（ランタイムコードは不要な場合）は、デフォルトのインポートを使用してください:

```typescript
// 型のみ（ランタイム依存なし - どこでも安全）
import type { 
  PublicComment, 
  CreateCommentParams, 
  GetCommentsPublic200Response 
} from 'fastcomments-sdk';
```

### 個別の API クラスの使用

#### ブラウザ環境

```typescript
import { PublicApi, Configuration } from 'fastcomments-sdk/browser';

const config = new Configuration({
  basePath: 'https://fastcomments.com'
});

const publicApi = new PublicApi(config);
```

#### サーバー環境  

```typescript
import { DefaultApi, PublicApi, Configuration } from 'fastcomments-sdk/server';

const config = new Configuration({
  apiKey: 'your-api-key',
  basePath: 'https://fastcomments.com'
});

const defaultApi = new DefaultApi(config);
const publicApi = new PublicApi(config);
```