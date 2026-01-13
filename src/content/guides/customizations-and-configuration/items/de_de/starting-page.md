[related-parameter-start name = 'startingPage'; type = 'number'; related-parameter-end]

Beim Abrufen und Anzeigen von Kommentaren muss das Kommentar-Widget wissen, auf welcher Seite es beginnen soll. Standardmäßig beginnt es mit der ersten Seite und zeigt nur diese Seite an.

Wenn gewünscht, kann die genaue zu rendende Seite dem Kommentar-Widget als Einstellung *startingPage* übergeben werden.

[code-example-start config = {startingPage: 1}; linesToHighlight = [6]; title = 'Specifying The Page to Render'; code-example-end]

Beachte, dass die Seitennummern bei null beginnen, daher zeigt das obige Beispiel die zweite Seite an.

---