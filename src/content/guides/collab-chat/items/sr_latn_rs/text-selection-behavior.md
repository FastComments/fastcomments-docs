### Kako funkcioniše izbor teksta

Kada korisnici izaberu tekst unutar kontejnera Collab Chat, vidžet zabeleži taj izbor i omogućava im da započnu diskusiju. Izbor može biti mali kao jedna reč ili obilniji kao više pasusa koji obuhvataju različite elemente.

### Odgoda pri izboru

Na desktop uređajima postoji odgoda od 3,5 sekunde između trenutka kada korisnik izabere tekst i pojave upita za diskusiju. Ovo sprečava treperenje UI-ja kada korisnici samo ističu tekst da bi ga kopirali ili pročitali. Na mobilnim uređajima upit se pojavljuje odmah jer je izbor teksta na touch ekranima obično namerniji.

### Jedinstveni ID-evi konverzacija

Svaka konverzacija dobija jedinstveni `urlId` koji kombinuje URL stranice, DOM putanju elementa i serijalizovani opseg teksta. Ovo osigurava da svaki izbor teksta kreira posebnu konverzaciju koju kasnije možete ponovo pronaći.

Ako u svojoj konfiguraciji obezbedite prilagođeni `urlId`, on će biti kombinovan sa element path i text range da bi se kreirao finalni identifikator.

### Vizuelna isticanja

Kada za određeni izbor teksta postoji diskusija, taj tekst dobija vizuelno isticanje. Isticanje je realizovano korišćenjem pozadinskih boja i pojavljuje se pri prelasku miša ili kada je povezani chat prozor otvoren.

Sistem isticanja radi tako što obavije izabrani tekst u poseban element koji se može stilizovati. Ovakav pristup obezbeđuje da isticanja ostanu precizna čak i kada je osnovna HTML struktura kompleksna.

### Pozicioniranje chat prozora

Kada korisnik klikne na isticanje ili kreira novu anotaciju, chat prozor se pojavljuje u blizini izabranog teksta. Vidžet automatski izračunava najbolju poziciju za ovaj prozor na osnovu raspoloživog prostora u viewportu.

Sistem pozicioniranja koristi CSS klase kao što su `to-right`, `to-left`, `to-top` i `to-bottom` da označi u kom pravcu bi chat prozor trebalo da se proširi od isticanja. Na mobilnim uređajima (ekrani manje od 768px širine), chat prozor se uvek prikazuje preko celog ekrana radi bolje upotrebljivosti.

### Dimenzije chat prozora

Chat prozori su široki 410px na desktopu sa razmakom od 20px i vizuelnom strelicom od 16px koja pokazuje ka istaknutom tekstu. Ove dimenzije su fiksne kako bi se obezbedilo dosledno ponašanje, ali izgled možete prilagoditi pomoću CSS-a.

### Selekcija preko više elemenata

Korisnici mogu izabrati tekst koji obuhvata više HTML elemenata, na primer isticanje od sredine jednog pasusa do početka drugog. Sistem serijalizacije opsega pravilno obrađuje ovakve situacije i istaknuće sav izabrani tekst čak i preko granica elemenata.

### Kompatibilnost sa pregledačima

Sistem za izbor teksta koristi standardni `window.getSelection()` API koji je podržan u svim modernim pregledačima. Za starije verzije Internet Explorera koristi se fallback na `document.selection` radi kompatibilnosti.

### Očuvanje selekcije

Kada se za izbor teksta kreira konverzacija, ta anotacija ostaje čak i ako se stranica ponovo učita. Serijalizovani opseg i DOM putanja omogućavaju vidžetu da obnovi isticanja na tačno istom mestu kada se korisnici vrate na stranicu.

Ovo funkcioniše pouzdano sve dok sadržaj vaše stranice ostane stabilan. Ako promenite tekstualni sadržaj ili restrukturirate svoj HTML, postojeće anotacije možda više neće biti pravilno poravnate sa tekstom. Iz tog razloga, najbolje je izbegavati velike promene sadržaja na stranicama sa aktivnim anotacijama, ili razmotriti migraciju anotacija kada su promene sadržaja neophodne.