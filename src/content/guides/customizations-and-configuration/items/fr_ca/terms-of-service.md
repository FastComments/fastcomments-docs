FastComments vous permet d'exiger que les commentateurs pour la première fois acceptent vos Conditions d'utilisation avant de soumettre un commentaire.

Une fois activée :
- **Les utilisateurs anonymes** verront une case à cocher des Conditions d'utilisation à chaque fois qu'ils commentent
- **Les utilisateurs authentifiés** verront la case uniquement lors de leur premier commentaire, ou lorsque vous mettez à jour vos Conditions d'utilisation

### Activation des Conditions d'utilisation

Accédez à la page de personnalisation du widget et activez la case "Exiger l'acceptation des Conditions d'utilisation" :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Personnaliser le texte des Conditions d'utilisation

Par défaut, la case affiche "J'accepte les Conditions d'utilisation et la Politique de confidentialité" avec des liens vers les deux documents. Vous pouvez personnaliser ce texte par locale si nécessaire :

1. Sélectionnez "Personnaliser le texte par locale"
2. Sélectionnez la locale dans le menu déroulant et saisissez votre texte personnalisé

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Mise à jour de vos Conditions d'utilisation

Lorsque vous mettez à jour vos Conditions d'utilisation, définissez la date "Date de dernière mise à jour". Les utilisateurs qui ont accepté les Conditions avant cette date devront accepter de nouveau :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### Comment ça fonctionne

- L'horodatage de l'acceptation des Conditions est enregistré par utilisateur et par commentaire
- Lorsqu'un utilisateur accepte les Conditions, la date est enregistrée dans son profil utilisateur (par locataire)
- Si vous définissez une date "Date de dernière mise à jour" qui est postérieure à la date d'acceptation de l'utilisateur, celui-ci devra accepter à nouveau
- Pour les utilisateurs anonymes qui ne peuvent pas être suivis, la case apparaît à chaque soumission de commentaire