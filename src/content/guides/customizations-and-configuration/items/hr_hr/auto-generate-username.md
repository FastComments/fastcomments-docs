Kada korisnici komentiraju ili glasaju, a nisu prijavljeni, bit će zatraženo da unesu svoju e‑mail adresu i korisničko ime.

Za neke web‑stranice, traženje od posjetitelja da smišljaju jedinstveno korisničko ime predstavlja prepreku, osobito na mobilnim uređajima. FastComments može generirati neutralno korisničko ime za svakog novog posjetitelja i unaprijed ga popuniti u polje za korisničko ime, poput `BraveOtter4172`.

Posjetitelj može ostaviti to kako jest, ili ga zamijeniti imenom po svom izboru.

Ovo se može omogućiti u sučelju za prilagodbu, pod postavkom pod nazivom `Generate Usernames Automatically`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='Opcija Generiraj korisnička imena automatski u sučelju za prilagodbu widgeta'; title='Generiraj korisnička imena automatski' app-screenshot-end]

#### Kako se ponaša

- Svako generirano ime je jedinstveno. Provjerava se protiv postojećih računa i rezervira za sesiju preglednika tog posjetitelja, tako da dva posjetitelja ne dobiju isto ime.
- Ime se generira samo za posjetitelje koji ga još nemaju. Prijavljeni korisnici, SSO korisnici i posjetitelji koji su već komentirali zadržavaju svoje postojeće ime.
- Radi s ili bez [anonymous commenting](/guide-customizations-and-configuration.html#allow-anon). Kada je anonimno komentiranje isključeno, posjetitelj i dalje unosi svoju e‑mail adresu, ali više ne mora razmišljati o korisničkom imenu.
- Povratni posjetitelj koji unese e‑mail koji je prethodno koristio povezuje se s postojećim računom i zadržava ime na tom računu.
- Ako je također postavljeno [Default Username](/guide-customizations-and-configuration.html#default-username), generirano ime ima prednost.