---
FastComments vous permet d'exiger que les commentateurs qui commentent pour la première fois acceptent vos Conditions d\'utilisation avant de soumettre un commentaire.

Lorsqu\'il est activé :
- **Utilisateurs anonymes** verront une case à cocher des CGU à chaque fois qu\'ils commentent
- **Utilisateurs authentifiés** ne verront la case à cocher que lors de leur premier commentaire, ou lorsque vous mettez à jour vos CGU

### Configuration

Accédez à la page de personnalisation du widget et activez la case à cocher « Exiger l\'acceptation des Conditions d\'utilisation ». Une fois activée, vous verrez les options suivantes :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; alt='Panneau des Conditions d\'utilisation affichant le sélecteur de mode de texte des CGU et le champ de date de dernière mise à jour'; title='Options des Conditions d\'utilisation' app-screenshot-end]

- **Mode de texte des CGU** : Par défaut, la case à cocher affiche « J\'accepte les Conditions d\'utilisation et la Politique de confidentialité » avec des liens vers les deux documents. Sélectionnez « Personnaliser le texte par langue » pour fournir votre propre texte pour chaque langue.
- **Date de dernière mise à jour des CGU** : Lorsque vous mettez à jour vos Conditions d\'utilisation, définissez cette date. Les utilisateurs qui ont accepté avant cette date devront accepter à nouveau.

### Fonctionnement

- L\'horodatage d\'acceptation des CGU est stocké par utilisateur et par commentaire
- Lorsqu\'un utilisateur accepte les CGU, la date est enregistrée sur son profil utilisateur (par locataire)
- Si vous définissez une date de « Dernière mise à jour » postérieure à la date d\'acceptation de l\'utilisateur, ils devront réaccepter
- Pour les utilisateurs anonymes qui ne peuvent pas être suivis, la case à cocher apparaît à chaque soumission de commentaire
---