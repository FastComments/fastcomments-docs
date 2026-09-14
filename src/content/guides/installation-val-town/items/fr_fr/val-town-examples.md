---
Quatre vals publics que vous pouvez remixer, chacun couvrant une partie de ce guide.

**[Blog avec commentaires](https://www.val.town/x/fastcomments/blog-with-comments)** ([en direct](https://fastcomments-blog.val.run)) est un blog Markdown avec un fil de discussion sous chaque article et des comptes de commentaires en masse sur l'index. Il fonctionne dès que vous le remixer, et une variable d'environnement le pointe vers votre propre compte.

**[Démo SSO](https://www.val.town/x/fastcomments/sso-demo)** ([en direct](https://fastcomments-sso.val.run)) connecte le visiteur avec son compte Val Town et transmet cette identité au widget, de sorte qu'il n'y ait pas de deuxième connexion.

**[Récepteur de webhook](https://www.val.town/x/fastcomments/webhook-receiver)** ([en direct](https://fastcomments-webhooks.val.run)) vérifie la signature HMAC à chaque livraison et stocke les événements dans SQLite. Il possède un bouton qui signe une charge utile de test et la délivre à lui-même, vous permettant de voir la vérification réussir avant de configurer un vrai webhook.

**[Compétences d'agent](https://www.val.town/x/fastcomments/skills)** ([en direct](https://fastcomments-skills.val.run)) est une bibliothèque de compétences d'agent FastComments couvrant le widget, le SSO, l'API REST, la modération et la migration depuis Disqus. Remixez-le et l'agent de Val Town, Townie, récupère automatiquement les compétences dans `skills/`, de sorte que votre agent sache comment configurer les commentaires sans que vous ayez à coller la documentation dans le chat.

Les mêmes compétences s'installent partout ailleurs avec `npx skills add fastcomments/skills`.
---