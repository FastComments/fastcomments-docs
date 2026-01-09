[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Pour les sites qui permettent de basculer en mode sombre après le chargement initial de la page, c'est un peu plus complexe.

D'abord, toutes les versions actuelles de la bibliothèque du widget de commentaires (React, Vue) contiennent des exemples de basculement en mode sombre dans leurs dépôts respectifs.

Pour le widget VanillaJS, il faudra faire un peu plus de travail. Premièrement, FastCommentsUI renvoie un objet avec les fonctions "destroy" et "update".

Nous pouvons simplement appeler la fonction update chaque fois que nous voulons mettre à jour la configuration du widget de commentaires, comme suit. Voici un exemple complet et fonctionnel de basculement
du mode sombre avec le widget VanillaJS.

[inline-code-attrs-start title = 'Exemple complet : bascule du mode sombre'; inline-code-attrs-end]
[inline-code-start]
<script src="https://fastcomments.com/js/embed-v2.min.js"></script>
<button id="toggle-dark-mode">Toggle Dark Mode</button>
<div id="fastcomments-widget"></div>
<script>
    (function() {
        const button = document.getElementById('toggle-dark-mode');
        const config = {
            tenantId: 'demo',
            hasDarkBackground: false
        };
        const instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);
        button.addEventListener('click', function() {
            config.hasDarkBackground = !config.hasDarkBackground;
            if (config.hasDarkBackground) {
                document.body.classList.add('dark');
            } else {
                document.body.classList.remove('dark');
            }
            instance.update(config);
        });
    })();
</script>
<style>
    body.dark {
        background: #000;
        color: #fff;
    }
</style>
[inline-code-end]

---