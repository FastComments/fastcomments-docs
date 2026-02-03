FastComments vous permet d'exiger que les commentateurs pour la première fois acceptent vos Conditions d'utilisation avant de soumettre un commentaire.

When enabled:
- **Les utilisateurs anonymes** verront une case à cocher des Conditions d'utilisation à chaque fois qu'ils commentent
- **Les utilisateurs authentifiés** verront la case à cocher uniquement lors de leur premier commentaire, ou lorsque vous mettez à jour vos Conditions d'utilisation

### Enabling Terms of Service

Accédez à la page de personnalisation du widget et activez la case à cocher "Exiger l'acceptation des Conditions d'utilisation" :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-enabled'; title='Enable Terms of Service Checkbox' app-screenshot-end]

### Customizing the TOS Text

Par défaut, la case affiche "J'accepte les Conditions d'utilisation et la Politique de confidentialité" avec des liens vers les deux documents. Vous pouvez personnaliser ce texte par locale si nécessaire :

1. Sélectionnez "Personnaliser le texte par locale"
2. Sélectionnez la locale dans le menu déroulant et saisissez votre texte personnalisé

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-text-mode'; title='Customize TOS Text' app-screenshot-end]

### Updating Your Terms of Service

Lorsque vous mettez à jour vos Conditions d'utilisation, définissez la date "Dernière mise à jour". Les utilisateurs qui ont accepté les Conditions d'utilisation avant cette date devront accepter à nouveau :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.tos-last-updated'; title='TOS Last Updated Date' app-screenshot-end]

### How It Works

- L'horodatage d'acceptation des Conditions d'utilisation est stocké par utilisateur et par commentaire
- Lorsqu'un utilisateur accepte les Conditions d'utilisation, la date est enregistrée sur son profil utilisateur (par locataire)
- Si vous définissez une date "Dernière mise à jour" postérieure à la date d'acceptation de l'utilisateur, il devra accepter à nouveau
- Pour les utilisateurs anonymes qui ne peuvent pas être suivis, la case apparaît à chaque soumission de commentaire