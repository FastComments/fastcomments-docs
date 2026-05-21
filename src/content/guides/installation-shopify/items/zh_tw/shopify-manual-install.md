如果你無法安裝 [Shopify App Store app](https://apps.shopify.com/fastcomments)，你仍然可以透過編輯佈景主題來加入 FastComments。當你想要串接已擁有的 FastComments tenant，或是在無法使用 App 的 Shopify 商店上嵌入評論時，這種做法會很有用。

以 App 為基礎的安裝是大多數商店的建議方式。只有在 App 無法滿足需求時才採用此方法。

### Step 1: Disable Shopify's native comments

在你的 Shopify 管理後台，前往 **Blog posts > Manage blogs**，打開每一個部落格，並在右側面板中將 **Comments are disabled** 設定好。儲存。

這會避免 Shopify 內建的評論系統與 FastComments 同時顯示。

### Step 2: Open the blog theme template

在你的 Shopify 管理後台：

1. 前往 **Online Store > Themes**。
2. 在你目前的佈景主題下，點選 **Actions > Edit code**。
3. 在左側的檔案瀏覽器中，打開 **Sections** 並點選 `main-article.liquid`。

這是 Shopify 在呈現單篇部落格文章時所使用的模板。

### Step 3: Paste the FastComments snippet

捲動到 `main-article.liquid` 大約第 100 行、文章內容的 `</div>` 關閉標籤之後。貼上以下片段：

[inline-code-attrs-start title = 'Shopify FastComments 程式碼片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget" class="page-width page-width--narrow"></div>
<script>
    window.fcConfigs = [{
        target: '#fastcomments-widget',
        tenantId: "demo",
        urlId: window.location.pathname
    }];
</script>
[inline-code-end]

將 `"demo"` 換成你在 [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret) 上的 Tenant ID。點選 **Save**。

### Step 4: Authorize your shop domain

在你的線上商店開啟一篇部落格文章。如果你看到授權錯誤而非評論元件，表示 FastComments 需要知道你的商店被允許使用該 tenant。請參閱 [Domain Errors](/guide-installation-shopify.html#shopify-domain-errors)。

### Adding FastComments to other pages

相同的片段可用於任何 Liquid 模板，包括商品頁、客製化頁面與首頁。將它貼到你希望顯示評論的位置，並在需要每頁使用穩定識別碼時調整 `urlId`（例如在商品模板上使用 `urlId: "{{ product.id }}"`）。