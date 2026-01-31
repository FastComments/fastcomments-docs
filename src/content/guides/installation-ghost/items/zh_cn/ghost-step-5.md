接下来我们需要确定在何处添加 FastComments.com 小部件代码。

如果您使用默认的 `casper` 主题，您将在第 `82` 行看到如下部分：

<div class="screenshot white-bg">
    <div class="title">已禁用的评论部分</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="已禁用的评论部分" />
</div>

如果您使用其他主题，您将看不到此内容，需要在最后一个 `</section>` 之后添加此代码：

[inline-code-attrs-start title = 'Section 示例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

您应该会看到类似如下的准备状态：

<div class="screenshot white-bg">
    <div class="title">模板已准备好用于评论代码</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="模板已准备好用于评论代码" />
</div>

准备好后，复制 FastComments.com 小部件代码：

[inline-code-attrs-start title = 'Ghost FastComments.com 评论代码片段'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...and it should look like this:

<div class="screenshot white-bg">
    <div class="title">添加 FastComments.com 评论代码</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="添加 FastComments.com 评论代码" />
</div>

编码完成。现在我们只需重新导入主题！

---