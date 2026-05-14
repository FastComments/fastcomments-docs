#### "Registracijski žeton ni najden, je potekel ali je že uporabljen"

Žeton v vaši registracijski URL (<a href="https://fastcomments.com/auth/my-account/lti-config" target="_blank">pridobite ga tukaj</a>) je veljaven 30 minut in ga je mogoče uporabiti le enkrat. Če je vaš LMS potreboval več časa, ali če je bila registracija ponovljena po uspehu, bo žeton zavrnjen. Generirajte novo URL na strani za konfiguracijo FastComments LTI 1.3 in začnite znova.

#### "Platform rejected registration"

Vaš LMS je zavrnil vzpostavitev registracije. Najpogostejši vzroki:

- **Tool already registered with the same client name.** Nekatere platforme (zlasti D2L) zavrnejo drugo registracijo "FastComments", dokler prejšnje ni izbrisano. Odstranite staro orodje v vašem LMS, nato poskusite znova.
- **Wrong field in the LMS.** Prepričajte se, da ste prilepili URL v polje **registration / tool initiation registration endpoint**, ne v polje launch URL ali login URL.
- **The LMS doesn't actually support Dynamic Registration.** Starejše različice Moodle in Blackboard oglašujejo LTI 1.3, vendar dovoljujejo le ročno konfiguracijo. Preverite dokumentacijo vaše platforme.

#### "Failed to fetch platform configuration"

FastComments ni mogel prebrati dokumenta openid-configuration vašega LMS. To je redko in običajno pomeni, da je LMS posredoval nepravilno oblikovan ali nedosegljiv discovery URL. Obrnite se na podporo vašega LMS.

#### "Launch shows "Configuration not found""

Konfiguracija v FastComments je bila ali izbrisana, ali pa je zagon prišel iz para `iss`/`client_id`, ki ga ne prepoznamo. Če ste izbrisali in ponovno registrirali, povejte vašemu LMS, naj odstrani in ponovno doda orodje FastComments, da bo dobil nov client_id.

#### "Launch shows "Deployment not registered""

Zagnali ste FastComments iz namestitve Brightspace/Moodle/Blackboard, ki se razlikuje od tiste, v kateri je bil prvič zagnan. FastComments ob prvem zagonu zabeleži `deployment_id` kot varnostni ukrep. Če želite dodati novo namestitev pod istim klientom, kontaktirajte podporo - mi bomo dodali ID namestitve v konfiguracijo.

#### "Launch shows "Unsupported message_type""

LMS je poslal LTI sporočilo, s katerim FastComments ne upravlja (npr. `LtiSubmissionReviewRequest`). FastComments podpira le standardni resource-link launch in deep-linking flows. Obrnite se na nas, če potrebujete dodan določen tip sporočila.

#### Iframe doesn't resize

Večina LMS-ov samodejno prilagodi velikost LTI iframe-ov. Če vaš tega ne počne, preverite, ali nastavitve zagona v LMS dovoljujejo orodju pošiljanje dogodkov postMessage v nadrejeni okvir. FastComments oddaja tako sporočila za spreminjanje velikosti v slogu Canvas (`lti.frameResize`) kot po specifikaciji IMS (`org.imsglobal.lti.frameResize`).