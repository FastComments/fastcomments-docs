Şimdi FastComments.com widget kodunu nereye ekleyeceğimizi belirlememiz gerekiyor.

Varsayılan `casper` temasını kullanıyorsanız, `82` satırında şuna benzer bir bölüm göreceksiniz:

<div class="screenshot white-bg">
    <div class="title">Devre Dışı Yorum Bölümü</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Devre Dışı Yorum Bölümü" />
</div>

Diğer temaları kullanıyorsanız bunu görmeyeceksiniz ve bu kodu son `</section>` etiketinden sonra eklemeniz gerekecek:

[inline-code-attrs-start title = 'Bölüm Örneği'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Şuna benzer bir şey hazır olmalı:

<div class="screenshot white-bg">
    <div class="title">Yorum Kodu İçin Hazır Şablon</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Yorum Kodu İçin Hazır Şablon" />
</div>

Hazır olduğunuzda FastComments.com widget kodunu kopyalayın:

[inline-code-attrs-start title = 'Ghost FastComments.com Yorum Kod Parçacığı'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...ve şöyle görünmeli:

<div class="screenshot white-bg">
    <div class="title">FastComments.com Yorum Kodunu Ekle</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="FastComments.com Yorum Kodunu Ekle" />
</div>

Kodlama tamam. Şimdi sadece temamızı yeniden içe aktarmamız gerekiyor!