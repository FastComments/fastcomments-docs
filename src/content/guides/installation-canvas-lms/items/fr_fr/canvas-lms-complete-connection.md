#### Saisissez le Client ID dans FastComments

Retournez à <a href="https://fastcomments.com/auth/my-account/canvas-lti-config" target="_blank">votre configuration LTI FastComments</a>. L'assistant devrait être sur **Étape 2 : Connexion**.

Collez le **Client ID** que vous avez copié depuis Canvas dans le champ **Client ID**. Saisissez éventuellement le **Deployment ID** si votre LMS en fournit un.

Cliquez sur **Enregistrer et continuer**.

#### Activez l'intégration

L'assistant passe à **Étape 3 : Passer en production**. Un résumé de votre configuration est affiché (nom, URL de la plateforme, Client ID et deployment ID).

Vérifiez les détails, puis cliquez sur **Enable Integration** pour activer la connexion LTI.

Après activation, l'assistant affiche la **Vue de gestion** où vous pouvez modifier votre configuration, voir toutes les URL LTI ou ajouter des déploiements supplémentaires.

#### Installez l'application externe dans Canvas

Dans Canvas, allez dans **Admin** > sélectionnez votre compte > **Paramètres** > onglet **Applications**.

Cliquez sur **+ App** et configurez :

1. Définissez **Type de configuration** sur **Par Client ID**.
2. Collez le **Client ID** depuis le tableau Clés développeur.
3. Cliquez sur **Soumettre**.
4. Confirmez l'installation lorsqu'on vous le demande.

FastComments est maintenant installé au niveau du compte et disponible pour tous les cours.