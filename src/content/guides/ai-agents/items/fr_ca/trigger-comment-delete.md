Se déclenche lorsqu’un commentaire est supprimé.

### Contexte reçu par l'agent

- Le commentaire qui vient d’être supprimé - texte, auteur, page.
- Contexte optionnel : fil / historique de l’utilisateur / page tel que configuré.

### À noter

- Se déclenche pour les **soft deletes** (lorsque le commentaire est masqué mais conservé pour l'audit) et les **hard deletes** (lorsque le commentaire est entièrement supprimé). Le gestionnaire du déclencheur résout le commentaire depuis le pipeline de suppression en cascade ; ce que l'agent voit est le dernier état connu.
- Une fois qu’un commentaire est complètement supprimé, les outils qui le ciblent (`pin_comment`, `mark_comment_spam`, etc.) pour cet ID de commentaire échoueront.

### Usages courants

- **Transmission d'audit via [Webhooks](#webhooks-overview)** - émettre un événement `trigger.succeeded` pour qu’un système externe enregistre ce qui a été supprimé.
- **Écritures de mémoire** - faire enregistrer par l'agent une [note de mémoire](#tools-overview) concernant un motif de suppression (le commentaire supprimé était le troisième de l'utilisateur en 24 heures, etc.).
- **Effets entre fils** - détecter lorsqu'une suppression modifie la structure d’un fil que l'agent a précédemment résumé, et envisager de le résumer à nouveau.

### Note sur le coût d'exploitation

Si vous gérez un site à fort volume de suppressions (modération humaine intensive), ce déclencheur peut se déclencher fréquemment.