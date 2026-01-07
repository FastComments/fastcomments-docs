L'utilisation des Commentaires Programmes FastComments est simple. Tout d'abord, vous voudrez [importer les commentaires ici](https://fastcomments.com/auth/my-account/manage-data/import-scheduled).

Cette page peut egalement etre accessible via `Manage Data->Schedule Comments`.

### Commentaires lus manuellement

*Pour les commentaires* lus manuellement (vous devez appuyer manuellement sur Play), vous avez la possibilite de commencer la lecture des commentaires. Cela lira les commentaires sur chaque page que vous avez definie dans le fichier CSV,
avec les delais entre chaque commentaire bases sur le delai que vous avez specifie.

Ceci est utile lorsque vous avez un webinaire en direct programme qui commence a une heure specifique. Lorsque le webinaire commence, appuyez simplement sur Play
dans le tableau de bord.

### Lecture automatique des commentaires

*Pour les commentaires* lus automatiquement, les commentaires sont lus a chaque chargement de page pour chaque utilisateur.

Ceci est utile pour les scenarios ou les videos ou autre contenu commencent depuis le debut a chaque chargement.

### Lecture a croissance dynamique

Chaque fois que le script de lecture automatique est execute pour un utilisateur - au chargement de la page - il y a encore
l'opportunite pour d'autres de commenter.

Au fur et a mesure que les gens laissent des commentaires, leurs commentaires sont **automatiquement ajoutes au script de
lecture** au meme decalage depuis le chargement de la page, de sorte que la conversation continue de croitre sans
travail manuel.

Vous pouvez en plus **moderer** les commentaires publies, pour selectionner quels commentaires vous voulez afficher
chaque fois que le script de lecture automatique est execute.

La page `Moderate Comments` affichera egalement un horodatage comme `AutoPlay 1hr 2m 30s` a cote de chaque
commentaire au lieu de la date.

Ceci n'est disponible que pour la lecture automatique, pas pour la lecture programmee manuellement.

### Configuration

Chaque commentaire sera publie **en direct**. Vous voudrez peut-etre considerer [l'activation de l'affichage des commentaires en direct immediatement](/guide-customizations-and-configuration.html#show-live-right-away).

Vous pouvez en apprendre davantage sur le format d'importation dans la section Format d'Importation de cette documentation.
