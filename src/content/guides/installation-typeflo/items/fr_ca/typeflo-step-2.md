Dans la section **Pied de page** de l'onglet Code personnalisé, collez le code suivant :

[inline-code-attrs-start title = 'Extrait de code de commentaires en direct Typeflo.io'; type = 'html'; isFunctional = false; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js" async></script>
<script>
    (function () {
        function load() {
            let target = null;
            let lastInstance;
            if (document.querySelector('.fastcomments-widget')) {
                setTimeout(load, 1000);
                return;
            }
            if (lastInstance) {
                lastInstance.destroy();
            }
            if (window.FastCommentsUI) {
                const newElement = document.createElement('div');
                newElement.classList.add('fastcomments-widget');
                const subscribeSection = document.querySelector('.nc-SectionSubscribe2');
                if (subscribeSection) {
                    subscribeSection.parentNode.insertBefore(newElement, subscribeSection);
                    target = newElement;
                } else {
                    const fullWidthSection = document.querySelector('.container.w-full');
                    if (fullWidthSection) {
                        fullWidthSection.prepend(newElement);
                        target = newElement;
                    }
                }
            }
            if (target) {
                lastInstance = FastCommentsUI(target, {
                    "tenantId": "demo"
                });
            }
            setTimeout(load, 1000);
        }

        load();
    })();
</script>
[inline-code-end]

<div class="screenshot white-bg">
    <div class="title">Collez le code dans la section Pied de page</div>
    <img class="screenshot-image" src="/images/installation-guides/typeflo-step-3-paste-code-in-footer-section.png" alt="Collez le code FastComments dans la section Pied de page" />
</div>

Après avoir collé le code, cliquez sur le bouton **Enregistrer** pour appliquer vos modifications.

Note : ce code inclut une logique pour placer dynamiquement le widget FastComments à l'emplacement optimal dans vos articles de blog Typeflo.io. Les autres extraits de code ne fonctionneront pas correctement avec la mise en page de Typeflo.io.

N'oubliez pas de remplacer 'demo' par votre ID de locataire FastComments réel après votre inscription. Si vous êtes connecté à FastComments.com, il devrait déjà être remplacé.