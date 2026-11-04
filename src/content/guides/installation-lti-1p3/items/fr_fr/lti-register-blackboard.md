Blackboard Learn SaaS et Ultra prennent en charge l'enregistrement dynamique LTI 1.3.

#### Ouvrir l'écran des fournisseurs d'outils

1. Connectez-vous à Blackboard en tant qu'administrateur système.
2. Accédez à **Administrator Panel** > **Integrations** > **LTI Tool Providers**.
3. Cliquez sur **Register LTI 1.3 / LTI Advantage Tool**.

Si vous ne voyez que « Register LTI 1.1 Provider », votre version de Blackboard ne prend pas encore en charge LTI 1.3 - effectuez une mise à niveau ou contactez le support Blackboard.

#### Coller l'URL

Collez l'URL d'enregistrement de FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenez-la ici</a>) dans le champ **Client ID** / **Registration URL** (le libellé de Blackboard varie selon la version). Validez.

Blackboard effectue la poignée de main d'enregistrement avec FastComments et affiche un écran de confirmation.

#### Approuver et activer

Blackboard marque par défaut les outils nouvellement enregistrés comme **Approved but excluded** :

1. Trouvez l'entrée FastComments dans la liste des fournisseurs d'outils.
2. Ouvrez le menu et choisissez **Edit**.
3. Définissez **Tool Status** sur **Approved**.
4. Sous **Institution Policies**, vérifiez les données utilisateur envoyées (nom, e-mail, rôle). Enregistrez.

L'outil est désormais disponible pour les enseignants lorsqu'ils ajoutent du contenu aux cours.