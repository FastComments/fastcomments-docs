### Comment fonctionne la sélection de texte

Lorsque les utilisateurs sélectionnent du texte dans le conteneur Collab Chat, le widget capture cette sélection et leur permet de démarrer une discussion. La sélection peut être aussi petite qu’un seul mot ou aussi grande que plusieurs paragraphes couvrant différents éléments.

### Délai de sélection

Sur les appareils de bureau, il y a un délai de 3,5 secondes entre le moment où un utilisateur sélectionne du texte et l’apparition de l’invite de discussion. Cela évite que l’interface clignote lorsque les utilisateurs mettent simplement en surbrillance du texte pour le copier ou le lire. Sur les appareils mobiles, l’invite apparaît immédiatement puisque la sélection de texte est plus délibérée sur les écrans tactiles.


### Identifiants de conversation uniques

Chaque conversation reçoit un `urlId` unique qui combine l’URL de la page, le chemin de l’élément DOM et la plage de texte sérialisée. Cela garantit que chaque sélection de texte crée une conversation distincte qui peut être retrouvée ultérieurement.

Si vous fournissez un `urlId` personnalisé dans votre configuration, il sera combiné avec le chemin de l’élément et la plage de texte pour créer l’identifiant final.

### Mises en évidence visuelles

Lorsqu’une discussion existe pour une sélection de texte particulière, ce texte reçoit une mise en évidence visuelle. Le surlignage est implémenté à l’aide de couleurs de fond et apparaît au survol ou lorsque la fenêtre de chat associée est ouverte.

Le système de surlignage fonctionne en enveloppant le texte sélectionné dans un élément spécial qui peut être stylé. Cette approche garantit que les surlignages restent précis même lorsque la structure HTML sous-jacente est complexe.

### Positionnement de la fenêtre de chat

Lorsqu’un utilisateur clique sur un surlignage ou crée une nouvelle annotation, une fenêtre de chat apparaît près du texte sélectionné. Le widget calcule automatiquement la meilleure position pour cette fenêtre en fonction de l’espace disponible dans la fenêtre d’affichage.

Le système de positionnement utilise des classes CSS comme `to-right`, `to-left`, `to-top` et `to-bottom` pour indiquer dans quelle direction la fenêtre de chat doit s’étendre à partir du surlignage. Sur les appareils mobiles (écrans de moins de 768px de large), la fenêtre de chat apparaît toujours en plein écran pour une meilleure ergonomie.

### Dimensions de la fenêtre de chat

Les fenêtres de chat font 410px de large sur desktop avec un espacement de 20px et une flèche visuelle de 16px pointant vers le texte surligné. Ces dimensions sont fixes pour garantir un comportement cohérent, mais vous pouvez personnaliser l’apparence avec du CSS.

### Sélections inter-éléments

Les utilisateurs peuvent sélectionner du texte qui s’étend sur plusieurs éléments HTML, comme un surlignage allant du milieu d’un paragraphe au début d’un autre. Le système de sérialisation de plage gère cela correctement et mettra en évidence tout le texte sélectionné même à travers les limites des éléments.

### Compatibilité des navigateurs

Le système de sélection de texte utilise l’API standard `window.getSelection()` qui est prise en charge par tous les navigateurs modernes. Pour les anciennes versions d’Internet Explorer, il revient à `document.selection` pour la compatibilité.

### Persistance des sélections

Une fois qu’une conversation est créée pour une sélection de texte, cette annotation persiste même si la page est rechargée. La plage sérialisée et le chemin DOM permettent au widget de restaurer les surlignages exactement au même endroit lorsque les utilisateurs reviennent sur la page.

Cela fonctionne de manière fiable tant que le contenu de votre page reste stable. Si vous modifiez le contenu textuel ou restructurez votre HTML, les annotations existantes peuvent ne plus s’aligner correctement avec le texte. Pour cette raison, il est préférable d’éviter des modifications majeures du contenu sur les pages comportant des annotations actives, ou d’envisager de migrer les annotations lorsque des changements de contenu sont nécessaires.