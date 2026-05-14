#### "Registration token not found, expired, or already used"

Token u vašem registracionom URL-u važi 30 minuta i može se iskoristiti samo jednom. Ako je vaš LMS uzeo duže od toga, ili je registracija ponovljena nakon uspeha, token će biti odbijen. Generišite novi URL na stranici FastComments LTI 1.3 Configuration i počnite ispočetka.

#### "Platform rejected registration"

Vaš LMS je odbio registracioni handshake. Najčešći uzroci:

- **Tool already registered with the same client name.** Neke platforme (posebno D2L) odbijaju drugu registraciju "FastComments" dok se prethodna ne obriše. Uklonite stari alat u svom LMS-u, pa pokušajte ponovo.
- **Wrong field in the LMS.** Uverite se da ste nalepili URL u polje **registration / tool initiation registration endpoint**, a ne u polje launch URL ili login URL.
- **The LMS doesn't actually support Dynamic Registration.** Starije verzije Moodle i Blackboard reklamiraju LTI 1.3 ali dozvoljavaju samo ručnu konfiguraciju. Proverite dokumentaciju svoje platforme.

#### "Failed to fetch platform configuration"

FastComments nije mogao da pročita openid-configuration dokument vašeg LMS-a. Ovo je retko i obično znači da je LMS dao neispravan ili nedostupan discovery URL. Obratite se podršci vašeg LMS-a.

#### Launch shows "Configuration not found"

Ili je konfiguracija u FastComments obrisana, ili je launch došao od `iss`/`client_id` para koji ne prepoznajemo. Ako ste obrisali pa ponovo registrovali, naložite svom LMS-u da ukloni i ponovo doda FastComments alat da bi dobio novi client_id.

#### Launch shows "Deployment not registered"

Pokrenuli ste FastComments iz Brightspace/Moodle/Blackboard deployment-a različitog od onog u kojem je prvi put pokrenut. FastComments zakači `deployment_id` pri prvom pokretanju kao bezbednosnu proveru. Da biste dodali novi deployment pod istim client-om, kontaktirajte podršku — dodaćemo deployment ID u konfiguraciju.

#### Launch shows "Unsupported message_type"

LMS je poslao LTI poruku koju FastComments ne obrađuje (npr. `LtiSubmissionReviewRequest`). FastComments podržava samo standardne resource-link launch i deep-linking tokove. Obratite nam se ako vam je potreban dodatak specifičnog tipa poruke.

#### Iframe doesn't resize

Većina LMS-ova automatski prilagođava veličinu LTI iframe-ova. Ako vaš ne radi to, proverite da li podešavanja pokretanja u LMS-u dozvoljavaju alatu da šalje postMessage događaje roditeljskom okviru. FastComments emituje i Canvas-style (`lti.frameResize`) i IMS-spec (`org.imsglobal.lti.frameResize`) resize poruke.