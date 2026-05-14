D2L Brightspace expose l'enregistrement dynamique via l'interface d'administration LTI Advantage. Vous aurez besoin d'un accès administrateur.

#### Ouvrir l'écran d'enregistrement

1. Connectez-vous à votre instance Brightspace en tant qu'administrateur.
2. Allez dans **Outils d'administration** > **Gérer l'extensibilité** > **LTI Advantage**.
3. Cliquez sur **Enregistrer l'outil**. (L'URL directe est `https://<your-brightspace-host>/d2l/le/ltiadvantage/registrations/create`.)

#### Coller l'URL

Vous verrez un formulaire d'enregistrement. Le champ clé est **point de terminaison d'enregistrement pour l'initiation de l'outil** (certaines versions de Brightspace l'appellent "URL d'enregistrement d'initiation de l'outil").

Collez l'URL d'enregistrement FastComments dans ce champ. Laissez les autres champs vides — ils sont remplis automatiquement par FastComments lors de l'échange d'enregistrement.

Cliquez sur **Enregistrer**.

#### Approuver l'outil

Brightspace ouvre une fenêtre contextuelle qui communique avec FastComments, échange des clés et affiche un écran de confirmation. La fenêtre se ferme automatiquement lorsque l'enregistrement est terminé.

Le nouvel outil apparaît dans la liste de vos outils LTI Advantage. Par défaut, Brightspace marque les nouveaux outils comme **désactivés** — basculez l'interrupteur sur **activés** pour que vos cours puissent l'utiliser.

#### Ajouter un déploiement

Dans Brightspace, les outils LTI ont besoin d'un **déploiement** avant de pouvoir être utilisés dans les cours :

1. Ouvrez l'outil FastComments nouvellement enregistré.
2. Cliquez sur **Afficher les déploiements** > **Nouveau déploiement**.
3. Donnez un nom au déploiement (p. ex. "FastComments - Tous les cours"), choisissez les unités organisationnelles dans lesquelles il doit être disponible, puis enregistrez.

Après le premier lancement via ce déploiement, FastComments épingle le `deployment_id` à son enregistrement de configuration - les lancements ultérieurs depuis un autre déploiement sous le même client seront rejetés à moins que vous ne vous réenregistriez.