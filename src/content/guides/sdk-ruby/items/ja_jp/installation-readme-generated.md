Add this line to your application's Gemfile:

```ruby
gem 'fastcomments'
```

And then execute:

```bash
bundle install
```

Or install it yourself as:

```bash
gem install fastcomments
```

### Library Contents

このライブラリには、生成された API クライアントと、API の使用を簡単にする SSO ユーティリティが含まれています。

- [API Client Library Docs](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### Public vs Secured APIs

API クライアントには、`DefaultApi`、`PublicApi`、`ModerationApi` の 3 つのクラスがあります。`DefaultApi` には API キーが必要なメソッドが含まれ、`PublicApi` には認証なしでブラウザやモバイルデバイス等から直接呼び出せる API 呼び出しが含まれます。`ModerationApi` にはモデレーター ダッシュボードを支えるメソッドが含まれています。

`ModerationApi` は、豊富なライブおよび高速モデレーション API のスイートを提供します。すべての `ModerationApi` メソッドは `sso` パラメータを受け取り、SSO または FastComments.com のセッション cookie を使用して認証できます。