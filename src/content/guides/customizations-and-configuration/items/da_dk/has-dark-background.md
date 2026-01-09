[related-parameter-start name = 'hasDarkBackground'; type = 'boolean'; related-parameter-end]

Som standard vil FastComments kommentar-widget automatisk opdage mørk tilstand på de fleste websteder.

Når mørk tilstand opdages, skifter FastComments fra sort tekst på hvid baggrund til hvid tekst på sort baggrund. Billeder vil også ændre sig.

Når siden indlæses, vil widget'en forsøge at bestemme, hvor mørk baggrunden på siden er bag kommentarfeltet. Det betyder, at
siden kan have en hvid baggrund, men hvis du placerer kommentarfeltet inde i en container med en sort baggrund, bør mørk tilstand
stadig automatisk blive aktiveret for at gøre kommentarerne læsbare.

Dog kan detektionsmekanismen, som er afhængig af at bestemme "luminans", muligvis ikke aktivere mørk tilstand, når du ønsker det. For at tvinge den til at aktivere mørk tilstand, sæt
*hasDarkBackground*-flaget til true som følger:

[code-example-start config = {hasDarkBackground: true}; linesToHighlight = [6]; title = 'Force Dark Background Mode'; additionalDemoCode = '<style>body { background: black; }</style>'; code-example-end]

---