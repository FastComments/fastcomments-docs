Budući da e-mail predlošci podržavaju varijable i logiku, moguće je stvoriti predloške koji neće uspješno renderirati, ili ponekad ne uspiju renderirati.

To može biti vrlo frustrirajuće za dijagnosticirati i otkloniti pogreške, posebno ako je riječ o povremenom problemu, ili ako se pojavljuje samo kada podaci izgledaju na određeni način.

Da bi pomoglo, FastComments Email Templates ima nekoliko značajki:

1. Ako predložak ne uspije u pretpregledu, ne može se spremiti. Prikazat će se poruka o pogrešci.
2. Kvarovi pri renderiranju predložaka prate se i prikazuju u administratorskom sučelju.

Druga točka opisuje kvarove pri renderiranju koji se događaju u produkciji. Kao u slučaju da stvorite predložak koji se u pretpregledu prikazuje ispravno — ali kasnije iz nekog razloga ne uspije. Na primjer, ako u našem predlošku imamo ovo:

    <% if (comment.commenterEmail.includes('test') { %>

Ovo ponekad može ne uspjeti ako imamo omogućeno anonimno komentiranje, budući da e-mail adresa neće uvijek biti dostupna. Pa kako to saznati?

Odgovor je da se pogreške prikazuju na dva mjesta. Prvo, sam popis predložaka pokazuje broj pogrešaka pri renderiranju uz svaki predložak.

Zatim, kada pregledavamo predložak, vidimo broj, po svakoj pogrešci, koliko puta je predložak zakazao pri renderiranju.

Gumb za resetiranje nalazi se pored svake pogreške i njenog broja, kako bismo mogli resetirati brojač nakon što smo riješili problem.