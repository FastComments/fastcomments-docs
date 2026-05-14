Blackboard Learn SaaS et Ultra prennent en charge l'enregistrement dynamique LTI 1.3.

#### Ouvrir l'écran du fournisseur d'outils

1. Connectez-vous à Blackboard en tant qu'administrateur système.
2. Accédez à **Panneau d'administration** > **Intégrations** > **Fournisseurs d'outils LTI**.
3. Cliquez sur **Register LTI 1.3 / LTI Advantage Tool**.

If you only see "Register LTI 1.1 Provider", your Blackboard version doesn't support LTI 1.3 yet - upgrade or contact Blackboard support.

#### Coller l'URL

Collez l'URL d'enregistrement FastComments dans le champ **Client ID** / **Registration URL** (la dénomination varie selon la version de Blackboard). Soumettez.

Blackboard effectue l'échange d'enregistrement (handshake) avec FastComments et affiche un écran de confirmation.

#### Approuver et activer

Par défaut, Blackboard marque les outils nouvellement enregistrés comme **Approved but excluded** :

1. Trouvez l'entrée FastComments dans la liste des fournisseurs d'outils.
2. Ouvrez le menu et choisissez **Edit**.
3. Définissez **Tool Status** sur **Approved**.
4. Sous **Institution Policies**, vérifiez les données utilisateur envoyées (nom, email, rôle). Enregistrez.

L'outil est désormais disponible pour les enseignants lorsqu'ils ajoutent du contenu aux cours.