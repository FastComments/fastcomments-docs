---
この行をアプリケーションの Gemfile に追加してください:

```ruby
gem 'fastcomments'
```

そして次を実行してください:

```bash
bundle install
```

または、手動でインストールするには:

```bash
gem install fastcomments
```

### ライブラリの内容

このライブラリには、自動生成された API クライアントと、API の利用を容易にする SSO ユーティリティが含まれます。

- [APIクライアントライブラリのドキュメント](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### 公開 API と保護された API

API クライアントには `DefaultApi`、`PublicApi`、`ModerationApi` の3つのクラスがあります。`DefaultApi` は API キーを必要とするメソッドを含み、`PublicApi` はブラウザ／モバイル端末などから認証なしで直接行える API 呼び出しを含みます。`ModerationApi` はモデレーターダッシュボードを駆動するメソッドを含みます。

`ModerationApi` はコメントのモデレーション（一覧、件数、検索、ログ、エクスポート）、モデレーション操作（削除/復元、フラグ、レビュー/スパム/承認ステータスの設定、投票、スレッドの再開/クローズ）、バン（コメントからのバン、取り消し、事前バンの要約、バンの状態/設定、バンされたユーザーの集計）、およびバッジと信頼（バッジの授与/削除、手動バッジ、信頼度の取得/設定、ユーザー内部プロファイル）を扱います。  
各 `ModerationApi` メソッドは `sso` パラメータを受け取り、SSO 認証済みのモデレーターを代理してリクエストを行えるようにします。
---