---
接下來我們需要找出要將 FastComments.com 小工具程式碼加入的位置。

如果您使用預設的 `casper` 主題，您會在第 `82` 行看到像這樣的一個區段：

<div class="screenshot white-bg">
    <div class="title">已停用的評論區</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="已停用的評論區" />
</div>

如果您使用其他主題，則不會看到此內容，您需要在最後一個 `</section>` 之後加入此程式碼：

[inline-code-attrs-start title = '區段範例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

您應該會有類似這樣的準備：

<div class="screenshot white-bg">
    <div class="title">範本已準備好放置評論程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="範本已準備好放置評論程式碼" />
</div>

準備就緒後，複製 FastComments.com 小工具程式碼：

[inline-code-attrs-start title = 'Ghost FastComments.com 評論程式碼片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script async src="https://cdn.fastcomments.com/js/embed-v2-async.min.js"></script>
<div id="fastcomments-widget"></div>
<script>
    (function () {
        let simpleSSO = null;

        \{{#if access}}
            \{{#if @member}}
                simpleSSO = {
                    id: '\{{ @member.uuid }}',
                    email: '\{{@member.email}}',
                    username: '\{{@member.name}}',
                    avatar: '\{{ @member.avatar_image }}',
                    optedInNotifications: true,
                    optedInSubscriptionNotifications: true,
                    displayLabel: '\{{@member.labels}}'
                }
            \{{/if}}
        \{{/if}}

        window.fcConfigs = [{
            target: '#fastcomments-widget',
            tenantId: "demo",
            urlId: window.location.pathname,
            allowAnon: false,
            simpleSSO: simpleSSO
        }];
    })();
</script>
[inline-code-end]

…它應該會看起來像這樣：

<div class="screenshot white-bg">
    <div class="title">加入 FastComments.com 評論程式碼</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="加入 FastComments.com 評論程式碼" />
</div>

程式碼撰寫完成。現在我們只要重新匯入主題即可！

---