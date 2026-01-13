---
FastCommentsでは、記事のIDやコメントが紐付くページを、URLまたはIDのどちらでもあり得るため「URL ID」と呼びます。
URL IDは次のように定義します。コンポーネントはconfigオブジェクトの変更を監視しており、リロードされるため、URL IDを更新できます。

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### アカウントのリージョン（注意：EUのお客様）

アカウントがEUに所在する場合は、ウィジェットの設定で `region = 'eu'` を設定してください。例：

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

それ以外の場合は、`region` を定義する必要はありません。
---