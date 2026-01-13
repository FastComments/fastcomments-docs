在 FastComments 中，我們稱文章 id 或評論所綁定的頁面為 URL ID，因為它可以是 url 或一個 ID。
請以以下方式定義 URL ID。元件會監聽 config 物件的變更並重新載入，因此你可以更新 URL ID。

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id'}" />
```

### 帳戶地區（注意：歐盟客戶）

如果你的帳戶位於歐盟，請在小工具配置中設定 `region = 'eu'`，例如：

```vue
<FastComments v-bind:config="{tenantId: 'demo', url: 'https://example.com/somepage', urlId: 'some-page-id', region: 'eu'}" />
```

否則，你不必定義 `region`。