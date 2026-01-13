FastComments pruža nekoliko načina za isticanje novih komentara.

Prvo i najvažnije, podrazumevano komentari koji su pokrenuli in-app obaveštenje (odgovori, odgovori u istoj niti, ili komentari na stranici
na koju ste pretplaćeni), biće automatski istaknuti sa avatarom korisnika koji blago svetli. Boja se može prilagoditi putem CSS-a
koristeći `is-unread` klasu.

Komentari postavljeni u poslednja 24 sata imaju primenjenu `24hr` klasu koja se može koristiti za stilizovanje.

Na kraju, svi novi komentari u realnom vremenu koji se pojave u korisničkoj sesiji biće istaknuti na nekoliko sekundi putem animacije. Ovo se radi putem
`is-live` CSS klase i takođe se može prilagoditi.