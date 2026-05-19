D2L Brightspace propose l'enregistrement dynamique via l'interface d'administration LTI Advantage. Vous aurez besoin d'un accès administrateur.

#### Ouvrir l'écran d'enregistrement

1. Connectez-vous à votre instance Brightspace en tant qu'administrateur.
2. Accédez à **Outils d'administration** > **Gérer l'extensibilité** > **LTI Advantage**.
3. Cliquez sur **Register Tool**. (L'URL directe est `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Coller l'URL

Vous verrez un formulaire d'enregistrement. Le champ clé est **Tool initiation registration endpoint** (certaines versions de Brightspace l'étiquettent "Tool Initiation Registration URL").

Collez l'URL d'enregistrement FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenez-le ici</a>) dans ce champ. Laissez les autres champs vides - ils sont remplis automatiquement par FastComments lors de la transaction d'enregistrement.

Cliquez sur **Register**.

#### Approuver l'outil

Brightspace ouvre une fenêtre contextuelle qui communique avec FastComments, échange les clés et affiche un écran de confirmation. La fenêtre se ferme automatiquement lorsque l'enregistrement est terminé.

Le nouvel outil apparaît dans votre liste d'outils LTI Advantage. Par défaut, Brightspace marque les nouveaux outils comme **disabled** - basculez l'interrupteur sur **enabled** pour que vos cours puissent l'utiliser.

#### Ajouter un déploiement

Dans Brightspace, les outils LTI nécessitent un **déploiement** avant de pouvoir être utilisés dans les cours :

1. Ouvrez l'outil FastComments nouvellement enregistré.
2. Cliquez sur **View Deployments** > **New Deployment**.
3. Donnez un nom au déploiement (par ex. "FastComments - All Courses"), choisissez les unités organisationnelles dans lesquelles il doit être disponible, puis enregistrez.

Après le premier lancement via ce déploiement, FastComments attache le `deployment_id` à son enregistrement de configuration - les lancements suivants depuis un autre déploiement sous le même client seront refusés à moins que vous ne vous réenregistriez.