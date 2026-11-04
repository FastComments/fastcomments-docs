接下来我们需要确定在哪里添加 FastComments.com 小部件代码。

如果您使用的是默认的 `casper` 主题，您会在第 `82` 行看到类似这样的部分：

<div class="screenshot white-bg">
    <div class="title">已禁用的评论区</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="已禁用的评论区" />
</div>

如果您使用的是其他主题，则不会看到这个，需要在最后一个 `</section>` 之后添加以下代码：

[inline-code-attrs-start title = 'Section 示例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

您应该准备好类似这样的内容：

<div class="screenshot white-bg">
    <div class="title">准备好添加评论代码的模板</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="准备好添加评论代码的模板" />
</div>

准备就绪后，复制 FastComments.com 小部件代码：

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

……它应该看起来像这样：

<div class="screenshot white-bg">
    <div class="title">添加 FastComments.com 评论代码</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="添加 FastComments.com 评论代码" />
</div>

代码部分完成了。现在我们只需重新导入主题即可！