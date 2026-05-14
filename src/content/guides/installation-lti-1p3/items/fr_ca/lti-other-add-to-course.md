Une fois que FastComments est enregistré auprès de la plateforme, les enseignants l'ajoutent au contenu de cours en utilisant les flux standard d'outils externes de la plateforme. Cette page couvre Sakai 23.x et Schoology Enterprise.

#### Sakai

**1. Ajouter FastComments à un site**

Le responsable du site active l'outil au niveau de chaque site :

1. Ouvrez le site et cliquez sur **Info du site** dans la navigation de gauche.
2. Cliquez sur **Gérer les outils**.
3. Faites défiler jusqu'à la liste **Outils externes** et activez **FastComments**.
4. Cliquez sur **Continuer**, vérifiez la liste des outils, puis cliquez sur **Terminer**.

FastComments apparaît maintenant comme un élément de navigation à gauche dans le site.

**2. Réordonner l'entrée de navigation gauche**

Allez dans **Info du site** > **Ordre des outils**. Faites glisser **FastComments** à la position désirée et cliquez sur **Enregistrer**. Vous pouvez aussi renommer l'étiquette de navigation et la masquer aux étudiants depuis cet écran.

**3. Intégrer en ligne dans une page Leçons**

Pour placer FastComments directement à l'intérieur d'une page Leçons plutôt que comme outil autonome dans la navigation de gauche :

1. Ouvrez l'outil **Leçons** dans le site.
2. Cliquez sur **Ajouter du contenu** > **Ajouter un outil externe**.
3. Sélectionnez **FastComments** dans la liste.
4. Si FastComments a annoncé le Deep Linking lors de l'enregistrement, Sakai ouvre le sélecteur de contenu de l'outil afin que vous puissiez choisir ou étiqueter le fil. Si le Deep Linking n'a pas été annoncé, Sakai insère un lien de lancement par défaut.
5. Enregistrez l'élément Leçons.

Chaque instance intégrée obtient son propre fil, limité à ce lien de ressource.

**4. Ajustements de permissions pour l'accès étudiant**

Sakai contrôle les lancements d'outils externes via les Realms. Pour confirmer que les étudiants peuvent lancer FastComments :

1. Connectez-vous en tant qu'admin Sakai et ouvrez **Administration Workspace** > **Realms**.
2. Ouvrez le realm pertinent (par exemple, `!site.template.course` ou le realm du site spécifique).
3. Confirmez que le rôle `access` a `lti.launch` activé et que les permissions de rôle dans le groupe **external.tools** sont accordées.
4. Enregistrez le realm.

Pour des remplacements au niveau du site, le mainteneur peut ajuster la visibilité de l'outil par rôle depuis **Info du site** > **Ordre des outils** en cachant ou en affichant FastComments pour chaque rôle.

**5. Ce que voient les étudiants**

Les étudiants cliquent sur l'élément de navigation FastComments (ou font défiler jusqu'au bloc Leçons intégré) et arrivent directement dans la vue de fil de commentaires. L'authentification unique (SSO) se fait automatiquement : Sakai envoie l'identité de l'utilisateur dans le lancement LTI et FastComments les connecte sous leur compte Sakai.

Correspondance des rôles :

- Sakai `Instructor` -> modérateur FastComments
- Sakai `Admin` (admin dans Administration Workspace) -> administrateur FastComments
- Sakai `Student` / `access` -> commentateur FastComments

**6. Problèmes courants de Sakai**

- **Outil non visible dans Gérer les outils.** Si FastComments n'apparaît pas dans la liste Outils externes, l'administrateur Sakai doit ouvrir le registre d'outils (**Administration Workspace** > **Outils externes** > **FastComments**) et définir **Stealthed** sur `false`. Les outils stealthés sont cachés du sélecteur Gérer les outils par site.
- **Lancements qui échouent dans des navigateurs avec sessions partagées.** Le jeton CSRF du portail Sakai est lié à la session du navigateur. Si un étudiant est connecté à deux sites Sakai dans différents onglets ou a une session périmée, le lancement renvoie un 403. Solution : fermer les autres onglets Sakai, se déconnecter, se reconnecter et relancer. Les administrateurs peuvent aussi augmenter `sakai.csrf.token.cache.ttl` si cela se produit à l'échelle du cluster.
- **Intégration par iframe.** Vérifiez que `lti.frameheight` dans `sakai.properties` est suffisamment grand (600 ou plus) afin que le fil de commentaires ne soit pas tronqué à l'intérieur d'une page Leçons.

#### Schoology

Schoology Enterprise comporte deux scénarios d'installation. Confirmez lequel s'applique avant d'ajouter l'outil à un cours.

**1. Deux scénarios d'installation**

- **(a) Installation au niveau de l'entreprise.** L'administrateur système Schoology a installé FastComments au niveau de l'organisation et l'a assigné à tous les cours ou à des modèles de cours spécifiques. Les enseignants sautent l'installation et vont directement à « Ajouter du matériel ».
- **(b) Auto-installation par l'enseignant.** L'enseignant installe l'outil dans un seul cours depuis **Options du cours** > **Outils externes** > **Installer des applications LTI**. L'auto-installation requiert que l'administrateur système ait d'abord approuvé l'application FastComments au niveau organisationnel.

**2. Ajouter FastComments comme matériel de cours**

Dans le cours :

1. Ouvrez le cours et allez dans **Matériel**.
2. Cliquez sur **Ajouter du matériel** > **Ajouter fichier/lien/outil externe**.
3. Choisissez **Outil externe**.
4. Sélectionnez **FastComments** dans la liste des outils enregistrés.
5. Définissez un **Nom** (c'est ce que les étudiants voient dans la liste de matériel) et une **Description** optionnelle.
6. Laissez **Enable Grading** (grade passback) **OFF**. FastComments ne renvoie pas de notes à Schoology, donc activer la rétroaction de notes crée une colonne vide dans le carnet de notes.
7. Cliquez sur **Soumettre**.

Le matériel apparaît maintenant dans la liste du cours et ouvre le fil FastComments lorsqu'on clique dessus.

**3. Intégration en ligne via l'éditeur de texte enrichi**

Si l'administrateur système a activé le placement Deep Linking pour FastComments lors de l'enregistrement, les enseignants peuvent intégrer le fil de commentaires dans n'importe quel champ de texte enrichi (instructions d'activité, corps de page, amorces de discussion) :

1. Ouvrez l'éditeur de texte enrichi sur la page cible.
2. Cliquez sur l'icône **Outil externe** (pièce de puzzle) dans la barre d'outils.
3. Choisissez **FastComments**.
4. Configurez l'intégration dans la boîte de dialogue de deep-linking et cliquez sur **Insérer**.
5. Enregistrez la page.

Si le bouton Outil externe n'apparaît pas dans l'éditeur de texte enrichi, le Deep Linking est désactivé pour cet outil sur ce locataire. Voir les problèmes ci-dessous.

**4. Visibilité et assignation par section**

Schoology limite la disponibilité des outils par section via Options du cours :

1. Depuis le cours, cliquez sur **Options du cours** > **Outils externes**.
2. Pour chaque application LTI installée, vous contrôlez si elle est disponible pour toutes les sections du cours ou pour des sections spécifiques.
3. Pour restreindre FastComments à certaines sections, décochez les sections qui ne doivent pas voir l'outil.
4. L'accès au niveau de la section détermine aussi quelles sections voient l'entrée **Ajouter du matériel** > **Outil externe** pour FastComments.

**5. Ce que voient les étudiants**

Les étudiants cliquent sur le matériel FastComments (ou font défiler jusqu'à l'intégration en ligne) et arrivent dans la discussion en fil. L'authentification unique (SSO) se fait automatiquement via le lancement LTI sous leur compte Schoology.

Correspondance des rôles :

- Schoology `Administrator` -> administrateur FastComments
- Schoology `Instructor` -> modérateur FastComments
- Schoology `Student` -> commentateur FastComments

**6. Problèmes courants de Schoology**

- **Réservé à l'entreprise.** Les comptes Schoology personnels et gratuits ne peuvent pas installer d'outils LTI 1.3. Si votre locataire est sur le niveau gratuit, l'option **Outils externes** est absente des Options du cours. Passez à Schoology Enterprise pour utiliser FastComments.
- **Deep Linking désactivé par défaut pour le locataire.** Certains locataires Schoology restreignent le placement Deep Linking au niveau organisationnel. Dans ce cas, les enseignants ne voient que le flux **Ajouter du matériel** > **Outil externe** et pas le bouton Outil externe dans l'éditeur de texte enrichi. Pour activer l'intégration en ligne, l'administrateur système se rend dans **Paramètres système** > **Intégration** > **LTI 1.3** > **FastComments** et active le placement **Content Item / Deep Linking**, puis enregistre.
- **Remplacement d'assignation par section.** Si FastComments est assigné au niveau de l'entreprise mais que l'enseignant ne le voit pas dans **Ajouter du matériel**, la section du cours est exclue dans l'assignation au niveau organisationnel. Demandez à l'administrateur système d'ajouter la section à l'assignation de l'application FastComments.
- **Nom du matériel vs. identité du fil.** Renommer le matériel dans Schoology ne déplace pas le fil de commentaires. Les fils sont indexés sur l'ID du lien de ressource LTI, donc un renommage conserve le même fil ; supprimer et recréer le matériel crée un nouveau fil vide.