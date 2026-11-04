#### "Registracijski token nije pronađen, istekao je ili je već iskorišten"

Token u vašem registracijskom URL-u (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">nabavite ga ovdje</a>) vrijedi 30 minuta i može se koristiti samo jednom. Ako je vašem LMS-u trebalo duže, ili je registracija ponovljena nakon uspjeha, token će biti odbijen. Generirajte novi URL na stranici FastComments LTI 1.3 Configuration i počnite ispočetka.

#### "Platforma je odbila registraciju"

Vaš LMS odbio je registracijski handshake. Najčešći uzroci:

- **Alat je već registriran s istim nazivom klijenta.** Neke platforme (posebno D2L) odbijaju drugu registraciju "FastComments" dok se prethodna ne izbriše. Uklonite stari alat u svom LMS-u, a zatim pokušajte ponovno.
- **Pogrešno polje u LMS-u.** Provjerite jeste li URL zalijepili u polje **registration / tool initiation registration endpoint**, a ne u polje za launch URL ili login URL.
- **LMS zapravo ne podržava Dynamic Registration.** Starije verzije Moodlea i Blackboarda deklariraju LTI 1.3, ali dopuštaju samo ručnu konfiguraciju. Provjerite dokumentaciju svoje platforme.

#### "Nije moguće dohvatiti konfiguraciju platforme"

FastComments nije mogao pročitati openid-configuration dokument vašeg LMS-a. To je rijetko i obično znači da je LMS pružio neispravan ili nedostupan discovery URL. Obratite se podršci svog LMS-a.

#### Prilikom pokretanja prikazuje se "Configuration not found"

Ili je konfiguracija u FastComments izbrisana, ili je pokretanje došlo iz para `iss`/`client_id` koji ne prepoznajemo. Ako ste izbrisali i ponovno registrirali, uputite svoj LMS da ukloni i ponovno doda FastComments alat kako bi dobio novi client_id.

#### Prilikom pokretanja prikazuje se "Deployment not registered"

Pokrenuli ste FastComments iz Brightspace/Moodle/Blackboard deploymenta koji se razlikuje od onog u kojem je prvi put pokrenut. FastComments fiksira `deployment_id` pri prvom pokretanju kao sigurnosnu provjeru. Da biste dodali novi deployment pod istim klijentom, obratite se podršci - dodat ćemo deployment ID u konfiguraciju.

#### Prilikom pokretanja prikazuje se "Unsupported message_type"

LMS poslao je LTI poruku koju FastComments ne obrađuje (npr. `LtiSubmissionReviewRequest`). FastComments podržava samo standardni resource-link launch i deep-linking tokove. Javite nam ako trebate dodati određenu vrstu poruke.

#### Iframe se ne mijenja u veličini

Većina LMS-ova automatski prilagođava veličinu LTI iframeova. Ako vaš to ne radi, provjerite dopuštaju li postavke pokretanja LMS-a alatu slanje postMessage događaja nadređenom okviru. FastComments šalje i Canvas-stil (`lti.frameResize`) i IMS-spec (`org.imsglobal.lti.frameResize`) poruke za promjenu veličine.