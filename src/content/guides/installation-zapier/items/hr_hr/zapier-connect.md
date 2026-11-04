## Povežite svoj račun

1. U Zapieru dodajte FastComments korak u Zap ili otvorite FastComments stranicu aplikacije u Zapier App Directory.
2. Odaberite **Sign in to FastComments**. Zapier prvo traži vašu regiju: odaberite **United States** osim ako je vaš
   račun stvoren na EU regiji (`eu.fastcomments.com`).
3. Otvorit će se FastComments prozor. Prijavite se ako već niste prijavljeni.
4. Pregledajte stranicu s privolom. Prikazuje Zapier aplikaciju, račun s kojim će se povezati i tražene
   dozvole (čitanje i pisanje). Odaberite **Approve**.
5. Zapier pohranjuje vezu i označava je nazivom vaše stranice i korisničkim imenom.

Veza koristi OAuth. API ključ se ne kopira u Zapier, a token koji Zapier drži radi samo za
račun koji ste odobrili.

## Tko može povezati

Osoba koja odobrava vezu mora biti **API admin** na FastComments računu. Vlasnici računa
imaju ovu dozvolu; drugim članovima tima može se dodijeliti na stranici Users. Netko tko je nema vidjet će
stranicu "nemate dozvolu" umjesto obrasca s privolom.

## Povezivanje ispravne stranice

Stranica s privolom povezuje račun na koji ste trenutno prijavljeni. Ako upravljate više računa,
prebacite se na ispravni putem prekidača računa prije odobravanja ili koristite poveznicu **switch account**
na stranici s privolom. Oznaka veze u Zapieru prikazuje naziv stranice, pa je pogrešan izbor lako uočiti.

## Pregled i opoziv pristupa

Svaka veza pojavljuje se pod **Connected Apps** u FastComments nadzornoj ploči, s dozvolama koje
ima i vremenom posljednje upotrebe. Opozivom tamo veza se odmah prekida; svaki Zap koji koristi tu
vezu staje dok se ne ponovno poveže. Vezu možete ukloniti i sa strane Zapiera pod
**My Apps**.