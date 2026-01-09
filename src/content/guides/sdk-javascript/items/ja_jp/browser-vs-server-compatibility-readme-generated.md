---
この SDK は、最適な互換性を確保し実行時エラーを防ぐために **デュアルエントリポイント** を使用します:

- **`fastcomments-sdk/browser`** - ブラウザ安全版（ネイティブの `fetch` 使用）
- **`fastcomments-sdk/server`** - SSO をサポートするフル Node.js バージョン
- **`fastcomments-sdk`** (デフォルト) - 型定義のみ、どこでも安全にインポート可能
---