---
[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Lors de l'envoi de courriels de notification, ou de l'affichage des commentaires dans des interfaces utilisateur comme la page de modération, il est utile de pouvoir créer un lien
à partir du commentaire vers la page sur laquelle il se trouve.

Si l'ID d'URL n'est pas toujours un identifiant, alors nous devons stocker l'URL ailleurs. C'est à cela que sert la propriété "url", définie comme suit.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Defining a Custom URL'; code-example-end]

Un cas d'utilisation courant consiste à lier le fil de commentaires à un identifiant, comme un article, puis à faire un lien vers une page particulière, par exemple :

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Defining Custom URL and URL IDs together'; code-example-end]

L'URL n'est pas nettoyée des paramètres marketing courants. Par défaut, quelle que soit l'URL de la page actuelle, c'est cette URL qui est enregistrée avec le commentaire.

---