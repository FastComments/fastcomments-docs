---
### PyPI

```bash
pip install fastcomments
```

### ライブラリの内容

このライブラリは2つのモジュールを含んでいます：生成されたAPIクライアントと、SSOサポートを含むAPIの利用を容易にする手書きのユーティリティを含むコアPythonライブラリ。

- [APIクライアントライブラリのドキュメント](https://github.com/FastComments/fastcomments-python/blob/main/client/README.md)
- [コアライブラリのドキュメント（SSOの例を含む）](https://github.com/FastComments/fastcomments-python/blob/main/sso/README.md)

### 公開APIと保護されたAPI

APIクライアントには `DefaultApi`、`PublicApi`、`ModerationApi` の3つのクラスがあります。`DefaultApi` はAPIキーが必要なメソッドを含み、`PublicApi` はブラウザやモバイルデバイスなどから認証なしで直接呼び出せるメソッドを含みます。`ModerationApi` はモデレーターダッシュボードを支え、コメントのモデレーション（一覧、カウント、検索、ログ、エクスポート）、モデレーションアクション（削除/復元、フラグ、レビュー/スパム/承認ステータスの設定、投票、スレッドの再開/閉鎖）、バン（コメントからのバン、取り消し、事前バンの概要、バンのステータス/設定、バンされたユーザーのカウント）、およびバッジと信頼（バッジの付与/削除、手動バッジ、信頼度の取得/設定、ユーザーの内部プロファイル）に関するメソッドを含みます。すべての `ModerationApi` メソッドは `sso` パラメータを受け取り、SSO認証されたモデレーターを代理して呼び出すことができます。
---