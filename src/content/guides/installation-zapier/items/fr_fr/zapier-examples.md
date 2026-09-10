## Exemples de Zaps

Quelques flux de travail qui ne prennent que quelques minutes à configurer.

**Recevez une notification pour les nouveaux commentaires.** New Comment, then Slack "Send Channel Message" or Discord "Send
Channel Message". Mappez le nom du commentateur, le texte du commentaire et l'URL de la page dans le message. Ajoutez le
filtre de domaine pour notifier un canal différent par site.

**Conservez un journal de chaque commentaire.** New Comment, then Google Sheets "Create Spreadsheet Row". Ajoutez Deleted
Comment comme un deuxième Zap qui ajoute une ligne avec l'ID du commentaire, de sorte que la feuille serve également de piste d'audit.

**Envoyez un e‑mail à l'auteur lorsqu'un commentaire est approuvé.** Updated Comment with a Zapier filter on Approved is true,
then Gmail "Send Email". Parce que Updated Comment se déclenche à chaque modification, le filtre est ce qui fait que ce Zap
réagit uniquement aux approbations.

**Ajoutez les commentateurs à votre CRM ou liste de diffusion.** New Comment, then HubSpot "Create or Update Contact" or
Mailchimp "Add or Update Subscriber" en utilisant l'e‑mail du commentateur. Respectez votre politique de confidentialité et la législation locale
avant d'ajouter quiconque à une liste marketing.

**Créez un commentaire à partir d'un formulaire.** Typeform ou Google Forms "New Response", then FastComments Create Comment
avec l'ID d'URL de page que votre site utilise pour les témoignages. Laissez Approved décoché pour examiner chaque commentaire avant qu'il
n'apparaisse.

**Publiez des annonces dans un flux.** RSS by Zapier "New Item in Feed", then Create Feed Post avec le titre, le contenu et le lien de l'élément.

**Provisionnez les membres en tant qu'utilisateurs SSO.** Memberstack, Memberful, ou votre propre webhook, then Find SSO User suivi
par Create SSO User en mode "find or create".

**Escaladez les commentaires signalés.** Updated Comment, filtré sur un nombre de drapeaux supérieur à zéro, then Trello "Create
Card" ou Linear "Create Issue" avec l'ID du commentaire et un lien vers la page de modération.

**Publiez les pages dès qu'elles sont en ligne.** WordPress ou Ghost "New Post", then Create Page avec l'URL de l'article, de sorte que la
page soit répertoriée et restreinte avant le premier commentaire.

**Archivez les commentaires supprimés.** Deleted Comment, then Airtable "Create Record" avec le commentaire complet pour
la rétention conforme.