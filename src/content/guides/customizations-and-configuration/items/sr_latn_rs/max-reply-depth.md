[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

Podrazumevano, FastComments dozvoljava neograničeno ugnježđavanje odgovora, stvarajući strukturu teme gde korisnici mogu beskonačno odgovarati na odgovore.

Opcija maxReplyDepth vam omogućava da ograničite koliko duboko mogu ići nitovi odgovora. Kada se dostigne maksimalna dubina, korisnici više neće videti dugme za odgovor na komentarima na tom nivou.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

Kada je maxReplyDepth postavljen na 2:
- Korisnici mogu komentarisati na najvišem nivou (dubina 0)
- Korisnici mogu odgovarati na komentare na najvišem nivou (dubina 1)
- Korisnici mogu odgovarati na te odgovore (dubina 2)
- Dalji odgovori nisu dozvoljeni iznad dubine 2

Podešavanje na 1 bi dozvolilo samo odgovore na komentare sa najvišeg nivoa, stvarajući pliću strukturu diskusije.

Postavljanje maxReplyDepth na 0 bi onemogućilo sve odgovore, dozvoljavajući samo komentare na najvišem nivou. Ako nije navedeno, odgovori se mogu ugnježđavati bez ograničenja.