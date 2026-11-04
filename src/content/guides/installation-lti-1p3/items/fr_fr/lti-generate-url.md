#### Accéder à la configuration LTI 1.3

Connectez-vous à FastComments et accédez à <a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">votre page de configuration LTI 1.3</a>.

Si votre compte n'a pas encore accès à LTI, vous verrez « LTI not enabled for this account » - contactez le support pour l'activer sur votre offre.

#### Choisir une plateforme (facultatif)

Sous **Generate a Dynamic Registration URL**, utilisez le menu déroulant **Platform** pour indiquer à FastComments quel LMS vous connectez :

- D2L Brightspace
- Moodle
- Blackboard Learn
- Sakai
- Schoology
- Autre plateforme LTI 1.3

Vous pouvez également laisser **Auto-detect**. La plateforme est lue depuis l'openid-configuration de votre LMS lors de l'enregistrement ; le menu déroulant sert uniquement à préremplir le libellé d'affichage de la configuration résultante.

#### Générer l'URL

Cliquez sur **Generate URL**. FastComments crée un jeton d'enregistrement à usage unique et vous affiche une URL qui ressemble à ceci :

`https://fastcomments.com/lti/v1p3/register/<long-token>`

Copiez-la. Cette URL :

- Est **à usage unique** - une fois que votre LMS l'appelle avec succès, le jeton est consommé.
- Expire après **30 minutes** si elle n'est pas utilisée.
- Doit rester privée - toute personne disposant de l'URL peut enregistrer un outil auprès de votre tenant pendant ces 30 minutes.

#### Configurations existantes

Une fois l'enregistrement terminé avec succès, la nouvelle configuration apparaît dans le tableau **Existing Configurations** sur la même page, avec sa plateforme, son émetteur (Issuer), son Client ID et son statut. Vous pouvez supprimer des configurations depuis ce tableau si vous devez un jour désenregistrer un outil.