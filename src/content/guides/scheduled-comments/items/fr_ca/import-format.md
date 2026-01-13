### Exemple de format

Le CSV des Commentaires Programmes ressemble a ceci:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### Details du format

Le fichier CSV des Commentaires Programmes prend en charge les colonnes suivantes:

- ID
- Parent ID
- URL ID
- URL
- Name
- Avatar
- Comment
- Hours
- Minutes
- Seconds

Les colonnes suivantes sont **obligatoires**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

Vous aurez besoin de la colonne Parent ID si vous souhaitez prendre en charge les reponses imbriquees automatisees.

### Comment fonctionne le format

Le format d'importation fonctionne comme suit:

1. Vous definissez une ligne dans le CSV pour chaque commentaire que vous souhaitez publier.
2. Le commentaire est publie apres le delai specifie (heures + minutes + secondes).
   1. Pour les importations programmees manuellement, les delais sont relatifs au moment ou vous appuyez sur "play" dans l'interface, apres que l'importation est terminee - **pas quand l'importation commence**.
   2. Pour les importations programmees automatiquement, le delai est a partir du moment du chargement de la page.
3. Vous devez definir un ID. Vous pouvez simplement utiliser des identifiants incrementaux comme 1, 2, 3, 4, 5.
4. Si vous souhaitez utiliser des reponses imbriquees, definissez simplement la valeur de la colonne `Parent ID` sur l'`ID` d'un autre commentaire.
5. Le champ `Comment` prend en charge la meme syntaxe que FastComments prend en charge dans son widget de commentaires pour styliser le texte et ajouter des images.
6. Le champ `Avatar`, s'il est utilise, doit etre une image accessible publiquement. Elle sera copiee et servie depuis notre CDN.
7. Vous pouvez supprimer tous les commentaires apres la lecture, ou si la lecture est arretee.
8. La suppression se fait en direct, vous pouvez donc reutiliser la meme importation programmee encore et encore.

### Un exemple

[Un exemple de fichier CSV est ici](/csv/fastcomments-scheduled-comments-example.csv).
