#### "Registracijski token nije pronađen, istekao ili je već upotrijebljen"

Token u vašem registracijskom URL-u (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">preuzmite ga ovdje</a>) vrijedi 30 minuta i može se koristiti samo jednom. Ako je vaš LMS trebao više vremena od toga, ili je registracija pokušana ponovno nakon uspjeha, token će biti odbijen. Generirajte novi URL na FastComments LTI 1.3 Configuration stranici i počnite ispočetka.

#### "Platform rejected registration"

Vaš LMS je odbio registracijski handshake. Najčešći uzroci:

- **Tool already registered with the same client name.** Neke platforme (posebice D2L) odbijaju drugu registraciju "FastComments" dok prethodna nije izbrisana. Uklonite stari alat u vašem LMS-u, pa pokušajte ponovno.
- **Wrong field in the LMS.** Uvjerite se da ste zalijepili URL u polje **registration / tool initiation registration endpoint**, a ne u polje launch URL ili login URL.
- **The LMS doesn't actually support Dynamic Registration.** Starije verzije Moodla i Blackboarda oglašavaju LTI 1.3, ali dopuštaju samo ručnu konfiguraciju. Provjerite dokumentaciju vaše platforme.

#### "Failed to fetch platform configuration"

FastComments nije mogao pročitati openid-configuration dokument vašeg LMS-a. To je rijetko i obično znači da je LMS dao neispravan ili nedostupan discovery URL. Obratite se podršci vašeg LMS-a.

#### Launch shows "Configuration not found"

Ili je konfiguracija u FastComments izbrisana, ili je pokretanje došlo s `iss`/`client_id` parom kojeg ne prepoznajemo. Ako ste izbrisali i ponovno registrirali, uputite vaš LMS da ukloni i ponovo doda alat FastComments kako bi dobio novi client_id.

#### Launch shows "Deployment not registered"

Pokrenuli ste FastComments iz Brightspace/Moodle/Blackboard deploymenta različitog od onog u kojem je prvobitno pokrenut. FastComments "pins" the `deployment_id` on first launch as a security check. Za dodavanje nove implementacije pod istim klijentom, kontaktirajte podršku — mi ćemo dodati deployment ID u konfiguraciju.

#### Launch shows "Unsupported message_type"

LMS je poslao LTI poruku koju FastComments ne obrađuje (npr. `LtiSubmissionReviewRequest`). FastComments podržava samo standardno resource-link pokretanje i deep-linking tokove. Obratite nam se ako trebate da se doda određeni tip poruke.

#### Iframe doesn't resize

Većina LMS-a automatski prilagođava veličinu LTI iframeova. Ako to vaš ne radi, provjerite dopuštaju li postavke pokretanja LMS-a da alat šalje postMessage događaje roditeljskom okviru. FastComments emitira resize poruke u oba formata: Canvas-style (`lti.frameResize`) i IMS-spec (`org.imsglobal.lti.frameResize`).

---