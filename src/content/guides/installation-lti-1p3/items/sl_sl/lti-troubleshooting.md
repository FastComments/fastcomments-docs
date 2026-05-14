---
#### "Registracijski žeton ni bil najden, je potekel ali je že bil uporabljen"

Žeton v vaši registracijski URL je veljaven 30 minut in je lahko uporabljen samo enkrat. Če je vaš LMS potreboval več časa ali je bila registracija ponovno poskusjena po uspehu, bo žeton zavrnjen. Ustvarite nov URL na strani FastComments LTI 1.3 Configuration in začnite znova.

#### "Platforma je zavrnila registracijo"

Vaš LMS je zavrnil postopek registracije. Najpogostejši vzroki:

- **Orodje je že registrirano z istim client name.** Nekatere platforme (zlasti D2L) zavrnejo drugo registracijo "FastComments", dokler prejšnje ni izbrisano. Odstranite staro orodje v vašem LMS, nato poskusite znova.
- **Napačno polje v LMS.** Prepričajte se, da ste URL prilepili v polje **registration / tool initiation registration endpoint**, ne v polje launch URL ali login URL.
- **LMS dejansko ne podpira Dynamic Registration.** Starejše različice Moodla in Blackboarda oglašujejo LTI 1.3, vendar dovoljujejo samo ročno konfiguracijo. Preverite dokumentacijo svoje platforme.

#### "Neuspešno pridobivanje konfiguracije platforme"

FastComments ni mogel prebrati dokumenta openid-configuration vašega LMS. To je redko in običajno pomeni, da je LMS posredoval nepravilno oblikovan ali nedosegljiv discovery URL. Obrnite se na podporo vašega LMS.

#### Ob zagonu se prikaže "Configuration not found"

Bodisi je bila konfiguracija v FastComments izbrisana, ali pa je zagon prišel iz para `iss`/`client_id`, ki ga ne prepoznamo. Če ste izbrisali in ponovno registrirali, prosite svoj LMS, naj odstrani in znova doda orodje FastComments, da bo prejel nov client_id.

#### Ob zagonu se prikaže "Deployment not registered"

Zagnali ste FastComments iz namestitve Brightspace/Moodle/Blackboard, ki se razlikuje od tiste, v kateri je bil orodje prvič zagnano. FastComments ob prvem zagonu pripne `deployment_id` kot varnostni ukrep. Če želite dodati novo namestitev znotraj istega klienta, se obrnite na podporo - dodali bomo deployment ID v konfiguracijo.

#### Ob zagonu se prikaže "Unsupported message_type"

LMS je poslal LTI sporočilo, ki ga FastComments ne obravnava (npr. `LtiSubmissionReviewRequest`). FastComments podpira samo standardne tokove resource-link zagona in deep-linking. Obrnite se na nas, če potrebujete dodaten specifičen message type.

#### Iframe se ne prilagodi velikosti

Večina LMS-ov samodejno prilagodi velikost LTI iframe-ov. Če vaš tega ne počne, preverite, ali nastavitve zagona LMS dovoljujejo orodju pošiljanje postMessage dogodkov v nadrejeni okvir. FastComments oddaja tako Canvas-slog (`lti.frameResize`) kot IMS-spec (`org.imsglobal.lti.frameResize`) sporočila za spreminjanje velikosti.

---