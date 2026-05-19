**Vous utilisez Moodle ?** Nous publions également un plugin Moodle dédié pour FastComments offrant une intégration plus poussée que LTI 1.3 (hooks de synchronisation des notes, rapports d'activité plus approfondis, interface native de paramètres Moodle). Consultez le <a href="/guide-installation-moodle.html" target="_blank">guide d'installation du plugin Moodle</a>. Le flux LTI 1.3 ci-dessous est le bon choix si vous souhaitez un enregistrement unique couvrant également d'autres LMS, ou si votre administrateur Moodle n'installera pas de plugins tiers.

Moodle 4.0+ prend en charge l'enregistrement dynamique LTI 1.3 via le plugin Outil externe.

#### Ouvrir l'écran de gestion des outils

1. Connectez-vous à Moodle en tant qu'administrateur du site.
2. Accédez à **Administration du site** > **Plugins** > **Modules d'activité** > **Outil externe** > **Gérer les outils**.

#### Coller l'URL

Vous verrez une carte intitulée **URL de l'outil**. Collez l'URL d'enregistrement FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenez-la ici</a>) dans le champ de texte et cliquez sur **Ajouter LTI Advantage**.

Moodle ouvre un écran d'enregistrement affichant l'identité de l'outil et les autorisations demandées. Vérifiez et cliquez sur **Activer** (ou **Enregistrer**, selon la version de Moodle).

La fenêtre contextuelle se ferme lorsque l'enregistrement est terminé ; le nouvel outil FastComments apparaît dans la liste **Outils** avec le statut **Actif**.

#### Rendre l'outil disponible

Par défaut, Moodle ajoute les nouveaux outils à la liste "Outils du cours" mais ne les affiche pas dans le sélecteur d'activité. Pour rendre FastComments visible dans tout le cours :

1. Cliquez sur l'icône d'engrenage sur la tuile FastComments.
2. Sous **Utilisation de la configuration de l'outil**, choisissez **Afficher dans le sélecteur d'activités et comme outil préconfiguré**.
3. Enregistrez.

Les enseignants peuvent maintenant ajouter FastComments à n'importe quel cours via **Ajouter une activité ou une ressource** > **FastComments**.