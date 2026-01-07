### Formatbeispiel

Die geplante Kommentare CSV sieht so aus:

|ID |Parent ID|URL ID       |URL|Name         |Avatar|Comment                              | Hours | Minutes | Seconds |
|---|---------|-------------|---|-------------|------|-------------------------------------|-------|---------|---------|
|1  |         |scheduledtest|   |Test Person 1|      |Hello!                               | 0     | 0       | 3       |
|2  |1        |scheduledtest|   |Test Person 2|      |How are you?                         | 0     | 0       | 10      |
|3  |2        |scheduledtest|   |Test Person 3|      |Leave them alone                     | 0     | 1       | 30      |
|4  |         |scheduledtest|   |Test Person 4|      |Does anyone have a cute cat to share?| 1     | 10      | 0       |


### Formatdetails

Die geplante Kommentare CSV-Datei unterstutzt die folgenden Spalten:

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

Die folgenden Spalten sind **erforderlich**:

- ID
- URL ID
- Name
- Comment
- Hours
- Minutes
- Seconds

Sie benotigen die Parent ID-Spalte, wenn Sie automatisierte verschachtelte Antworten unterstutzen mochten.

### Wie das Format funktioniert

Das Importformat funktioniert so:

1. Sie definieren eine Zeile in der CSV fur jeden Kommentar, den Sie posten mochten.
2. Der Kommentar wird nach der angegebenen Verzogerung gepostet (Stunden + Minuten + Sekunden).
   1. Fur manuell geplante Importe sind Verzogerungen relativ zu dem Zeitpunkt, an dem Sie "play" in der Benutzeroberflache drucken, nachdem der Import abgeschlossen ist - **nicht wenn der Import startet**.
   2. Fur automatisch geplante Importe ist die Verzogerung ab dem Zeitpunkt des Seitenladens.
3. Sie mussen eine ID definieren. Sie konnen einfach aufsteigende Bezeichner wie 1, 2, 3, 4, 5 verwenden.
4. Wenn Sie verschachtelte Antworten verwenden mochten, setzen Sie einfach den `Parent ID` Spaltenwert auf die `ID` eines anderen Kommentars.
5. Das `Comment`-Feld unterstutzt dieselbe Syntax, die FastComments in seinem Kommentar-Widget fur die Textgestaltung und das Hinzufugen von Bildern unterstutzt.
6. Das `Avatar`-Feld muss, wenn verwendet, ein offentlich zugangliches Bild sein. Es wird auf unser CDN kopiert und von dort bereitgestellt.
7. Sie konnen alle Kommentare nach der Wiedergabe loschen, oder wenn die Wiedergabe gestoppt wird.
8. Das Loschen geschieht live, sodass Sie denselben geplanten Import immer wieder verwenden konnen.

### Ein Beispiel

[Eine Beispiel-CSV-Datei finden Sie hier](/csv/fastcomments-scheduled-comments-example.csv).
