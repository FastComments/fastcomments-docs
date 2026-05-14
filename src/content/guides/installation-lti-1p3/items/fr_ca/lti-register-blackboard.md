Blackboard Learn SaaS and Ultra prennent en charge LTI 1.3 Dynamic Registration.

#### Open the Tool Provider Screen

1. Connectez-vous à Blackboard en tant qu'administrateur système.
2. Accédez à **Administrator Panel** > **Integrations** > **LTI Tool Providers**.
3. Cliquez sur **Register LTI 1.3 / LTI Advantage Tool**.

Si vous ne voyez que "Register LTI 1.1 Provider", votre version de Blackboard ne prend pas encore en charge LTI 1.3 - effectuez une mise à niveau ou contactez le support de Blackboard.

#### Paste the URL

Collez l'URL d'enregistrement FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenez-la ici</a>) dans le champ **Client ID** / **Registration URL** (le libellé varie selon la version de Blackboard). Soumettez.

Blackboard effectue la poignée de main d'enregistrement avec FastComments et vous affiche un écran de confirmation.

#### Approve and Enable

Blackboard marque par défaut les outils nouvellement enregistrés comme **Approved but excluded** :

1. Trouvez l'entrée FastComments dans la liste des fournisseurs d'outils.
2. Ouvrez le menu et choisissez **Edit**.
3. Réglez **Tool Status** sur **Approved**.
4. Sous **Institution Policies**, vérifiez quelles données utilisateur sont envoyées (nom, courriel, rôle). Enregistrez.

L'outil est maintenant disponible pour les enseignants lorsqu'ils ajoutent du contenu aux cours.