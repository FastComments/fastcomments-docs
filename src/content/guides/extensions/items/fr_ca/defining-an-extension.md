La plus petite extension possible serait:

[inline-code-attrs-start title = 'Une extension simple'; type = 'javascript'; inline-code-attrs-end]
[inline-code-start]
(function () {
    const extension = FastCommentsUI.extensions.find((extension) => {
        return extension.id === 'my-extension';
    });
})();
[inline-code-end]

Pour cet exemple, enregistrez ceci sous `my-extension.js`, et rendez-le disponible à `https://example.com/my-extension.min.js`.

Cette extension ne fait rien, sauf qu'au chargement elle récupère l'objet créé `Extension` par la bibliothèque principale de commentaires.

Cet objet `Extension` est un singleton et n'est pas partagé avec d'autres extensions.

Ensuite, pour charger notre extension, nous devons en informer le widget de commentaires. Par exemple:

[code-example-start config = {extensions: [{id: "my-extension", path: "https://example.com/my-extension.min.js"}]}; linesToHighlight = [6,7,8,9,10,11]; title = 'Using a Custom Extension'; code-example-end]

Pour des exemples fonctionnels, voir la section suivante.