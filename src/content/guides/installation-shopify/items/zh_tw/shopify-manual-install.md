如果您無法安裝 [Shopify App Store 應用程式](https://apps.shopify.com/fastcomments)，您仍然可以透過編輯佈景主題來新增 FastComments。當您想要串接已經擁有的 FastComments 租戶，或是在無法使用應用程式的 Shopify 店面上嵌入留言時，這個方法會很有用。

對大多數商店而言，透過應用程式安裝是官方支援的方式。只有在應用程式不適合您的情況時，才使用此方法。

### 步驟 1：停用 Shopify 的原生留言功能

在您的 Shopify 管理介面中，前往 **Blog posts > Manage blogs**，開啟每個網誌，並在右側面板中設定 **Comments are disabled**。儲存。

這樣可以避免 Shopify 內建的留言功能與 FastComments 同時顯示。

### 步驟 2：開啟網誌的佈景主題範本

在您的 Shopify 管理介面中：

1. 前往 **Online Store > Themes**。
2. 在您目前的佈景主題下，點擊 **Actions > Edit code**。
3. 在左側的檔案瀏覽器中，開啟 **Sections** 並點擊 `main-article.liquid`。

這是 Shopify 用來呈現單篇網誌文章的範本。

### 步驟 3：貼上 FastComments 程式碼片段

捲動至 `main-article.liquid` 大約第 100 行的位置，也就是文章內容的 `</div>` 結束標籤之後。貼上以下程式碼片段：

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

將 `"demo"` 替換為您自己的 Tenant ID，可從 [fastcomments.com/auth/my-account/api-secret](https://fastcomments.com/auth/my-account/api-secret) 取得。點擊 **Save**。

### 步驟 4：授權您的商店網域

在您的線上商店開啟一篇網誌文章。如果您看到的是授權錯誤而非留言小工具，表示 FastComments 需要知道您的商店獲准使用此租戶。請參閱 [Domain Errors](/guide-installation-shopify.html#shopify-domain-errors)。

### 將 FastComments 新增至其他頁面

同樣的程式碼片段可用於任何 Liquid 範本，包括產品頁面、自訂頁面和首頁。將它貼在您希望留言出現的位置，如果您想為每個頁面使用穩定的識別碼，可以調整 `urlId`（例如，在產品範本上使用 `urlId: "{{ product.id }}"`）。