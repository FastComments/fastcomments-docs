Ensuite, il faut identifier l'endroit où ajouter le code du widget FastComments.com.

Si vous utilisez le thème par défaut `casper`, vous verrez une section comme celle-ci à la ligne `82` :

<div class="screenshot white-bg">
    <div class="title">Section de commentaires désactivée</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-1-identify-section.png" alt="Section de commentaires désactivée" />
</div>

Si vous utilisez d'autres thèmes, vous ne verrez pas cela, et devrez ajouter ce code après le dernier `</section>` :

[inline-code-attrs-start title = 'Exemple de section'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<section class="article-comments gh-canvas">
</section>
[inline-code-end]

Vous devriez avoir quelque chose comme ceci prêt :

<div class="screenshot white-bg">
    <div class="title">Modèle prêt pour le code de commentaires</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-2-cleanup.png" alt="Modèle prêt pour le code des commentaires" />
</div>

Une fois prêt, copiez le code du widget FastComments.com :

[inline-code-attrs-start title = 'Extrait de code de commentaire FastComments.com pour Ghost'; type = 'html'; isFunctional = false; inline-code-attrs-end]
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

...et cela devrait ressembler à ceci :

<div class="screenshot white-bg">
    <div class="title">Ajouter le code de commentaire FastComments.com</div>
    <img class="screenshot-image" src="/images/installation-guides/ghost-step-5-3-paste-code.png" alt="Ajouter le code de commentaire FastComments.com" />
</div>

Le codage est terminé. Il ne nous reste plus qu'à réimporter notre thème !