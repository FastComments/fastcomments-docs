FastCommentsでは、記事ID、つまりコメントが紐付けられるページのことを、URLにもIDにもなり得ることから「URL ID」と呼んでいます。

URL IDは以下のように定義します。コンポーネントはconfigオブジェクトの変更を監視しており、変更があれば再読み込みするため、"url" と "urlId" の設定を更新するだけでOKです。

完全に動作する例は[こちら](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts)でご覧いただけます。

ページネーションの例は以下のコマンドで実行できます:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### アカウントのリージョン(注意: EUのお客様へ)

アカウントがEUにある場合は、ウィジェットの設定で `region = 'eu'` を指定してください。例:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

それ以外の場合は、`region` を定義する必要はありません。