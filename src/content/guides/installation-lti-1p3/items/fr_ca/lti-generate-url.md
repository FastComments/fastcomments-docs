#### Navigate to LTI 1.3 Configuration

Connectez-vous à FastComments et allez à <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">votre page de configuration LTI 1.3</a>.

Si votre compte n'a pas encore accès à LTI, vous verrez "LTI not enabled for this account" - contactez l'assistance pour l'activer sur votre forfait.

#### Pick a Platform (Optional)

Sous **Generate a Dynamic Registration URL**, utilisez le menu déroulant **Platform** pour indiquer à FastComments à quel LMS vous vous connectez :

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Other LTI 1.3 platform

Vous pouvez aussi laisser sur **Auto-detect**. La plateforme est lue depuis l'openid-configuration de votre LMS lors de l'enregistrement ; le menu déroulant ne sert qu'à définir l'étiquette affichée pour la configuration résultante.

#### Generate the URL

Cliquez sur **Generate URL**. FastComments crée un jeton d'enregistrement à usage unique et vous affiche une URL ressemblant à :

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Copiez-la. Cette URL :

- Est **single-use** - once your LMS calls it successfully, the token is consumed.
- Expire après **30 minutes** si elle n'est pas utilisée.
- Doit rester privée - toute personne disposant de l'URL peut enregistrer un outil sur votre tenant pendant ces 30 minutes.

#### Existing Configurations

Une fois l'enregistrement terminé avec succès, la nouvelle configuration apparaît dans le tableau **Existing Configurations** sur la même page, avec sa Platform, son Issuer, son Client ID, et son Status. Vous pouvez supprimer des configurations depuis ce tableau si vous devez les désenregistrer.