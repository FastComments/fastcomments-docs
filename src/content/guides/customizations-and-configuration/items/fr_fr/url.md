[related-parameter-start name = 'url'; type = 'string'; related-parameter-end]

Lors de l'envoi d'e-mails de notification, ou du rendu des commentaires dans des interfaces utilisateur comme la page de modération, il est utile de pouvoir faire un lien
à partir du commentaire vers la page sur laquelle il se trouve.

Si URL ID n'est pas toujours un ID, alors nous devons stocker l'URL ailleurs. C'est à cela que sert la propriété "url", définie comme suit.

[code-example-start config = {url: 'https://example.com/article-5'}; linesToHighlight = [6]; title = 'Définir une URL personnalisée'; code-example-end]

Un cas d'utilisation courant est d'associer le fil de commentaires à un identifiant, comme un article, puis de créer un lien vers une page particulière, par exemple :

[code-example-start config = {url: 'https://example.com/article-5', urlId: 'article-5'}; linesToHighlight = [6, 7]; title = 'Définir ensemble l\'URL personnalisée et les ID d\'URL'; code-example-end]

L'URL n'est pas nettoyée des paramètres marketing courants. Par défaut, quelle que soit l'URL de la page actuelle, c'est l'URL qui est enregistrée avec le commentaire.