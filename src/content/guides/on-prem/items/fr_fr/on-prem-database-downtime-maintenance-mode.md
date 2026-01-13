---
FastComments prend en charge un mode de maintenance automatique. Si la base de données tombe en panne, il peut continuer à servir les fils de commentaires populaires.

De plus, en mode maintenance, tous les commentaires sont enregistrés dans `BACKUP_DIR`. Ils seront traités (contrôlés pour le spam, etc.) et sauvegardés une fois le système de nouveau en ligne.

Il le fait en déterminant, toutes les heures, les 100 fils de commentaires les plus populaires et en mettant en cache leur contenu sur disque. La détermination des 100 meilleurs fils
est déjà effectuée à partir d'un état pré-calculé, donc ce n'est pas un travail périodique lourd.

Ceci est complètement optionnel, et n'est activé que si `CACHE_DIR` et `BACKUP_DIR` sont définis. Cela rend bien sûr les nœuds de l'application avec état, cependant il s'agit d'un état qui peut être perdu à tout moment sans provoquer de dysfonctionnement de l'application.

Notez qu'en mode maintenance, l'authentification correcte des fils de commentaires ne peut pas être effectuée, donc seuls les fils considérés comme étant publics en toute sécurité sont sauvegardés périodiquement.

En mode maintenance, de nombreuses fonctionnalités ne sont pas disponibles.

---