Next we need to identify where to add the FastComments.com widget code.

If you're using the default `casper` theme, you'll see a section like this at line `82`:

<div class="screenshot white-bg">
    <div class="title">Devre Dışı Yorum Bölümü</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Devre Dışı Yorum Bölümü" />
</div>

If you're using other themes, you won't see this, and will need to add this code after the last `</section>`:

[inline-code-attrs-start title = 'Bölüm Örneği'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

You should have something like this ready:

<div class="screenshot white-bg">
    <div class="title">Yorum Kodu İçin Hazır Şablon</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Yorum Kodu İçin Hazır Şablon" />
</div>

Once ready, copy the FastComments.com widget code:

[inline-code-attrs-start title = 'Ghost FastComments.com Yorum Kod Kesiti'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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
    <div class="title">FastComments.com Yorum Kodunu Ekle</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="FastComments.com Yorum Kodunu Ekle" />
</div>

Coding done. Now we just have to re-import our theme!