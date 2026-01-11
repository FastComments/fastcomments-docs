アプリケーションの Gemfile に次の行を追加してください:

```ruby
gem 'fastcomments'
```

その後、次を実行してください:

```bash
bundle install
```

あるいは手動でインストールするには:

```bash
gem install fastcomments
```

### ライブラリの内容

このライブラリには、生成された API クライアントと API の利用を容易にする SSO ユーティリティが含まれています。

- [API クライアント ライブラリのドキュメント](https://github.com/FastComments/fastcomments-ruby/blob/master/client/README.md)

### 公開 API と保護された API

API クライアントには `DefaultApi` と `PublicApi` の2つのクラスがあります。`DefaultApi` は API キーを必要とするメソッドを含み、`PublicApi` はブラウザ／モバイルデバイス等から認証なしで直接呼び出せる API 呼び出しを含みます。