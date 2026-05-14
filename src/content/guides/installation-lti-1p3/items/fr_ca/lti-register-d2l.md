D2L Brightspace offre l’enregistrement dynamique via l’interface d’administration LTI Advantage. Vous aurez besoin d’un accès administrateur.

#### Ouvrir l'écran d'enregistrement

1. Connectez-vous à votre instance Brightspace en tant qu'administrateur.
2. Accédez à **Admin Tools** > **Manage Extensibility** > **LTI Advantage**.
3. Cliquez sur **Register Tool**. (L'URL directe est `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Collez l'URL

Vous verrez un formulaire d'enregistrement. Le champ clé est **Tool initiation registration endpoint** (certaines versions de Brightspace l'appellent "Tool Initiation Registration URL").

Collez l'URL d'enregistrement FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenez-la ici</a>) dans ce champ. Laissez les autres champs vides — ils sont remplis automatiquement par FastComments pendant la négociation d'enregistrement.

Cliquez sur **Register**.

#### Approuver l'outil

Brightspace ouvre une fenêtre contextuelle qui communique avec FastComments, échange des clés et affiche un écran de confirmation. La fenêtre contextuelle se ferme automatiquement lorsque l'enregistrement est terminé.

Le nouvel outil apparaît dans la liste des outils LTI Advantage. Par défaut, Brightspace marque les nouveaux outils comme **disabled** — basculez le commutateur sur **enabled** pour que vos cours puissent l'utiliser.

#### Ajouter un déploiement

Dans Brightspace, les outils LTI requièrent un **deployment** avant de pouvoir être utilisés dans les cours :

1. Ouvrez l'outil FastComments nouvellement enregistré.
2. Cliquez sur **View Deployments** > **New Deployment**.
3. Donnez un nom au déploiement (p. ex. "FastComments - All Courses"), choisissez les unités organisationnelles où il devrait être disponible, puis enregistrez.

Après le premier lancement via ce déploiement, FastComments attache le `deployment_id` à son enregistrement de configuration — les lancements ultérieurs depuis un autre déploiement sous le même client seront refusés, sauf si vous le réenregistrez.