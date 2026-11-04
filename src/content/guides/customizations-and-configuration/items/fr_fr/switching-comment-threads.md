[related-parameter-start name = 'urlId'; type = 'string'; related-parameter-end]

Nous avons vu comment `urlId` est l'identifiant de la page ou de l'article auquel les commentaires sont liés.

Aussi, pour rappel, si non défini, le `urlId` sera par défaut l'URL de la page actuelle.

Mais qu'en est-il des SPAs (Single-Page-Applications), où la page ou le contenu auquel les commentaires sont liés change
dynamiquement sans rechargement de page ?

#### Angular, React, Vue, etc

Avec nos bibliothèques telles que Angular et React, il suffit de mettre à jour la propriété `urlId` passée au widget
pour que le widget de commentaires se rafraîchisse. Vous pouvez voir cela en action pour l'application React, par exemple, <a href="https://github.com/FastComments/fastcomments-react/blob/master/examples/example-paginated/src/PaginatedApp.tsx#L39" target="_blank">ici</a>.

#### VanillaJS

Si vous utilisez la bibliothèque VanillaJS, c'est légèrement plus compliqué car il n'y a pas de framework comme Angular ou React
pour gérer la liaison de données ou la propagation d'état.

Lorsque vous instanciez le widget VanillaJS, il retourne des fonctions qui peuvent être appelées pour le mettre à jour.

Voici un exemple fonctionnel où nous changeons le hash de la page et mettons à jour le widget de commentaires :

[inline-code-attrs-start title = 'Exemple de changement de hash de page'; inline-code-attrs-end]
[inline-code-start]
<script src="https://cdn.fastcomments.com/js/embed-v2.min.js"></script>
<button id="change-page"></button>
<div id="fastcomments-widget"></div>
<script>
    (function fastCommentsMain() {
        let config = {
            tenantId: 'demo'
        };
        let instance = window.FastCommentsUI(document.getElementById('fastcomments-widget'), config);

        let page = '#page-1';
        function getNextPage() {
            if (page === '#page-1') {
                return '#page-2';
            } else {
                return '#page-1';
            }
        }

        let button = document.getElementById('change-page');
        function nextPage() {
            page = getNextPage();
            button.innerText = 'Go to ' + getNextPage();
            window.location.hash = page;
            let locationString = window.location.toString();

            config.url = locationString; // Nous mettons également à jour url pour que les notifications puissent renvoyer vers la bonne page
            config.urlId = locationString;

            instance.update(config);
        }
        nextPage();
        button.addEventListener('click', nextPage);
    })();
</script>
[inline-code-end]