アプリケーションの Gemfile に以下の行を追加します:

```ruby
gem 'fastcomments'
```

その後、以下を実行します:

```bash
bundle install
```

または、自分でインストールする場合は:

```bash
gem install fastcomments
```

### ライブラリの内容

このライブラリには、生成された API クライアントと、API の操作を容易にするための SSO ユーティリティが含まれています。

- [API クライアントライブラリのドキュメント](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### 公開 API と保護された API

API クライアントには、`DefaultApi`、`PublicApi`、`ModerationApi` の 3 つのクラスがあります。`DefaultApi` には API キーを必要とするメソッドが含まれており、`PublicApi` には認証なしでブラウザやモバイルデバイスなどから直接実行できる API 呼び出しが含まれています。`ModerationApi` には、モデレーターダッシュボードを動かすメソッドが含まれています。

`ModerationApi` は、ライブかつ高速なモデレーション API の充実したスイートを提供します。すべての `ModerationApi` メソッドは `sso` パラメータを受け入れ、SSO または FastComments.com のセッションクッキーによる認証が可能です。