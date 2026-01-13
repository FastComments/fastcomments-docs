FastCommentsでは、記事IDまたはコメントが紐づくページをURL IDと呼びます。これはURLまたはIDになり得ます。
以下の方法でURL IDを定義してください。コンポーネントはconfigオブジェクトの変更を監視してリロードするため、"url"と"urlId"の設定を更新するだけで構いません。

完全な動作例は[こちら](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts)を参照してください。

次のコマンドでページネーションの例を実行します:

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### Account Region (ATTENTION: EU Customers)

アカウントがEUにある場合は、ウィジェット設定で`region = 'eu'`を設定してください。例えば:

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

それ以外の場合は`region`を定義する必要はありません。