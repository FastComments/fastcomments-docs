[related-parameter-start name = 'newCommentsToBottom'; type = 'boolean'; related-parameter-end]

Som standard vises nye live-kommentarer øverst i kommentarliste, når de postes i realtid.

Når denne indstilling er aktiveret, tilføjes nye live-kommentarer i stedet til bunden af listen. Dette påvirker, hvordan kommentarer vises, når de postes live, mens brugere ser kommentart tråden.

[code-example-start config = {newCommentsToBottom: true}; linesToHighlight = [6]; title = 'New Live Comments to Bottom'; code-example-end]

Når denne indstilling er aktiveret:
- Nye live-kommentarer, som postes af andre brugere, vises nederst i kommentarlisten
- Brugere vil se nye kommentarer dukke op under eksisterende kommentarer i realtid
- Dette påvirker kun live-opdateringer af kommentarer – ikke den oprindelige sideindlæsning
- Dette kan hjælpe med at bevare læseflowet, når brugere følger en diskussion

Bemærk, at denne indstilling kun påvirker, hvor nye live-kommentarer placeres, når de ankommer i realtid. Den påvirker ikke den oprindelige sorteringsrækkefølge, når siden indlæses.