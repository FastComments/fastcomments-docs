FastComments vous permet d'exiger que les personnes commentant pour la première fois acceptent vos Conditions d'utilisation avant de soumettre un commentaire.

Lorsque cette option est activée :
- **Utilisateurs anonymes** verront une case à cocher des Conditions d'utilisation à chaque commentaire
- **Utilisateurs authentifiés** verront la case à cocher uniquement sur leur premier commentaire, ou lorsque vous mettez à jour vos Conditions d'utilisation

### Configuration

Accédez à la page de personnalisation du widget et activez la case à cocher « Exiger l'acceptation des Conditions d'utilisation ». Une fois activée, vous verrez les options suivantes :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **Mode du texte des Conditions d'utilisation** : Par défaut, la case à cocher affiche « J'accepte les Conditions d'utilisation et la Politique de confidentialité » avec des liens vers les deux documents. Sélectionnez « Personnaliser le texte par langue » pour fournir votre propre texte pour chaque langue.
- **Date de dernière mise à jour des Conditions d'utilisation** : Lorsque vous mettez à jour vos Conditions d'utilisation, définissez cette date. Les utilisateurs ayant accepté avant cette date devront accepter à nouveau.

### Fonctionnement

- L'horodatage d'acceptation des Conditions d'utilisation est stocké par utilisateur et par commentaire
- Lorsqu'un utilisateur accepte les Conditions d'utilisation, la date est enregistrée sur son profil utilisateur (par locataire)
- Si vous définissez une date de « Dernière mise à jour » postérieure à la date d'acceptation de l'utilisateur, il devra accepter à nouveau
- Pour les utilisateurs anonymes qui ne peuvent pas être suivis, la case à cocher apparaît à chaque soumission de commentaire

---