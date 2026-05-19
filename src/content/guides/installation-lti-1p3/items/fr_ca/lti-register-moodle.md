**Vous utilisez Moodle ?** Nous publions également un plugin Moodle dédié pour FastComments offrant une intégration plus poussée que LTI 1.3 (hooks de synchronisation des notes, rapports d'activité approfondis, interface de configuration native Moodle). Voir le <a href="/guide-installation-moodle.html" target="_blank">guide d'installation du plugin Moodle</a>. Le flux LTI 1.3 ci‑dessous est le bon choix si vous voulez une inscription unique qui couvre également d'autres LMS, ou si votre administrateur Moodle n'installera pas de plugins tiers.

Moodle 4.0+ prend en charge l'enregistrement dynamique LTI 1.3 via le plugin External Tool.

#### Ouvrez l'écran de gestion des outils

1. Connectez-vous à Moodle en tant qu'administrateur du site.
2. Naviguez vers **Site administration** > **Plugins** > **Activity modules** > **External tool** > **Manage tools**.

#### Collez l'URL

Vous verrez une carte intitulée **Tool URL**. Collez l'URL d'enregistrement FastComments (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenez-la ici</a>) dans le champ de texte et cliquez sur **Add LTI Advantage**.

Moodle ouvre un écran d'enregistrement affichant l'identité de l'outil et les autorisations demandées. Vérifiez et cliquez sur **Activate** (ou **Register**, selon la version de Moodle).

La fenêtre contextuelle se ferme une fois l'enregistrement terminé ; le nouvel outil FastComments apparaît dans la liste **Tools** avec le statut **Active**.

#### Rendez-le disponible

Par défaut, Moodle ajoute les nouveaux outils à la liste « Course tools » mais ne les affiche pas dans le sélecteur d'activités. Pour rendre FastComments disponible dans tout le cours :

1. Cliquez sur l'icône d'engrenage de la vignette FastComments.
2. Sous **Tool configuration usage**, choisissez **Show in activity chooser and as a preconfigured tool**.
3. Enregistrez.

Les enseignants peuvent maintenant ajouter FastComments à n'importe quel cours via **Add an activity or resource** > **FastComments**.