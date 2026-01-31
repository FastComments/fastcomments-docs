---
次に、FastComments.com ウィジェットコードを追加する場所を特定する必要があります。

デフォルトの `casper` テーマを使用している場合、82行目に次のようなセクションが表示されます：

<div class="screenshot white-bg">
    <div class="title">無効化されたコメントセクション</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="無効化されたコメントセクション" />
</div>

他のテーマを使用している場合はこれが表示されないため、最後の `</section>` の後にこのコードを追加する必要があります：

[inline-code-attrs-start title = 'セクションの例'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

以下のようになっているはずです：

<div class="screenshot white-bg">
    <div class="title">コメントコード用テンプレートの準備完了</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="コメントコード用テンプレートの準備完了" />
</div>

準備ができたら、FastComments.com ウィジェットコードをコピーします：

[inline-code-attrs-start title = 'Ghost FastComments.com コメントコードスニペット'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...すると次のように表示されます：

<div class="screenshot white-bg">
    <div class="title">FastComments.com コメントコードを追加</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="FastComments.com コメントコードを追加" />
</div>

コーディングは完了です。次はテーマを再インポートするだけです！

---