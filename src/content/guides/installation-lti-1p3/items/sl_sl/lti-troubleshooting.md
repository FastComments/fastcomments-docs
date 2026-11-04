#### "Registracijski žeton ni bil najden, je potekel ali pa je že bil uporabljen"

Žeton v vašem registracijskem URL-ju (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">pridobite ga tukaj</a>) je veljaven 30 minut in ga je mogoče uporabiti samo enkrat. Če je vaš LMS potreboval več časa, ali če je bila registracija ponovljena po uspešni izvedbi, bo žeton zavrnjen. Ustvarite svež URL na strani za konfiguracijo FastComments LTI 1.3 in začnite znova.

#### "Platforma je zavrnila registracijo"

Vaš LMS je zavrnil registracijski handshake. Najpogostejši vzroki:

- **Orodje je že registrirano z istim imenom odjemalca.** Nekatere platforme (predvsem D2L) zavrnejo drugo registracijo "FastComments", dokler prejšnja ni izbrisana. Odstranite staro orodje v svojem LMS in poskusite znova.
- **Napačno polje v LMS.** Prepričajte se, da ste URL prilepili v polje **registration / tool initiation registration endpoint**, ne v polje za launch URL ali login URL.
- **LMS dejansko ne podpira dinamične registracije.** Starejše različice Moodle in Blackboard oglašujejo LTI 1.3, vendar dovoljujejo samo ročno konfiguracijo. Preverite dokumentacijo svoje platforme.

#### "Napaka pri pridobivanju konfiguracije platforme"

FastComments ni mogel prebrati dokumenta openid-configuration vašega LMS. To je redko in običajno pomeni, da je LMS zagotovil napačno oblikovan ali nedosegljiv discovery URL. Kontaktirajte podporo svojega LMS.

#### Zagon prikazuje "Konfiguracija ni bila najdena"

Konfiguracija v FastComments je bila bodisi izbrisana, bodisi je zagon prišel iz para `iss`/`client_id`, ki ga ne prepoznavamo. Če ste konfiguracijo izbrisali in ponovno registrirali, navodite svoj LMS, da odstrani in ponovno doda orodje FastComments, da dobi nov client_id.

#### Zagon prikazuje "Namestitev ni registrirana"

FastComments ste zagnali iz namestitve Brightspace/Moodle/Blackboard, ki je ni tista, iz katere je bil prvič zagnan. FastComments ob prvem zagonu pripne `deployment_id` kot varnostni preverjanje. Če želite dodati novo namestitev pod istim odjemalcem, kontaktirajte podporo – dodali bomo ID namestitve v konfiguracijo.

#### Zagon prikazuje "Nepodprt message_type"

LMS je poslal sporočilo LTI, ki ga FastComments ne obravnava (npr. `LtiSubmissionReviewRequest`). FastComments podpira samo standardni zagon resource-link in tokove deep-linking. Obrnite se na nas, če potrebujete dodano določeno vrsto sporočila.

#### IFrame se ne spremeni velikosti

Večina LMS-jev samodejno prilagodi velikost LTI iframe. Če vaš tega ne počne, preverite, ali nastavitve zagona LMS dovoljujejo orodju pošiljanje dogodkov postMessage starševskemu okvirju. FastComments oddaja sporočila za spreminjanje velikosti tako v slogu Canvas (`lti.frameResize`) kot po specifikaciji IMS (`org.imsglobal.lti.frameResize`).