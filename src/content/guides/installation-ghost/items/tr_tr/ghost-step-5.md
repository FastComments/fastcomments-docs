Sonraki adım, FastComments.com widget kodunu nereye ekleyeceğimizi belirlemektir.

Varsayılan `casper` temasını kullanıyorsanız, `82` satırında şu şekilde bir bölüm görürsünüz:

<div class="screenshot white-bg">
    <div class="title">Devre Dışı Yorum Bölümü</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Devre Dışı Yorum Bölümü" />
</div>

Diğer temaları kullanıyorsanız, bunu görmezsiniz ve bu kodu son `</section>` etiketinden sonra eklemeniz gerekir:

[inline-code-attrs-start title = 'Bölüm Örneği'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Elinizde şu şekilde bir şey hazır olmalıdır:

<div class="screenshot white-bg">
    <div class="title">Yorum Kodu İçin Şablon Hazır</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Yorum Kodu İçin Şablon Hazır" />
</div>

Hazır olduğunda, FastComments.com widget kodunu kopyalayın:

[inline-code-attrs-start title = 'Ghost FastComments.com Yorum Kod Parçası'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
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

        FastCommentsUI(document.getElementById('fastcomments-widget'), {
            tenantId: "demo",
            urlId: window.location.pathname,
            allowAnon: false,
            simpleSSO: simpleSSO
        });
    })();
</script>
[inline-code-end]

...ve şöyle görünmelidir:

<div class="screenshot white-bg">
    <div class="title">FastComments.com Yorum Kodunu Ekle</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="FastComments.com Yorum Kodunu Ekle" />
</div>

Kodlama tamam. Şimdi temanızı yeniden içe aktarmamız gerekiyor!