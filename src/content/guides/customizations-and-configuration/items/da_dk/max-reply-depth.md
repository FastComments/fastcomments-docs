[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

Som standard tillader FastComments ubegrænset indlejring af svar, hvilket skaber en trådstruktur, hvor brugere kan svare på svar på ubestemt tid.

Indstillingen maxReplyDepth giver dig mulighed for at begrænse, hvor dybt svartråde kan gå. Når den maksimale dybde er nået, vil brugerne ikke længere se en svar-knap på kommentarer på det niveau.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

Med maxReplyDepth sat til 2:
- Brugere kan kommentere på øverste niveau (dybde 0)
- Brugere kan svare på kommentarer på øverste niveau (dybde 1)
- Brugere kan svare på disse svar (dybde 2)
- Ingen yderligere svar er tilladt ud over dybde 2

At sætte maxReplyDepth til 1 vil kun tillade svar på kommentarer på øverste niveau, hvilket skaber en fladere diskussionsstruktur.

At sætte maxReplyDepth til 0 vil deaktivere alle svar og kun tillade kommentarer på øverste niveau. Hvis ikke angivet, kan svar indlejres uden begrænsning.