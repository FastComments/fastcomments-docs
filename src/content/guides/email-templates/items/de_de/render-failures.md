Da E-Mail-Vorlagen Variablen und Logik unterstützen, ist es möglich, Vorlagen zu erstellen
die nicht gerendert werden können, oder manchmal fehlschlagen.

Dies kann sehr frustrierend sein, dies zu diagnostizieren und zu debuggen, insbesondere wenn es sich um ein intermittierendes Problem handelt, oder
wenn es nur auftritt, wenn die Daten auf eine bestimmte Weise aussehen.

Um zu helfen, verfügt FastComments Email Templates über ein paar Funktionen:

1. Wenn die Vorlage nicht in der Vorschau gerendert werden kann, kann sie nicht gespeichert werden. Eine Fehlermeldung wird angezeigt.
2. Fehler beim Rendern von Vorlagen werden in der Admin-Oberfläche verfolgt und gemeldet.

Der zweite Punkt beschreibt Renderfehler, die in der Produktion auftreten. Das heißt, Sie erstellen eine Vorlage, die in der Vorschau
funktioniert – aber später aus irgendeinem Grund fehlschlägt. Zum Beispiel, wenn wir Folgendes in unserer Vorlage haben:

    <% if (comment.commenterEmail.includes('test') { %>

Dies kann manchmal fehlschlagen, wenn anonyme Kommentare aktiviert sind, da die E-Mail nicht immer
verfügbar sein wird. Wie erfahren wir also davon?

Die Antwort ist, dass Fehler an zwei Stellen sichtbar gemacht werden. Zuerst zeigt die Vorlagenliste selbst
bei jeder Vorlage eine Anzahl von Renderfehlern an.

Wenn man dann eine Vorlage ansieht, können wir für jeden Fehler eine Zählung sehen, wie oft die Vorlage
nicht gerendert werden konnte.

Neben jedem Fehler und seiner Zählung befindet sich eine Zurücksetzen-Schaltfläche, damit wir den Zähler
zurücksetzen können, nachdem wir ein Problem behoben haben.