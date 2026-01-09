### Positionnement basé sur des pourcentages

Image Chat utilise des coordonnées basées sur des pourcentages au lieu de coordonnées en pixels pour positionner les marqueurs de discussion sur les images. Lorsqu'un utilisateur clique sur une image, le widget convertit les coordonnées en pixels du clic en pourcentages de la largeur et de la hauteur de l'image. Cette approche garantit que les marqueurs restent à la bonne position quelle que soit la façon dont l'image est affichée.

Par exemple, si un utilisateur clique à 250 pixels du bord gauche d'une image de 1000px de large, le widget enregistre cela comme étant à 25% depuis la gauche. Quand un autre utilisateur affiche la même image à 500px de large sur un appareil mobile, le marqueur apparaît à 125 pixels du bord gauche, ce qui représente toujours 25% de la largeur.

### Avantages pour les mises en page adaptatives

Ce système en pourcentage permet à Image Chat de fonctionner de manière fluide sur toutes les tailles d'appareils et orientations. Vos images peuvent être affichées à différentes tailles selon la largeur de l'écran, les règles CSS ou les contraintes du conteneur, mais les marqueurs s'alignent toujours correctement avec les emplacements prévus.

Les utilisateurs sur ordinateurs de bureau avec de grands écrans voient les marqueurs aux mêmes positions relatives que les utilisateurs sur smartphones. Les marqueurs se redimensionnent proportionnellement avec l'image elle-même.

### Mise à l'échelle de l'image et rapport d'aspect

Tant que votre image conserve son rapport d'aspect lors du redimensionnement (ce qui est le comportement par défaut du navigateur), les marqueurs basés sur des pourcentages resteront parfaitement alignés. Le widget part du principe que les images se redimensionnent proportionnellement et calcule les positions en se basant sur cette hypothèse.

Si vous appliquez du CSS qui déforme le rapport d'aspect de l'image (comme en utilisant `object-fit: cover` avec des dimensions spécifiques), les positions des marqueurs peuvent ne pas s'aligner correctement. Pour de meilleurs résultats, laissez les images se redimensionner naturellement ou utilisez `object-fit: contain` pour conserver le rapport d'aspect.

### Taille des carrés de chat

La taille visuelle des marqueurs de chat est également basée sur des pourcentages. L'option de configuration `chatSquarePercentage` est par défaut à 5%, ce qui signifie que chaque carré représente 5% de la largeur de l'image. Cela garantit un poids visuel cohérent sur différentes tailles d'images.

Sur une image de 1000px de large avec le paramètre par défaut de 5%, les marqueurs font 50px de côté. Sur une image de 500px de large, les mêmes marqueurs font 25px de côté. Ils restent proportionnels à la taille de l'image.

### Comportement sur mobile

Sur les écrans de moins de 768px de large, Image Chat passe à une disposition optimisée pour mobile. Les fenêtres de discussion s'ouvrent en plein écran au lieu de flotter à côté du marqueur. Ceci offre une meilleure utilisabilité sur les petits écrans où les fenêtres flottantes cacheraient trop l'image.

Les marqueurs eux-mêmes restent visibles et cliquables à leurs positions basées sur des pourcentages. Les utilisateurs peuvent toujours voir où se situent toutes les discussions et toucher les marqueurs pour ouvrir l'interface de chat en plein écran.

### Chargement dynamique des images

Le système basé sur des pourcentages fonctionne correctement même lorsque les images se chargent de manière asynchrone ou changent de taille après le chargement de la page. Le widget surveille l'élément image et recalculera les positions des marqueurs chaque fois que les dimensions de l'image changent.

Si vous chargez les images paresseusement (lazy-loading) ou si vous mettez en œuvre des images responsives avec différentes tailles selon les points d'arrêt, les marqueurs s'ajusteront automatiquement lorsque la taille de l'image changera.

### Cohérence entre appareils

Parce que les coordonnées sont stockées en pourcentages dans la base de données, une discussion créée sur un ordinateur de bureau apparaît exactement au même emplacement relatif lorsqu'elle est consultée sur une tablette ou un téléphone. Les utilisateurs peuvent collaborer entre appareils sans incohérences de positionnement.

Cela fonctionne dans les deux sens. Une discussion créée en touchant un emplacement précis sur un appareil mobile apparaîtra au même emplacement relatif lorsqu'elle sera consultée sur un grand écran d'ordinateur de bureau.

### Considérations liées à la zone d'affichage

Le widget calcule les pourcentages par rapport à l'élément image lui-même, et non par rapport à la zone d'affichage. Cela signifie que faire défiler la page ou modifier la taille de la fenêtre du navigateur n'affecte pas les positions des marqueurs. Les marqueurs restent ancrés à leurs emplacements sur l'image, indépendamment des changements de la zone d'affichage.

### Préparation du contenu pour l'avenir

L'approche basée sur des pourcentages rend vos discussions sur les images résilientes aux changements de mise en page, de design ou d'écosystème d'appareils. À mesure que de nouvelles tailles d'écran et de nouveaux appareils apparaissent, les discussions existantes continueront de s'afficher correctement sans nécessiter de mises à jour ou de migrations.