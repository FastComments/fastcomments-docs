Kada korisnici komentarišu ili glasaju, a nisu prijavljeni, biće zatraženo da unesu svoj email i korisničko ime.

Za neke sajtove, traženje od posetioca da smišlja jedinstveno korisničko ime predstavlja prepreku, posebno na mobilnim uređajima. FastComments može
generisati neutralno korisničko ime za svakog novog posetioca i unapred popuniti polje za korisničko ime, kao `BraveOtter4172`.

Posetilac može ostaviti ime kako je, ili ga zameniti imenom po svom izboru.

Ovo se može omogućiti iz UI prilagođavanja, pod podešavanjem pod nazivom `Generate Usernames Automatically`:

[app-screenshot-start url='/auth/my-account/customize-widget/new'; selector = '.auto-generate-username'; alt='Opcija Generiši korisnička imena automatski u UI prilagođavanja widgeta'; title='Generiši korisnička imena automatski' app-screenshot-end]

#### Kako se ponaša

- Svako generisano ime je jedinstveno. Proverava se protiv postojećih naloga i rezerviše za sesiju pregledača tog posetioca, tako da dva posetioca ne dobiju isto ime.
- Ime se generiše samo za posetioce koji još nemaju ime. Prijavljeni korisnici, SSO korisnici i posetioci koji su već komentarisali zadržavaju svoje postojeće ime.
- Radi sa ili bez [anonymous commenting](/guide-customizations-and-configuration.html#allow-anon). Kada je anonimno komentarisanje isključeno, posetilac i dalje unosi svoj email, ali više ne mora da razmišlja o korisničkom imenu.
- Povratni posetilac koji unese email koji je ranije koristio biće povezan sa svojim postojećim nalogom i zadržaće ime na tom nalogu.
- Ako je takođe postavljen [Default Username](/guide-customizations-and-configuration.html#default-username), generisano ime ima prednost.