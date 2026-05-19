#### "Token za registraciju nije pronađen, istekao je ili je već iskorišćen"

Token u vašem registracionom URL-u (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">get it here</a>) važi 30 minuta i može se iskoristiti samo jednom. Ako je vaš LMS potrošio više vremena od toga, ili je registracija ponovljena nakon uspeha, token će biti odbijen. Generišite novi URL na FastComments LTI 1.3 Configuration stranici i počnite ispočetka.

#### "Platform rejected registration"

Vaš LMS je odbio registracioni handshake. Najčešći uzroci:

- **Tool already registered with the same client name.** Neke platforme (posebno D2L) odbijaju drugu registraciju "FastComments" dok prethodna nije izbrisana. Uklonite stari alat u vašem LMS-u, zatim pokušajte ponovo.
- **Wrong field in the LMS.** Uverite se da ste zalepili URL u polje **registration / tool initiation registration endpoint**, a ne u polje launch URL ili login URL.
- **The LMS doesn't actually support Dynamic Registration.** Starije verzije Moodle-a i Blackboard-a oglašavaju LTI 1.3, ali dozvoljavaju samo ručnu konfiguraciju. Proverite dokumentaciju vaše platforme.

#### "Failed to fetch platform configuration"

FastComments nije mogao da pročita openid-configuration dokument vašeg LMS-a. Ovo je retko i obično znači da je LMS obezbedio neispravan ili nedostupan discovery URL. Kontaktirajte podršku vašeg LMS-a.

#### Launch shows "Configuration not found"

Ili je konfiguracija u FastComments obrisana, ili je pokretanje došlo od `iss`/`client_id` para koji ne prepoznajemo. Ako ste obrisali i ponovo registrovali, naložite vašem LMS-u da ukloni i ponovo doda FastComments alat kako bi dobio novi client_id.

#### Launch shows "Deployment not registered"

Pokrenuli ste FastComments iz Brightspace/Moodle/Blackboard deployment-a različitog od onog u kojem je prvi put pokrenut. FastComments zakači `deployment_id` pri prvom pokretanju kao bezbednosnu proveru. Da biste dodali novi deployment pod istim klijentom, kontaktirajte podršku — dodaćemo deployment ID u konfiguraciju.

#### Launch shows "Unsupported message_type"

LMS je poslao LTI poruku koju FastComments ne obrađuje (npr. `LtiSubmissionReviewRequest`). FastComments podržava samo standardni resource-link launch i deep-linking tokove. Obratite nam se ako vam je potreban dodatak specifičnog tipa poruke.

#### Iframe doesn't resize

Većina LMS-ova automatski prilagođava veličinu LTI iframe-ova. Ako vaš to ne radi, proverite da li podešavanja pokretanja u LMS-u omogućavaju alatu da šalje postMessage događaje roditeljskom okviru. FastComments emituje i Canvas-style (`lti.frameResize`) i IMS-spec (`org.imsglobal.lti.frameResize`) poruke za promenu veličine.