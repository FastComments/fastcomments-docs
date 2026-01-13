---
在 FastComments 中，我们将文章 id（或评论所绑定的页面）称为 URL ID，因为它可以是一个 url 或一个 ID。
按以下方式定义 URL ID。组件会监听 config 对象的更改并重新加载，因此您可以更新 URL ID。

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### 账户区域（注意：欧盟客户）

如果您的账户位于欧盟，请在小部件配置中将 `region = 'eu'` 设置，例如：

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

否则，您无需定义 `region`。
---