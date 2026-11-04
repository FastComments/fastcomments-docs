Nous allons maintenant ajouter le code du widget FastComments à votre site. Ce code recherchera tous les formulaires avec le titre `FastComments Goes Here` et
le remplacera par FastComments.

Allons donc dans `Settings` en bas à gauche de l'éditeur de site :

<div class="screenshot white-bg">
    <div class="title">Ouvrir les paramètres</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-settings.png" alt="Ouvrir les paramètres" />
</div>

Ouvrez la section `Custom Head Code` :

<div class="screenshot white-bg">
    <div class="title">Ouvrir Custom Head Code</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-open-custom-head-code.png" alt="Ouvrir Custom Head Code" />
</div>

Pour Ionos, nous avons besoin d'une **version spéciale** du code du widget FastComments. Les extraits de code provenant **d'autres tutoriels ne fonctionneront pas.**

Copiez maintenant le code suivant :

[inline-code-attrs-start title = 'Extrait FastComments pour Ionos'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<script>
    (function () {
        let loaded = false;
        let interval = null;

        function attemptLoad() {
            const nodes = document.querySelectorAll('h2');

            nodes.forEach(function (node) {
                if (node.innerText && node.innerText.trim() === 'FastComments Goes Here') {
                    // we get the element that is not full width
                    const target = node.parentNode.parentNode.parentNode.parentNode.parentNode;
                    target.innerHTML = '';
                    FastCommentsUI(target, {
                        tenantId: "demo"
                    });
                    interval && clearInterval(interval);
                    loaded = true;
                }
            });
        }

        attemptLoad();
        if (!loaded) {
            interval = setInterval(attemptLoad, 300);
        }
    })();
</script>
[inline-code-end]

...et collez-le comme indiqué :

<div class="screenshot white-bg">
    <div class="title">Coller et enregistrer</div>
    <img class="screenshot-image" src="/images/installation-guides/ionos-step-2-paste-and-save.png" alt="Coller et enregistrer" />
</div>