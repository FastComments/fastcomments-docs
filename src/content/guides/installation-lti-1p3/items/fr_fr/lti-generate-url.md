---
#### Accédez à la configuration LTI 1.3

Connectez-vous à FastComments et rendez-vous sur <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">votre page de configuration LTI 1.3</a>.

Si votre compte n'a pas encore accès à LTI, vous verrez "LTI not enabled for this account" - contactez le support pour l'activer sur votre forfait.

#### Choisissez une plateforme (facultatif)

Sous **Générer une URL d'enregistrement dynamique**, utilisez le menu déroulant **Plateforme** pour indiquer à FastComments auquel LMS vous vous connectez :

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Autre plateforme LTI 1.3

Vous pouvez aussi le laisser sur **Auto-detect**. La plateforme est lue à partir de l'openid-configuration de votre LMS lors de l'enregistrement ; le menu déroulant ne sert qu'à définir l'étiquette d'affichage pour la configuration résultante.

#### Générer l'URL

Cliquez sur **Générer l'URL**. FastComments crée un jeton d'enregistrement à usage unique et vous affiche une URL qui ressemble à :

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Copiez-la. Cette URL :

- Est **à usage unique** - une fois que votre LMS appelle l'URL avec succès, le jeton est consommé.
- Expire après **30 minutes** si elle n'est pas utilisée.
- Doit être gardée privée - toute personne disposant de l'URL peut enregistrer un outil contre votre tenant pendant ces 30 minutes.

#### Existing Configurations

Une fois un enregistrement effectué avec succès, la nouvelle configuration apparaît dans le tableau **Existing Configurations** sur la même page, avec ses Platform, Issuer, Client ID, et Status. Vous pouvez supprimer des configurations de ce tableau si vous devez un jour vous désenregistrer.

---