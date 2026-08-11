In dem Fall, dass Daten verschoben werden müssen, bietet FastComments ein Self‑Service‑Tool zum Verschieben von Kommentaren zwischen Seiten und Artikeln.

So sieht das Formular zum Kopieren von Kommentaren aus:

[app-screenshot-start url='/auth/my-account/manage-data/copy-comments'; selector = '.account-block'; alt='Kopierkommentar-Formular mit dem Feld From URL ID und den Feldern To URL ID und URL'; title='Das Kopierkommentar-Formular' app-screenshot-end]

### Ausfüllen der "From"-Felder

Um zu entscheiden, von wo Kommentare verschoben werden sollen, benötigen wir lediglich die Quell‑`URL ID`.

Wenn Sie keinen Wert für `urlId` in der Konfiguration des Kommentar‑Widgets übergeben, ist dies eine „saubere“ Version der Seiten‑URL.

Sie können sehen, welche Werte Ihre Kommentare für `URL ID` haben, indem Sie sie exportieren.

### Ausfüllen der "To"-Felder

Um zu entscheiden, wohin Kommentare verschoben werden sollen, benötigen wir die Ziel‑`URL ID` und die `URL`.

Die `URL ID` ist der Behälter, in den der Kommentar abgelegt wird. Das `URL`‑Feld wird verwendet, damit Sie direkt aus E‑Mails und Moderations‑Tools zum Kommentar navigieren können.

#### WordPress

Wenn Sie WordPress verwenden, würden Sie beispielsweise die Artikel‑IDs in die To/From `URL ID`‑Felder im Migrations‑Tool eingeben, anstatt einer URL.