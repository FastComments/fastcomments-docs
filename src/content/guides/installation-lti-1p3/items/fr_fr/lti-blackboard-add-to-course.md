Une fois qu'un administrateur a enregistré FastComments comme outil LTI 1.3 Advantage et approuvé les politiques de l'institution, les enseignants l'ajoutent aux cours via les points d'insertion standard de Blackboard. Les étapes exactes diffèrent entre Ultra Course View et Original Course View, donc les deux sont couvertes ci-dessous.

#### Ultra Course View

Ultra Course View est le mode par défaut dans Blackboard Learn SaaS depuis 2026.

1. Ouvrez le cours et accédez à la page **Course Content**.
2. Survolez ou appuyez à l'endroit où vous souhaitez que le fil de commentaires apparaisse dans le plan et cliquez sur le bouton violet **+** (Add content).
3. Choisissez **Content Market**. Le panneau Content Market répertorie tous les outils LTI approuvés et les points d'insertion de Building Block pour votre institution.
4. Trouvez la tuile **FastComments** et cliquez dessus. Blackboard crée un élément de contenu à la position où vous avez ouvert le menu **+**.
5. L'élément apparaît dans le plan comme une entrée « Visible to students » par défaut pour les enseignants dont l'option **Hide from students** est désactivée par défaut. Si votre valeur par défaut est **Hidden**, l'élément est créé masqué et vous inversez le sélecteur de visibilité sur la ligne de l'élément quand vous êtes prêt.
6. Pour renommer l'élément, cliquez sur le titre dans le plan et saisissez un nouveau libellé. Le titre que les étudiants voient dans le plan est indépendant de l'identifiant du fil FastComments, donc le renommage est sans risque à tout moment.

Si vous ne voyez pas **Content Market** comme option, votre institution a masqué ce point d'insertion. Vous pouvez également accéder au même sélecteur via **More tools** dans le même menu **+** sous le groupe **LTI Tools**.

#### Original Course View

Original Course View est toujours pris en charge dans Learn SaaS et reste l'expérience principale pour les sites Learn 9.1 auto-hébergés sur la ligne de version Q4 2024 CU.

1. Ouvrez le cours et entrez dans une **Content Area** (par exemple, la zone **Information** ou **Content** par défaut dans le menu du cours).
2. Activez le **Edit Mode** avec le commutateur en haut à droite de la page.
3. Cliquez sur **Build Content** dans la barre d'actions.
4. Sous le sous-menu **Learning Tools**, cliquez sur **FastComments**. Le sous-menu Learning Tools est alimenté par les points d'insertion d'outils LTI 1.3 après qu'un administrateur a enregistré l'outil. Si vous ne le voyez pas, consultez la section des pièges ci-dessous.
5. Sur le formulaire **Create FastComments**, définissez :
   - **Name** : le libellé que les étudiants voient dans la zone de contenu.
   - **Description** : texte facultatif affiché au-dessus du fil intégré.
   - **Permit Users to View this Content** : commutateur de disponibilité Oui/Non.
   - **Track Number of Views** : activez si vous souhaitez les statistiques de vues par élément de Blackboard. FastComments exécute ses propres analyses de manière indépendante.
   - **Date and Time Restrictions** : fenêtres facultatives **Display After** / **Display Until**.
6. Soumettez. L'outil apparaît comme un élément cliquable dans la zone de contenu.

#### Intégration dans un élément ou un document

Dans les deux vues de cours, les enseignants intègrent FastComments en ligne dans le corps d'un Item, d'un Document ou de tout champ de texte enrichi via le bouton LTI Advantage de l'éditeur de contenu.

Ultra Course View :

1. Créez ou modifiez un **Document**.
2. Cliquez sur **Add content** dans le corps du document à l'endroit où vous souhaitez que le fil apparaisse.
3. Dans la barre d'outils de l'éditeur, ouvrez le menu **Insert content** et cliquez sur **Content Market** (le point d'entrée LTI Advantage / Deep Linking).
4. Choisissez **FastComments**. FastComments renvoie une charge utile de deep link et Blackboard insère un bloc intégré dans le corps du document à la position du curseur.
5. Enregistrez le document. Les étudiants voient le fil rendu en ligne lorsqu'ils le font défiler.

Original Course View :

1. Modifiez n'importe quel élément avec un corps de texte enrichi.
2. Dans la barre d'outils de l'éditeur de contenu, cliquez sur l'icône plus **Add Content** et choisissez **Content Market** (libellé **Add Content from External Tool** dans les anciennes versions Q4 2024 CU).
3. Choisissez **FastComments**. L'éditeur insère un bloc de remplacement référençant la ressource liée en profondeur.
4. Soumettez l'élément.

Chaque intégration de deep link produit son propre fil FastComments, donc un Item avec deux blocs FastComments intégrés possède deux flux de commentaires indépendants.

#### Visibilité, conditions de publication et restrictions de groupe

Les éléments de contenu FastComments se comportent comme tout autre élément de contenu Blackboard pour les règles de contrôle d'accès superposées.

- Ultra : cliquez sur le sélecteur de visibilité sur la ligne (**Visible to students**, **Hidden from students**, **Conditional availability**). La disponibilité conditionnelle prend en charge les fenêtres de date/heure, les règles de performance sur les éléments du relevé de notes et les règles de membres sur les groupes de cours.
- Original : ouvrez le menu contextuel de l'élément et choisissez **Adaptive Release** ou **Adaptive Release: Advanced** pour restreindre l'outil par date, appartenance, note ou statut de révision. Utilisez **Set Group Availability** sur l'élément pour le restreindre à des groupes de cours spécifiques.

FastComments respecte ce que la barrière de Blackboard décide. Si Blackboard masque l'élément à un étudiant, le lancement LTI n'a jamais lieu pour cet étudiant et il n'apparaît pas dans la vue de modération.

#### Comportement du relevé de notes

FastComments ne renvoie pas de notes via les LTI Advantage Assignment and Grade Services. Aucune colonne de note n'est créée automatiquement pour les éléments de contenu FastComments.

Si votre tenant Blackboard est configuré pour créer automatiquement une colonne de relevé de notes pour chaque nouvel élément de contenu, quelles que soient les métadonnées de notation, une colonne vide apparaît quand même. Pour la masquer :

- Ultra : ouvrez le **Gradebook**, cliquez sur l'en-tête de colonne, choisissez **Edit**, et désactivez **Show to students** ainsi que **Include in calculations**. Ou utilisez **Delete** si votre institution autorise la suppression de colonnes pour les éléments non notés.
- Original : ouvrez le **Grade Center**, cliquez sur le chevron de la colonne, choisissez **Hide from Users (on/off)**, et éventuellement **Hide from Instructor View** sous **Column Organization**.

#### Ce que voient les étudiants

Lorsqu'un étudiant ouvre l'élément FastComments ou fait défiler jusqu'à un bloc intégré :

1. Blackboard lance le message LTI 1.3 vers FastComments. L'étudiant est connecté via SSO en utilisant son identité Blackboard (nom, e-mail, avatar, rôle) sans voir de formulaire de connexion.
2. Le fil de commentaires s'affiche dans l'iframe. Le fil de discussion, les réponses, les mentions et les réactions sont tous disponibles selon les paramètres du widget de commentaires configurés dans FastComments.
3. Ses commentaires sont attribués à son compte Blackboard. Si l'étudiant modifie ultérieurement son nom ou sa photo dans Blackboard, le prochain lancement met à jour le profil FastComments.

Correspondance des rôles de Blackboard vers FastComments :

- **System Administrator** et **Course Builder** correspondent à **admin** FastComments.
- **Instructor** et **Teaching Assistant** correspondent à **moderator** FastComments.
- **Student**, **Guest** et **Observer** correspondent à **commenter** FastComments.

Les modérateurs voient les contrôles de modération (épingler, masquer, bannir, supprimer) en ligne sur chaque commentaire du fil.

#### Verrouiller l'accès public (recommandé)

Par défaut, les données de commentaires FastComments sont lisibles publiquement. Quiconque peut deviner l'URL d'un fil ou un point d'API peut consulter ses commentaires, même en dehors de Blackboard. Pour les discussions de cours, vous voudrez presque certainement restreindre la consultation aux seuls étudiants inscrits.

Ouvrez votre <a href="https://fastcomments.com/auth/my-account/customize-widget" target="_blank">page de personnalisation du widget</a> et créez une règle avec **Require SSO To View Comments** activé, puis définissez le niveau de sécurité sur **Secure SSO** afin que les fils ne puissent être chargés que via le lancement LTI signé.

Consultez [Protéger les fils de commentaires avec l'authentification unique](/guide-customizations-and-configuration.html#sso-require-to-view-comments) pour le guide complet, y compris comment limiter la règle à un seul domaine ou à une seule page.

#### Périmètre des fils

FastComments délimite chaque fil par **(hôte Blackboard, ID de cours, ID de resource link)**. Deux éléments FastComments dans le même cours produisent deux fils. Le même élément copié dans deux coquilles de cours (par exemple, via une copie de cours) produit deux fils, car Blackboard émet un nouvel ID de resource link lors de la copie. Pour conserver un fil partagé entre les copies de cours, utilisez le Deep Linking avec un URN de fil explicite configuré dans FastComments avant de lancer la copie.

#### Pièges spécifiques à Blackboard

**Tuile FastComments absente du menu Build Content (Original) ou du Content Market (Ultra).** L'administrateur a approuvé l'outil mais a laissé une politique d'institution bloquant le point d'insertion concerné. Allez dans **Administrator Panel** > **Integrations** > **LTI Tool Providers**, modifiez l'entrée FastComments, et confirmez que les points d'insertion **Course Content Tool** (Original) et **Course Content Tool - allow students** / **Deep Linking content tool** (Ultra) sont activés. Enregistrez et actualisez la page du cours.

**Erreur « Tool not configured for this context » ou « Tool is not deployed » au lancement.** Le périmètre de déploiement enregistré lors de l'enregistrement dynamique ne correspond pas au contexte d'institution auquel appartient le cours. Dans l'entrée du fournisseur d'outils de Blackboard, vérifiez que le **Deployment ID** correspond à ce que FastComments affiche sur sa page de configuration LTI 1.3 pour ce tenant. S'ils diffèrent, supprimez le point d'insertion et relancez l'enregistrement dynamique depuis une nouvelle URL d'enregistrement (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">obtenez-la ici</a>).

**La hauteur de l'iframe semble fixe ou le contenu est coupé.** Certains tenants Blackboard sont livrés avec une politique de sécurité de contenu (CSP) stricte qui bloque le postMessage de redimensionnement d'iframe LTI par défaut. FastComments émet à la fois le message de style Canvas `lti.frameResize` et le message conforme à la spécification IMS `org.imsglobal.lti.frameResize` pour maximiser la compatibilité, mais une surcharge CSP au niveau du tenant bloque l'écouteur parent. Demandez à votre administrateur de confirmer que `*.fastcomments.com` figure sur la liste autorisée des outils LTI et qu'aucun en-tête CSP personnalisé ne supprime les événements postMessage. Le redimensionnement fonctionne alors sans configuration supplémentaire.

**La copie de cours duplique les fils.** La copie de cours Blackboard émet de nouveaux ID de resource link pour les points d'insertion LTI, donc les cours copiés commencent avec des fils vides. C'est attendu. Si vous avez besoin que le cours copié hérite du fil d'origine, configurez le Deep Linking avec un URN de fil explicite avant de copier, ou contactez le support FastComments pour remapper les ID de fils en masse.

**L'étudiant voit une erreur Blackboard générique au lancement.** La cause est une revendication `email` manquante ou périmée. Confirmez que la politique d'institution pour FastComments a **Role**, **Name** et **Email Address** activés sous **User Fields to Send**. Enregistrez, puis relancez dans une nouvelle session de navigateur.