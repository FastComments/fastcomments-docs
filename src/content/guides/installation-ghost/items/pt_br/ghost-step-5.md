A seguir, precisamos identificar onde adicionar o código do widget do FastComments.com.

Se você estiver usando o tema padrão `casper`, verá uma seção como esta na linha `82`:

<div class="screenshot white-bg">
    <div class="title">Seção de Comentários Desativada</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Seção de Comentários Desativada" />
</div>

Se você estiver usando outros temas, não verá isso, e precisará adicionar esse código após o último `</section>`:

[inline-code-attrs-start title = 'Exemplo de Seção'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Você deve ter algo assim pronto:

<div class="screenshot white-bg">
    <div class="title">Template Pronto Para Código de Comentários</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Template Pronto Para Código de Comentários" />
</div>

Uma vez pronto, copie o código do widget do FastComments.com:

[inline-code-attrs-start title = 'Trecho de código de comentário do FastComments.com para Ghost'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...e deve ficar assim:

<div class="screenshot white-bg">
    <div class="title">Adicionar Código de Comentários do FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Adicionar Código de Comentários do FastComments.com" />
</div>

Código pronto. Agora só precisamos reimportar nosso tema!