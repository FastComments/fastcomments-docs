要讓 Weebly 與 FastComments 的整合順利運作，我們需要加入兩段小程式碼。

第一段用來隱藏 Weebly 的「Comments are Closed」訊息，第二段則是實際載入 FastComments。

首先，複製這段小程式碼片段：

[inline-code-attrs-start title = 'FastComments 標頭程式碼片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<style>
    #comments {
        display: none;
    }
    #commentArea:not(.loaded) {
        display: none;
    }
    #commentArea.loaded {
        display: block !important;
    }
</style>
[inline-code-end]

接著，在 `Step One` 的同一個設定頁面，點選 `+` 在 `Post header code` 旁邊。

<div class="screenshot white-bg">
    <div class="title">開啟篇文章標頭程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-1-open-post-header-code.png" alt="開啟篇文章標頭程式碼" />
</div>

你會看到像這樣打開的文字方塊：

<div class="screenshot white-bg">
    <div class="title">篇文章標頭程式碼已打開</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-2-post-header-code-open.png" alt="篇文章標頭程式碼已打開" />
</div>

現在把我們的程式碼片段貼上：

<div class="screenshot white-bg">
    <div class="title">已貼上標頭程式碼片段</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-3-post-header-code-added.png" alt="已貼上標頭程式碼片段" />
</div>

接下來是啟用 FastComments 的頁腳程式碼。點選 `Post footer code` 旁的加號：

<div class="screenshot white-bg">
    <div class="title">開啟篇文章頁腳程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-4-open-post-footer-code.png" alt="開啟篇文章頁腳程式碼" />
</div>

複製這段專為 **Weebly** 設計的程式碼片段：

[inline-code-attrs-start title = 'FastComments 頁腳程式碼片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let loaded = false;
        let interval = null;
        function attemptLoad() {
            if (loaded) {
                clearInterval(interval);
                return;
            }
            const comments = document.getElementById('comments');
            if (comments) { // 移除顯示評論按鈕
                comments.remove();
            }
            const commentArea = document.getElementById('commentArea');
            if (!commentArea) {
                return;
            }
            commentArea.innerHTML = '';
            commentArea.classList.add('loaded');
            FastCommentsUI(commentArea, {
                tenantId: "demo",
                urlId: window.location.pathname
            });
            loaded = true;
            clearInterval(interval);
        }
        attemptLoad();
        interval = setInterval(attemptLoad, 300);
    })();
</script>
[inline-code-end]

現在把我們的頁腳程式碼貼上：

<div class="screenshot white-bg">
    <div class="title">已加入篇文章頁腳程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/weebly-step-2-5-post-footer-code-added.png" alt="已加入篇文章頁腳程式碼" />
</div>

就這樣！

---