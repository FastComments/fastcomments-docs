[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

Standaard staat FastComments onbeperkte nesteling van antwoorden toe, waardoor er een draadstructuur ontstaat waarin gebruikers oneindig op antwoorden kunnen reageren.

De optie maxReplyDepth stelt je in staat te beperken hoe diep antwoorddraden kunnen worden. Wanneer de maximale diepte is bereikt, zien gebruikers geen antwoordknop meer bij opmerkingen op dat niveau.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

Met maxReplyDepth ingesteld op 2:
- Gebruikers kunnen op het hoogste niveau een reactie plaatsen (diepte 0)
- Gebruikers kunnen antwoorden op opmerkingen op het hoogste niveau (diepte 1)
- Gebruikers kunnen reageren op die antwoorden (diepte 2)
- Geen verdere antwoorden zijn toegestaan voorbij diepte 2

Instellen op 1 zou alleen antwoorden op opmerkingen op het hoogste niveau toestaan, waardoor een plattere discussiestructuur ontstaat.

Het instellen van maxReplyDepth op 0 zou alle antwoorden uitschakelen, waardoor alleen opmerkingen op het hoogste niveau mogelijk zijn. Als het niet is opgegeven, kunnen antwoorden onbeperkt genest worden.