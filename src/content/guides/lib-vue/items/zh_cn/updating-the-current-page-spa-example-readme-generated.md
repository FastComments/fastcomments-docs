在 FastComments 中，我们将文章 id，或评论绑定到的页面，称为 URL ID，因为它可以是一个 url 或一个 ID。  
请按以下方式定义 URL ID。组件会监视 config 对象的更改并重新加载，因此您只需更新 "url" 和 "urlId" 设置即可。

完整的可运行示例请见 [这里](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts)。

通过以下命令运行分页示例：

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### 帐户区域（注意：欧盟客户）

如果您的帐户位于欧盟，请在小部件配置中设置 `region = 'eu'`，例如：

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

否则，您无需定义 `region`。