Pošto su linkovi za prijavu u suštini lozinke, bezbednost shvatamo veoma ozbiljno.

Svi linkovi za prijavu u našem sistemu podešeni su da isteknu nakon određenog vremenskog perioda, i takođe imamo mehanizme na mestu za otkrivanje
pogađanja linka za prijavu. Neki linkovi za prijavu su podeljeni na više lozinki, i ako jedna bude pogođena,
ostale će biti poništene.

### Bezbednost u poređenju sa lozinkama

U većini sistema koji zahtevaju lozinku, možete proći kroz mehanizam za zaboravljenu lozinku
ako imate korisnikov email. To znači, ako imate pristup korisnikovom email nalogu,
nije važno da li sistem pod napadom koristi lozinke ili magične linkove.

### Upozorenja pri prijavi s novog IP-a

Kada dođe do prijave sa IP adrese koja ranije nije viđena za određeni nalog, FastComments šalje email sa sigurnosnim upozorenjem
sa približnom lokacijom i IP adresom. Ovo pomaže korisnicima da otkriju neovlašćeni pristup. Imajte na umu da FastComments ne čuva
neobrađene IP adrese — čuva se samo obfuskovani oblik iz bezbednosnih razloga.

### Bezbednost u poređenju sa MFA

Linkovi za prijavu su manje bezbedni od MFA. FastComments sada podržava dvofaktorsku autentifikaciju (2FA)
za administratorske naloge kako bi obezbedio povećanu bezbednost. Kada je 2FA omogućen, on je obavezan čak i pri korišćenju linkova za prijavu.