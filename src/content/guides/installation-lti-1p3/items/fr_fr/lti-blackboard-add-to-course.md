Une fois qu'un administrateur a enregistré FastComments en tant qu'outil LTI 1.3 Advantage et approuvé les politiques de l'établissement, les enseignants l'ajoutent aux cours via les points de placement standard de Blackboard. Les étapes exactes diffèrent entre la Vue de cours Ultra et la Vue classique du cours, les deux cas étant décrits ci‑dessous.

#### Vue de cours Ultra

La Vue de cours Ultra est la valeur par défaut dans Blackboard Learn SaaS depuis 2026.

1. Ouvrez le cours et allez sur la page **Contenu du cours**.
2. Survolez ou appuyez à l'endroit où vous voulez positionner le fil de commentaires dans la structure et cliquez sur le bouton violet **+** (Ajouter du contenu).
3. Choisissez **Marché de contenu**. Le panneau Marché de contenu liste tous les outils LTI approuvés et les placements Building Block pour votre établissement.
4. Trouvez la tuile **FastComments** et cliquez dessus. Blackboard crée un élément de contenu à la position où vous avez ouvert le menu **+**.
5. L'élément apparaît dans la structure par défaut comme une entrée « Visible pour les étudiants » pour les enseignants dont le réglage personnel par défaut **Masquer aux étudiants** est désactivé. Si votre par défaut est **Masqué**, l'élément est créé masqué et vous activez le sélecteur de visibilité sur la ligne de l'élément lorsque vous êtes prêt.
6. Pour renommer l'élément, cliquez sur le titre dans la structure et tapez un nouveau libellé. Le titre que les étudiants voient dans la structure est indépendant de l'identifiant du fil FastComments, donc le renommage est sans risque à tout moment.

Si vous ne voyez pas **Marché de contenu** comme option, votre établissement a masqué le placement. Vous pouvez aussi accéder au même sélecteur via **Plus d'outils** dans le même menu **+** sous le groupe **Outils LTI**.

#### Vue classique du cours

La Vue classique du cours est toujours prise en charge dans Learn SaaS et reste l'expérience principale pour les sites Learn 9.1 auto‑hébergés sur la ligne de correctifs CU de Q4 2024.

1. Ouvrez le cours et entrez dans une **Zone de contenu** (par exemple, la zone par défaut **Informations** ou **Contenu** dans le menu du cours).
2. Activez le **Mode édition** avec le sélecteur en haut à droite de la page.
3. Cliquez sur **Créer du contenu** dans la barre d'actions.
4. Sous le sous‑menu **Outils d'apprentissage**, cliquez sur **FastComments**. Le sous‑menu Outils d'apprentissage est alimenté par les placements d'outil LTI 1.3 après qu'un administrateur a enregistré l'outil. Si vous ne le voyez pas, consultez la section des problèmes connus ci‑dessous.
5. Dans le formulaire **Créer FastComments**, réglez :
   - **Nom** : le libellé que les étudiants voient dans la zone de contenu.
   - **Description** : texte optionnel affiché au‑dessus du fil intégré.
   - **Permettre aux utilisateurs de voir ce contenu** : bascule Oui/Non pour la disponibilité.
   - **Suivre le nombre de vues** : activez si vous voulez les statistiques de vues par élément de Blackboard. FastComments exécute ses propres analyses indépendamment.
   - **Restrictions de date et d'heure** : fenêtres optionnelles **Afficher après** / **Afficher jusqu'à**.
6. Soumettez. L'outil apparaît comme un élément cliquable dans la zone de contenu.

#### Intégration dans un élément ou un document

Dans les deux vues de cours, les enseignants intègrent FastComments en ligne dans le corps d'un Élement, d'un Document ou de tout champ riche via le bouton LTI Advantage de l'éditeur de contenu.

Vue Ultra :

1. Créez ou modifiez un **Document**.
2. Cliquez sur **Ajouter du contenu** à l'intérieur du corps du document à l'endroit où vous voulez que le fil apparaisse.
3. Dans la barre d'outils de l'éditeur, ouvrez le menu **Insérer du contenu** et cliquez sur **Marché de contenu** (le point d'entrée LTI Advantage / Deep Linking).
4. Choisissez **FastComments**. FastComments retourne une charge utile de deep-link et Blackboard insère un bloc intégré dans le corps du document à la position du curseur.
5. Enregistrez le document. Les étudiants voient le fil rendu en ligne lorsqu'ils font défiler la page.

Vue classique :

1. Modifiez n'importe quel élément avec un corps en texte enrichi.
2. Dans la barre d'outils de l'éditeur de contenu, cliquez sur l'icône plus **Ajouter du contenu** et choisissez **Marché de contenu** (étiqueté **Ajouter du contenu depuis un outil externe** dans les anciens CU de Q4 2024).
3. Choisissez **FastComments**. L'éditeur insère un bloc de remplacement référant la ressource deep-linked.
4. Soumettez l'élément.

Chaque intégration deep-link crée son propre fil FastComments, donc un élément contenant deux blocs FastComments intégrés possède deux flux de commentaires indépendants.

#### Visibilité, conditions de publication et restrictions de groupe

Les éléments de contenu FastComments se comportent comme n'importe quel autre élément de contenu Blackboard pour les règles de contrôle d'accès qui leur sont appliquées.

- Ultra : cliquez sur le sélecteur de visibilité sur la ligne (**Visible pour les étudiants**, **Masqué aux étudiants**, **Disponibilité conditionnelle**). La disponibilité conditionnelle prend en charge les fenêtres de date/heure, les règles de performance liées aux éléments du carnet de notes et les règles d'appartenance aux groupes de cours.
- Classique : ouvrez le menu contextuel de l'élément et choisissez **Publication adaptive** ou **Publication adaptive : Avancé** pour restreindre l'outil par date, appartenance, note ou statut d'examen. Utilisez **Définir la disponibilité par groupe** sur l'élément pour restreindre à des groupes de cours spécifiques.

FastComments respecte la décision de verrouillage de Blackboard. Si Blackboard masque l'élément pour un étudiant, le lancement LTI n'a jamais lieu pour cet étudiant et il n'apparaît pas dans la vue modérateur.

#### Comportement dans le carnet de notes

FastComments ne renvoie pas de notes via LTI Advantage Assignment and Grade Services. Aucune colonne de notes n'est créée automatiquement pour les éléments de contenu FastComments.

Si votre instance Blackboard est configurée pour créer automatiquement une colonne de carnet de notes pour chaque nouvel élément de contenu indépendamment des métadonnées de notation, une colonne vide apparaît quand même. Pour la masquer :

- Ultra : ouvrez le **Carnet de notes**, cliquez sur l'en‑tête de colonne, choisissez **Modifier**, et désactivez **Afficher aux étudiants** ainsi que **Inclure dans les calculs**. Ou utilisez **Supprimer** si votre établissement autorise la suppression de colonnes pour les éléments non notés.
- Classique : ouvrez le **Centre de notes**, cliquez sur le chevron de la colonne, choisissez **Masquer pour les utilisateurs (on/off)**, et éventuellement **Masquer pour la vue de l'enseignant** sous **Organisation des colonnes**.

#### Ce que voient les étudiants

Lorsqu'un étudiant ouvre l'élément FastComments ou fait défiler une zone contenant un bloc intégré :

1. Blackboard lance le message LTI 1.3 vers FastComments. L'étudiant est connecté via SSO avec son identité Blackboard (nom, e‑mail, avatar, rôle) sans voir de formulaire de connexion.
2. Le fil de commentaires s'affiche dans l'iframe. Le fil, les réponses imbriquées, les mentions et les réactions sont tous disponibles selon les paramètres du widget de commentaires configurés dans FastComments.
3. Leurs commentaires sont attribués à leur compte Blackboard. Si l'étudiant modifie ensuite son nom ou sa photo dans Blackboard, le lancement suivant mettra à jour le profil FastComments.

Correspondance des rôles de Blackboard vers FastComments :

- **Administrateur système** et **Constructeur de cours** correspondent à FastComments **admin**.
- **Enseignant** et **Assistant pédagogique** correspondent à FastComments **moderator**.
- **Étudiant**, **Invité**, et **Observateur** correspondent à FastComments **commenter**.

Les modérateurs voient les contrôles de modération (épingler, masquer, bannir, supprimer) directement sur chaque commentaire du fil.

#### Portée des fils

FastComments scope chaque fil par **(hôte Blackboard, ID de cours, ID de lien de ressource)**. Deux éléments FastComments dans le même cours produisent deux fils. Le même élément copié dans deux environnements de cours (par exemple, via la copie de cours) produit deux fils, car Blackboard émet un nouvel ID de lien de ressource lors de la copie. Pour conserver un fil partagé lors des copies de cours, utilisez le Deep Linking avec un URN de fil explicite configuré dans FastComments avant de lancer la copie.

#### Problèmes spécifiques à Blackboard

**Tuile FastComments manquante dans le menu Créer du contenu (Classique) ou le Marché de contenu (Ultra).** L'administrateur a approuvé l'outil mais a laissé une politique d'établissement bloquant le placement concerné. Allez dans **Panneau d'administration** > **Intégrations** > **Fournisseurs d'outils LTI**, modifiez l'entrée FastComments, et confirmez que les placements **Outil Contenu de cours** (Classique) et **Outil Contenu de cours - autoriser les étudiants** / **Outil de contenu Deep Linking** (Ultra) sont activés. Enregistrez et actualisez la page du cours.

**Erreur « Tool not configured for this context » ou « Tool is not deployed » au lancement.** La portée de déploiement enregistrée lors de l'enregistrement dynamique ne correspond pas au contexte institutionnel auquel appartient le cours. Dans l'entrée du fournisseur d'outils de Blackboard, vérifiez que **Deployment ID** correspond à ce que FastComments affiche sur sa page de configuration LTI 1.3 pour ce locataire. S'ils diffèrent, supprimez le placement et relancez l'enregistrement dynamique depuis une URL d'enregistrement fraîche (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">récupérez‑la ici</a>).

**La hauteur de l'iframe semble fixe ou le contenu est rogné.** Certains locataires Blackboard livrent une Content Security Policy stricte qui bloque le postMessage de redimensionnement d'iframe LTI par défaut. FastComments émet à la fois le message de style Canvas `lti.frameResize` et le message au format spécifié par l'IMS `org.imsglobal.lti.frameResize` pour maximiser la compatibilité, mais une surcharge CSP au niveau du locataire bloque l'écouteur parent. Demandez à votre administrateur de confirmer que `*.fastcomments.com` est sur la liste blanche des outils LTI et qu'aucun en‑tête CSP personnalisé ne supprime les événements postMessage. Le redimensionnement fonctionnera alors sans configuration supplémentaire.

**La copie de cours duplique les fils.** La copie de cours de Blackboard émet de nouveaux ID de lien de ressource pour les placements LTI, donc les cours copiés commencent avec des fils vides. C'est attendu. Si vous avez besoin que le cours copié hérite du fil original, configurez le Deep Linking avec un URN de fil explicite avant la copie, ou contactez le support FastComments pour remapper les ID de fil en masse.

**Un étudiant voit une erreur générique Blackboard au lancement.** La cause est une réclamation `email` manquante ou périmée. Confirmez que la politique d'établissement pour FastComments a **Rôle**, **Nom**, et **Adresse e‑mail** activés sous **Champs utilisateur à envoyer**. Enregistrez, puis relancez dans une nouvelle session de navigateur.