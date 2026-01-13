[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

Privzeto FastComments dovoljuje neomejeno gnezdenje odgovorov, kar ustvarja strukturo pogovora, kjer lahko uporabniki neomejeno odgovarjajo na odgovore.

Možnost maxReplyDepth vam omogoča omejiti, kako globoko se lahko razvežejo nitke odgovorov. Ko je dosežena največja globina, uporabniki na komentarjih na tej ravni ne bodo več videli gumba za odgovor.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

Ob nastavitvi maxReplyDepth na 2:
- Uporabniki lahko komentirajo na najvišji ravni (globina 0)
- Uporabniki lahko odgovarjajo na komentarje na najvišji ravni (globina 1)
- Uporabniki lahko odgovarjajo na te odgovore (globina 2)
- Nadaljnji odgovori onkraj globine 2 niso dovoljeni

Nastavitev na 1 bi dovolila le odgovore na komentarje na najvišji ravni, kar ustvari bolj plosko strukturo razprave.

Nastavitev maxReplyDepth na 0 bi onemogočila vse odgovore in dovolila le komentarje na najvišji ravni. Če ni določeno, so odgovori lahko gnezdeni brez omejitve.

[code-example-end]