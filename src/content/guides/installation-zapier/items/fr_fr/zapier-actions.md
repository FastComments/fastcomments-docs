## Actions et recherches

Les actions créent des données dans FastComments ; les recherches recherchent des données afin qu’une étape ultérieure puisse les utiliser. Chaque action appelle l’API REST de FastComments et consomme les mêmes crédits API que l’appel coûterait depuis votre propre code : un crédit par appel, sauf indication contraire.

## Créer un commentaire

Posts a comment on a page.

| Champ | Obligatoire | Remarques |
|-------|-------------|-----------|
| ID d’URL de la page | Oui | L’ID d’URL utilisé par le widget de commentaire sur la page. Les commentaires sont regroupés par cet ID. |
| URL de la page | Oui | L’URL complète de la page, utilisée dans les e‑mails de notification. |
| Commentaire | Oui | Le corps du commentaire en markdown FastComments. |
| Nom du commentateur | Oui | Les noms sont uniques par e‑mail, donc réutiliser un nom avec un e‑mail différent échoue. |
| E‑mail du commentateur | Non | Un utilisateur est créé pour cet e‑mail s’il n’existe pas encore. |
| ID d’utilisateur | Non | Un ID d’utilisateur SSO existant. Il prend le pas sur le nom et l’e‑mail. |
| ID du commentaire parent | Non | Définir pour publier une réponse. |
| Approuvé, Vérifié | Non | Les deux sont vrais par défaut. Les commentaires non approuvés restent cachés jusqu’à modération. |
| Date de publication | Non | Par défaut, maintenant. |
| URL d’avatar, Titre de la page, Locale | Non | La locale par défaut est `en_us`. |
| Afficher en direct dans le widget | Non | Envoie le commentaire aux spectateurs en temps réel. Coûte 2 crédits au lieu de 1. |
| Exécuter la vérification anti‑spam, Envoyer des e‑mails | Non | Désactivé par défaut. |

## Créer une page

Crée un enregistrement de page avant qu’aucun commentaire n’existe dessus, afin qu’il puisse être listé et restreint. Prend l’ID d’URL, le titre, l’URL, et éventuellement les IDs de groupe SSO autorisés à le voir.

## Créer un utilisateur SSO

Crée un utilisateur d’authentification unique (SSO). Prend votre propre ID d’utilisateur, nom d’utilisateur et e‑mail, ainsi que, en option, le nom d’affichage, le libellé d’affichage, l’avatar, le site web, les IDs de groupe, ainsi que les indicateurs de notification et de confidentialité. Les rôles administratifs ne peuvent pas être accordés depuis Zapier.

## Créer un article de flux

Crée un article dans un flux FastComments à partir de contenu HTML. L’ID d’utilisateur de l’auteur est requis (un ID d’utilisateur FastComments ou SSO) ; le titre, les tags et un aperçu de lien sont optionnels.

## Créer un hashtag

Crée un hashtag que les commentateurs peuvent utiliser, avec une URL optionnelle vers laquelle il pointe. Les tags sont uniques par compte, donc un Zap qui en crée un à chaque exécution doit contenir quelque chose d’unique dans le tag.

## Signaler un commentaire

Signale un commentaire pour révision par un modérateur. L’ID de l’utilisateur qui signale est requis ; l’ID d’auteur renvoyé par Créer un commentaire fonctionne.

## Recherches

| Recherche | Entrée | Retourne |
|-----------|--------|----------|
| Trouver le commentaire | ID du commentaire | Le commentaire, ou rien. |
| Trouver l’utilisateur SSO | E‑mail | L’utilisateur SSO, ou rien. |
| Trouver la page | ID d’URL | La page, ou rien. |

Une recherche qui ne trouve rien ne fait pas échouer le Zap. Combinez une recherche avec une création dans le mode « trouver ou créer » de Zapier pour créer la page ou l’utilisateur lorsqu’il manque.