Ker e-poštne predloge podpirajo spremenljivke in logiko, je mogoče ustvariti predloge,
ki se ne uspejo upodobiti, ali se včasih ne uspejo upodobiti.

To je lahko zelo frustrirajuče za diagnosticiranje in odpravljanje napak, še posebej, če gre za občasno težavo ali
če se pojavi samo, ko podatki izgledajo na določen način.

Da bi pomagali, ima FastComments Email Templates nekaj funkcij:

1. Če se predloga pri predogledu ne uspe upodobiti, je ni mogoče shraniti. Prikazano bo sporočilo o napaki.
2. Napake pri upodabljanju predlog so spremljane in poročane v upravljalskem vmesniku.

Druga točka opisuje napake pri upodabljanju, ki se zgodijo v produkciji. Torej, ustvarite predlogo, ki se v predogledu
prikaže brez težav - vendar pozneje iz neznanega razloga ne uspe. Na primer, če imamo v naši predlogi to:

    <% if (comment.commenterEmail.includes('test') { %>

To se včasih lahko zatakne, če imamo omogočeno anonimno komentiranje, saj e-pošta ne bo vedno
na voljo. Kako torej izvemo za to?

Odgovor je, da se napake pokažejo na dveh mestih. Najprej seznam predlog sam
prikaže število napak pri upodabljanju za vsako predlogo.

Ko nato gledamo posamezno predlogo, lahko vidimo število, za vsako napako posebej, kolikokrat se je predloga
neuspešno upodobila.

Gumb za ponastavitev se nahaja zraven vsake napake in njenega števila, tako da lahko ponastavimo števec
potem ko smo težavo odpravili.