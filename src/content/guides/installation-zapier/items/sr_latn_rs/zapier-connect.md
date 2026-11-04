## Povežite svoj nalog

1. U Zapier-u dodajte FastComments korak u Zap, ili otvorite FastComments stranicu aplikacije u Zapier App Directory.
2. Izaberite **Sign in to FastComments**. Zapier prvo traži vaš region: izaberite **United States** osim ako je vaš
   nalog kreiran na EU regionu (`eu.fastcomments.com`).
3. Otvara se FastComments prozor. Prijavite se ako već niste prijavljeni.
4. Pregledajte stranicu saglasnosti. Prikazuje Zapier aplikaciju, nalog sa kojim će biti povezana i
   tražene dozvole (čitanje i pisanje). Izaberite **Approve**.
5. Zapier čuva vezu i označava je imenom vašeg sajta i korisničkim imenom.

Veza koristi OAuth. API ključ se ne kopira u Zapier, a token koji Zapier drži funkcioniše samo za
nalog koji ste odobrili.

## Ko može da poveže

Osoba koja odobrava vezu mora biti **API admin** na FastComments nalogu. Vlasnici naloga
imaju ovu dozvolu; drugim članovima tima može biti dodeljena na stranici Users. Osoba bez nje vidi
stranicu "nemate dozvolu" umesto obrasca za saglasnost.

## Povezivanje pravog sajta

Stranica saglasnosti povezuje nalog na koji ste trenutno prijavljeni. Ako upravljate više naloga,
prebacite se na pravi preko prekidača naloga pre odobravanja, ili koristite link **switch account**
na stranici saglasnosti. Oznaka veze u Zapier-u prikazuje ime sajta, tako da je pogrešan izbor lako uočiti.

## Pregled i opoziv pristupa

Svaka veza se pojavljuje pod **Connected Apps** u FastComments kontrolnoj tabli, sa dozvolama koje
ima i vremenom poslednje upotrebe. Opozivom tamo se Zapier odmah prekida; svaki Zap koji koristi tu
vezu staje dok se ne poveže ponovo. Takođe možete ukloniti vezu sa Zapier strane pod
**My Apps**.