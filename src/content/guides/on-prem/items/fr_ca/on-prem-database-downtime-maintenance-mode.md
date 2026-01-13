---
FastComments prend en charge un mode de maintenance automatique. Si la base de données est hors service, il peut continuer à fournir les fils de commentaires populaires.

De plus, en mode maintenance, tous les commentaires sont sauvegardés dans `BACKUP_DIR`. Ils seront traités (vérifiés pour le spam, etc.) et enregistrés une fois le système de nouveau en ligne.

Cela fonctionne en déterminant, toutes les heures, les 100 fils de commentaires les plus populaires et en mettant leur contenu en cache sur disque. La détermination des 100 meilleurs fils
est déjà effectuée à partir d'un état pré-calculé, donc ce n'est pas une tâche périodique lourde.

Ceci est complètement optionnel, et n'est activé que si `CACHE_DIR` et `BACKUP_DIR` sont définis. Cela rend bien sûr les nœuds de l'application avec état, cependant il s'agit d'un état
qui peut être perdu à tout moment sans entraîner de dysfonctionnement de l'application.

Notez qu'en mode maintenance, l'authentification correcte des fils de commentaires ne peut pas être effectuée, donc seuls les fils considérés sans risque comme publics sont sauvegardés périodiquement.

En mode maintenance, de nombreuses fonctionnalités ne sont pas disponibles.

---