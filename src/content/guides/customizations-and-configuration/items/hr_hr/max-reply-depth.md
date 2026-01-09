[related-parameter-start name = 'maxReplyDepth'; type = 'number'; related-parameter-end]

Prema zadanim postavkama, FastComments dopušta neograničeno ugnježđivanje odgovora, stvarajući strukturu niti u kojoj korisnici mogu neograničeno odgovarati jedni na druge.

Opcija maxReplyDepth omogućuje vam ograničavanje koliko duboko niti odgovora mogu ići. Kada se postigne maksimalna dubina, korisnici više neće vidjeti gumb za odgovor na komentarima na toj razini.

[code-example-start config = {maxReplyDepth: 2}; linesToHighlight = [6]; title = 'Limiting Reply Depth to 2 Levels'; code-example-end]

With maxReplyDepth set to 2:
- Korisnici mogu komentirati na najvišoj razini (dubina 0)
- Korisnici mogu odgovarati na komentare na najvišoj razini (dubina 1)
- Korisnici mogu odgovarati na te odgovore (dubina 2)
- Nisu dopušteni daljnji odgovori iznad dubine 2

Postavljanje na 1 dopuštalo bi samo odgovore na komentare na najvišoj razini, stvarajući plitkiju strukturu rasprave.

Postavljanje maxReplyDepth na 0 onemogućilo bi sve odgovore, dopuštajući samo komentare na najvišoj razini. Ako nije navedeno, odgovori se mogu ugnježđivati bez ograničenja.