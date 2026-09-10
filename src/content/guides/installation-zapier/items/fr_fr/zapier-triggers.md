## Déclencheurs

Les déclencheurs démarrent un Zap lorsqu'un événement se produit dans FastComments. Les trois sont instantanés : FastComments transmet l'événement à Zapier via un webhook dès qu'il se produit. Aucun processus n'interroge votre compte et aucun crédit API n'est dépensé en attente.

| Déclencheur | Se déclenche quand |
|-------------|--------------------|
| Nouveau commentaire | Un commentaire est publié. Par défaut, seuls les commentaires approuvés et non‑spam déclenchent. |
| Commentaire mis à jour | Un commentaire est modifié, approuvé, voté, épinglé, verrouillé ou autrement modifié. |
| Commentaire supprimé | Un commentaire est supprimé. |

Chaque déclencheur renvoie le commentaire complet : id, URL de la page et ID d'URL, nom et e‑mail du commentateur, texte du commentaire en markdown et en HTML, nombre de votes, indicateurs d'approbation et de spam, la langue, le domaine et les mentions éventuelles. Les champs correspondent à la charge utile du webhook documentée sous Webhooks, Structures de données.

## Options

**Domaine.** Chaque déclencheur possède un filtre de domaine optionnel, listant les domaines configurés sur votre compte. Laissez‑le vide pour recevoir les événements de tous les domaines.

**Inclure les commentaires non approuvés et les spams.** Uniquement sur le déclencheur Nouveau commentaire. Les commentaires en attente de modération ou marqués comme spam sont ignorés par défaut. Lorsqu'un tel commentaire est approuvé ultérieurement, le déclencheur Commentaire mis à jour se déclenche pour celui‑ci, de sorte qu'un Zap qui doit réagir à chaque commentaire devenu visible utilise Commentaire mis à jour avec un filtre sur le champ approuvé.

## Fonctionnement de la livraison

Activer un Zap crée une souscription webhook sur votre compte, visible sur la page Webhooks avec la source **API**. Désactiver le Zap la supprime. Les limites propres à Zapier s'appliquent au nombre d'événements qu'il accepte par minute ; FastComments réessaye une livraison qui échoue, avec un délai croissant, et désactive une souscription qui continue d'échouer pendant six jours. Une souscription désactivée peut être réactivée depuis la page Webhooks, ou il suffit de désactiver puis réactiver le Zap pour en créer une nouvelle.

Un compte peut contenir jusqu'à 50 souscriptions API. Chaque Zap utilisant un déclencheur FastComments en utilise une.

---