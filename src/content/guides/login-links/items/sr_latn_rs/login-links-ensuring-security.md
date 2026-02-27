---
Pošto su linkovi za prijavu u suštini lozinke, ozbiljno shvatamo bezbednost.

Svi linkovi za prijavu u našem sistemu podešeni su da isteknu nakon određenog vremenskog perioda, i takođe imamo mehanizme za otkrivanje pogađanja linka za prijavu. Neki linkovi za prijavu su podeljeni na više lozinki, i ako se jedna pogodi, druge će biti poništene.

### Bezbednost u poređenju sa lozinkama

U većini sistema koji zahtevaju lozinku, možete proći kroz mehanizam „Zaboravljena lozinka“ ako imate korisnikovu email adresu. To znači da, ako imate pristup korisnikovom email nalogu, nije bitno da li sistem pod napadom koristi lozinke ili magične linkove.

### Upozorenja o prijavi sa novog IP-a

Kada prijava nastupi sa IP adrese koja ranije nije viđena za određeni nalog, FastComments šalje mejl sa bezbednosnim upozorenjem sa približnom lokacijom i IP adresom. Ovo pomaže korisnicima da otkriju neovlašćen pristup. Imajte na umu da FastComments ne čuva sirove IP adrese — čuva se samo obfuskovani oblik u svrhu bezbednosti.

### Rezervni email za oporavak naloga

Ako izgubite pristup svom primarnom emailu, možete koristiti verificirani rezervni email za oporavak naloga. Vaš rezervni email radi sa svim tokovima prijave. Možete ga uneti na stranici „Zaboravljeno korisničko ime“, koristiti ga za prijavu putem magičnog linka, ili ga upisati u polje za korisničko ime/email prilikom prijave sa lozinkom.

Da biste podesili rezervni email, idite na [Detalji naloga](https://fastcomments.com/auth/my-account/edit-details) i kliknite **Definiši rezervni email**. Vaš rezervni email se koristi isključivo za oporavak naloga i neće primati obaveštenja.

### Bezbednost u poređenju sa MFA

Linkovi za prijavu su manje sigurni od višefaktorske autentifikacije (MFA). FastComments sada podržava dvofaktorsku autentifikaciju (2FA) za administratorske naloge radi poboljšane bezbednosti. Kada je 2FA omogućena, obavezna je čak i pri korišćenju linkova za prijavu.

---