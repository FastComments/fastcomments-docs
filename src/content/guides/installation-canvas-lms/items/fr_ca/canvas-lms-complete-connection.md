#### Entrez le Client ID dans FastComments

Retournez à <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">Mon compte > Configuration LTI Canvas</a> dans FastComments. L'assistant devrait être à **Étape 2 : Connexion**.

Collez le **Client ID** que vous avez copié depuis Canvas dans le champ **Client ID**. Saisissez éventuellement le **Deployment ID** si votre LMS en fournit un.

Cliquez sur **Enregistrer et continuer**.

#### Activer l’intégration

L'assistant passe à **Étape 3 : Mise en production**. Un récapitulatif de votre configuration s'affiche (nom, URL de la plateforme, Client ID, et deployment ID).

Vérifiez les détails, puis cliquez sur **Activer l'intégration** pour activer la connexion LTI.

Après l'activation, l'assistant affiche la **vue de gestion** où vous pouvez modifier votre configuration, consulter toutes les URL LTI ou ajouter des déploiements supplémentaires.

#### Installer l’application externe dans Canvas

Dans Canvas, allez à **Administration** > sélectionnez votre compte > **Paramètres** > onglet **Applications**.

Cliquez sur **+ Application** et configurez :

1. Réglez **Type de configuration** sur **Par Client ID**.
2. Collez le **Client ID** provenant du tableau Clés du développeur.
3. Cliquez sur **Envoyer**.
4. Confirmez l'installation lorsqu'on vous le demande.

FastComments est maintenant installé au niveau du compte et accessible à tous les cours.