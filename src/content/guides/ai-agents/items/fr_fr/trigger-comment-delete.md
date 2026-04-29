Se déclenche lorsqu'un commentaire est supprimé.

### Contexte reçu par l'agent

- Le commentaire qui vient d'être supprimé - texte, auteur, page.
- Historique optionnel du fil / de l'utilisateur / contexte de la page selon la configuration.

### À noter

- Se déclenche pour les **suppressions dites "soft"** (où le commentaire est masqué mais conservé pour l'audit) et les **suppressions dites "hard"** (où le commentaire est entièrement retiré). Le gestionnaire de déclenchement résout le commentaire depuis le pipeline de suppression en cascade ; ce que l'agent voit est le dernier état connu.
- Une fois qu'un commentaire est complètement supprimé, les outils qui le ciblent (`pin_comment`, `mark_comment_spam`, etc.) sur cet ID de commentaire échoueront.

### Utilisations courantes

- **Transmission d'audit via [Webhooks](#webhooks-overview)** - émettre un événement `trigger.succeeded` afin qu'un système externe enregistre ce qui a été supprimé.
- **Écritures en mémoire** - faire enregistrer par l'agent une [note de mémoire](#tools-overview) concernant un schéma de suppression (le commentaire supprimé était le troisième de l'utilisateur en 24 heures, etc.).
- **Effets entre fils de discussion** - remarquer quand une suppression modifie la structure d'un fil que l'agent a précédemment résumé, et envisager de le résumer à nouveau.

### Remarque relative aux coûts d'exploitation

Si votre site a un volume élevé de suppressions (modération humaine intense), ce déclencheur peut se produire fréquemment.

---