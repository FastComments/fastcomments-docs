---
#### "Registracijski token nije pronađen, istekao je ili je već upotrijebljen"

Token u vašem URL‑u za registraciju vrijedi 30 minuta i može se upotrijebiti samo jednom. Ako je vaš LMS trebao više vremena, ili je registracija ponovno pokušana nakon što je već uspjela, token će biti odbijen. Generirajte novi URL na stranici konfiguracije FastComments LTI 1.3 i počnite ispočetka.

#### "Platforma je odbila registraciju"

Vaš LMS je odbio postupak rukovanja registracijom. Najčešći uzroci:

- **Alat je već registriran s istim imenom klijenta.** Neke platforme (osobito D2L) odbijaju drugu registraciju "FastComments" dok se prethodna ne obriše. Uklonite stari alat u svom LMS‑u, a zatim pokušajte ponovno.
- **Pogrešno polje u LMS‑u.** Provjerite jeste li zalijepili URL u polje **registration / tool initiation registration endpoint**, a ne u polje launch URL ili login URL.
- **LMS zapravo ne podržava Dynamic Registration.** Starije verzije Moodlea i Blackboarda oglašavaju LTI 1.3, ali dopuštaju samo ručnu konfiguraciju. Pogledajte dokumentaciju svoje platforme.

#### "Neuspjelo dohvaćanje konfiguracije platforme"

FastComments nije uspio pročitati openid-configuration dokument vašeg LMS‑a. To je rijetko i obično znači da je LMS dao nepravilno oblikovan ili nedostupan discovery URL. Obratite se podršci vašeg LMS‑a.

#### Launch shows "Configuration not found"

Ili je konfiguracija u FastComments izbrisana, ili je pokretanje stiglo iz `iss`/`client_id` para koji ne prepoznajemo. Ako ste izbrisali i ponovno registrirali, uputite svoj LMS da ukloni i ponovo doda FastComments alat kako bi dobio novi client_id.

#### Launch shows "Deployment not registered"

Pokrenuli ste FastComments iz Brightspace/Moodle/Blackboard deploymenta različitog od onog u kojem je prvi put pokrenut. FastComments pri prvom pokretanju „pin‑a” `deployment_id` kao sigurnosnu provjeru. Da biste dodali novi deployment pod istim clientom, kontaktirajte podršku — mi ćemo dodati deployment ID u konfiguraciju.

#### Launch shows "Unsupported message_type"

LMS je poslao LTI poruku koju FastComments ne obrađuje (npr. `LtiSubmissionReviewRequest`). FastComments podržava samo standardne resource-link launch i deep-linking tokove. Obratite nam se ako trebate dodati određeni tip poruke.

#### Iframe se ne mijenja veličina

Većina LMS‑ova automatski prilagođava veličinu LTI iframeova. Ako vaš to ne radi, provjerite dopuštaju li postavke pokretanja LMS‑a alatu slanje postMessage događaja prema roditeljskom okviru. FastComments emitira i Canvas‑stil (`lti.frameResize`) i IMS‑spec (`org.imsglobal.lti.frameResize`) poruke za promjenu veličine.

---