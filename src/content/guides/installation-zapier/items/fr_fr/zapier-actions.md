## Actions et recherches

Les actions créent des données dans FastComments ; les recherches recherchent des données afin qu’une étape ultérieure puisse les utiliser. Chaque action appelle l’API REST de FastComments et consomme le même nombre de crédits API que l’appel le coûterait depuis votre propre code : un crédit par appel, sauf indication contraire.

## Créer un commentaire

Publie un commentaire sur une page.

| Champ | Obligatoire | Remarques |
|-------|-------------|-----------|
| Page URL ID | Yes | L'ID d'URL utilisé par le widget de commentaire sur la page. Les commentaires sont regroupés par cet ID. |
| Page URL | Yes | L'URL complète de la page, utilisée dans les e‑mails de notification. |
| Comment | Yes | Le corps du commentaire en markdown FastComments. |
| Commenter Name | Yes | Les noms sont uniques par e‑mail, donc réutiliser un nom avec un e‑mail différent échoue. |
| Commenter Email | No | Un utilisateur est créé pour l'e‑mail lorsqu'il n'existe pas encore. |
| User ID | No | Un ID d'utilisateur SSO existant. Il prend le pas sur le nom et l'e‑mail. |
| Parent Comment ID | No | Définir pour publier une réponse. |
| Approved, Verified | No | Les deux sont vrais par défaut. Les commentaires non approuvés restent cachés jusqu'à modération. |
| Posted At | No | Par défaut, la date actuelle. |
| Avatar URL, Page Title, Locale | No | La locale est `en_us` par défaut. |
| Show Live In Widget | No | Envoie le commentaire aux spectateurs en temps réel. Coûte 2 crédits au lieu de 1. |
| Run Spam Check, Send Emails | No | Désactivé par défaut. |

## Créer ou mettre à jour une page

Crée un enregistrement de page avant qu'aucun commentaire n'existe dessus, afin qu'elle puisse être listée et restreinte. Prend l'ID d'URL, le titre, l'URL, et éventuellement les IDs de groupe SSO autorisés à la voir. Si une page avec cet ID d'URL existe déjà, elle est mise à jour avec les champs fournis, de sorte qu'un Zap puisse s'exécuter plusieurs fois pour la même page.

## Créer ou mettre à jour un utilisateur SSO

Crée un utilisateur d'authentification unique (SSO). Prend votre propre ID d'utilisateur, nom d'utilisateur et e‑mail, ainsi que, en option, le nom d'affichage, le libellé d'affichage, l'avatar, le site web, les IDs de groupe, ainsi que les indicateurs de notification et de confidentialité. Si un utilisateur avec cet ID existe déjà, il est mis à jour à la place. Les rôles administratifs ne peuvent pas être accordés depuis Zapier.

## Créer un article de flux

Crée un article dans un flux FastComments à partir de contenu HTML. L'ID d'utilisateur de l'auteur est requis (un ID FastComments ou SSO) ; le titre, les tags et un aperçu de lien sont optionnels.

## Créer ou mettre à jour un hashtag

Crée un hashtag que les commentateurs peuvent utiliser, avec une URL optionnelle vers laquelle il pointe. Si le hashtag existe déjà, il est mis à jour à la place.

## Signaler un commentaire

Signale un commentaire pour révision par un modérateur. L'ID de l'utilisateur qui signale est requis ; l'ID de l'auteur renvoyé par Créer un commentaire fonctionne.

## Recherches

| Recherche | Entrée | Retour |
|-----------|--------|--------|
| Find Comment | Comment ID | Le commentaire, ou rien. |
| Find SSO User | Email | L'utilisateur SSO, ou rien. |
| Find Page | URL ID | La page, ou rien. |

Une recherche qui ne trouve rien ne fait pas échouer le Zap. Find SSO User et Find Page offrent l'option de Zapier "create if it doesn't exist", qui exécute la création correspondante lorsqu'aucun résultat n'est trouvé.

---