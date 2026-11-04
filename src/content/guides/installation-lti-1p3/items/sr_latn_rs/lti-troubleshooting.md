#### "Registracioni token nije pronađen, istekao je ili je već iskorišćen"

Token u vašem registracionom URL-u (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">preuzmite ga ovde</a>) važi 30 minuta i može se iskoristiti samo jednom. Ako je vašem LMS-u trebalo duže od toga, ili ako je registracija ponovljena nakon uspeha, token će biti odbijen. Generišite nov URL na stranici FastComments LTI 1.3 Configuration i počnite iznova.

#### "Platforma je odbila registraciju"

Vaš LMS je odbio registracioni handshake. Najčešći uzroci:

- **Alat je već registrovan sa istim imenom klijenta.** Neke platforme (posebno D2L) odbijaju drugu registraciju "FastComments" dok se prethodna ne obriše. Uklonite stari alat u vašem LMS-u, a zatim pokušajte ponovo.
- **Pogrešno polje u LMS-u.** Proverite da ste URL nalepili u polje **registration / tool initiation registration endpoint**, a ne u polje za launch URL ili login URL.
- **LMS zapravo ne podržava Dynamic Registration.** Starije verzije Moodle-a i Blackboard-a deklarišu LTI 1.3, ali dozvoljavaju samo ručnu konfiguraciju. Proverite dokumentaciju vaše platforme.

#### "Nije uspelo preuzimanje konfiguracije platforme"

FastComments nije uspeo da pročita openid-configuration dokument vašeg LMS-a. Ovo je retko i obično znači da je LMS dostavio neispravan ili nedostupan discovery URL. Kontaktirajte podršku vašeg LMS-a.

#### Prilikom pokretanja se prikazuje "Configuration not found"

Ili je konfiguracija u FastComments obrisana, ili je pokretanje došlo iz `iss`/`client_id` para koji ne prepoznajemo. Ako ste obrisali i ponovo registrovali, uputite svoj LMS da ukloni i ponovo doda FastComments alat kako bi dobio novi client_id.

#### Prilikom pokretanja se prikazuje "Deployment not registered"

Pokrenuli ste FastComments iz Brightspace/Moodle/Blackboard deployment-a koji se razlikuje od onog u kome je prvi put pokrenut. FastComments pamti `deployment_id` pri prvom pokretanju kao bezbednosnu proveru. Da biste dodali novi deployment pod istim klijentom, kontaktirajte podršku - dodaćemo deployment ID u konfiguraciju.

#### Prilikom pokretanja se prikazuje "Unsupported message_type"

LMS je poslao LTI poruku koju FastComments ne obrađuje (npr. `LtiSubmissionReviewRequest`). FastComments podržava samo standardni resource-link launch i deep-linking tokove. Javite nam ako vam je potreban određeni tip poruke.

#### Iframe se ne menja u veličini

Većina LMS-ova automatski podešava veličinu LTI iframe-ova. Ako vaš to ne radi, proverite da podešavanja pokretanja LMS-a dozvoljavaju alatu da šalje postMessage događaje roditeljskom frejmu. FastComments šalje i Canvas-style (`lti.frameResize`) i IMS-spec (`org.imsglobal.lti.frameResize`) poruke za promenu veličine.