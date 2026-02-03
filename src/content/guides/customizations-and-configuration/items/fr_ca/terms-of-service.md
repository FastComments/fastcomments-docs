---
FastComments vous permet d'exiger que les personnes commentant pour la première fois acceptent vos Conditions d'utilisation avant de soumettre un commentaire.

When enabled:
- **Utilisateurs anonymes** verront une case à cocher TOS à chaque fois qu'ils commentent
- **Utilisateurs authentifiés** verront la case à cocher uniquement sur leur premier commentaire, ou lorsque vous mettez à jour vos TOS

### Configuration

Accédez à la page de personnalisation du widget et activez la case à cocher "Require Terms of Service acceptance". Une fois activée, vous verrez les options suivantes :

[app-screenshot-start url='/auth/my-account/customize-widget/new'; clickSelector = '.tos-enabled'; selector = '#tos-options'; title='Terms of Service Options' app-screenshot-end]

- **TOS Text Mode** : Par défaut, la case affiche "I agree to the Terms of Service and Privacy Policy" avec des liens vers les deux documents. Sélectionnez "Customize text per locale" pour fournir votre propre texte pour chaque langue.
- **TOS Last Updated Date** : Lorsque vous mettez à jour vos Conditions d'utilisation, définissez cette date. Les utilisateurs qui ont accepté avant cette date devront accepter à nouveau.

### How It Works

- L'horodatage d'acceptation des TOS est stocké par utilisateur et par commentaire
- Lorsqu'un utilisateur accepte les TOS, la date est enregistrée dans son profil utilisateur (par locataire)
- Si vous définissez une date de "Last Updated" qui est postérieure à la date d'acceptation de l'utilisateur, celui-ci devra accepter de nouveau
- Pour les utilisateurs anonymes qui ne peuvent pas être suivis, la case apparaît à chaque soumission de commentaire

---