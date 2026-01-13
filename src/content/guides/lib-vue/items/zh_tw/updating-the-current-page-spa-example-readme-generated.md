在 FastComments 中，我們將文章 id（或留言所綁定的頁面）稱為 URL ID，因為它可以是 url 或一個 ID。
請以下列方式定義 URL ID。元件會監聽 config object 的變更並重新載入，因此你只需更新 "url" 和 "urlId" 設定即可。

完整的運作範例請見 [這裡](https://github.com/FastComments/fastcomments-vue/blob/master/dev/serve-pagination.ts)。

透過以下指令執行分頁範例：

```
npm run serve-pagination
```

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id'}" />
```

### 帳戶區域（注意：歐盟客戶）

如果你的帳戶位於歐盟，請在 widget 設定中將 `region = 'eu'`，例如：

```vue
<fast-comments-vue v-bind:config="{tenantId: 'demo', url: 'https://example.com', urlId: 'some-page-id', region: 'eu'}" />
```

否則，你不必定義 `region`。