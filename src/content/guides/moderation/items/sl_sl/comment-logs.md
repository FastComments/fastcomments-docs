FastComments samodejno sledi podrobnim dogodkom za vsak komentar, da zagotavlja preglednost pri odločitvah moderacije in sistemskih dejanj. Ti dnevniki vam pomagajo razumeti, zakaj je bil komentar odobren, označen kot spam ali je bil spremenjen njegov status.

## Dostop do dnevnikov komentarjev

Za ogled dnevnikov za določen komentar:

1. Pojdite na stran **Moderate Comments** v nadzorni plošči FastComments
2. Poiščite komentar, ki ga želite pregledati
3. Kliknite gumb **View Logs** (ikona ure) v akcijski vrstici komentarja
4. Pojavil se bo dialog, ki prikazuje celotno zgodovino dogodkov za ta komentar

Vsak vnos v dnevniku prikazuje:
- **When** – Časovni žig dogodka
- **Who** – Uporabnik ali sistem, ki je sprožil dogodek (ko je to primerno)
- **What** – Vrsta dejanja ali dogodka
- **Details** – Dodatni kontekst, kot so vrednosti pred/po, imena motorjev ali povezani podatki

## Dogodki dnevnika komentarjev

Vsak komentar vodi dnevnik dogodkov, ki se zgodijo med njegovim življenjskim ciklom. Spodaj so vrste dogodkov, ki se beležijo:

### Dogodki anonimizacije
- **Anonymized** – Vsebina komentarja je bila izbrisana, uporabnik označen kot izbrisan
- **RestoredFromAnonymized** – Komentar je bil obnovljen iz anonimiziranega stanja

### Dogodki odobritve
- **ApprovedDueToPastComment** – Komentar odobren, ker je uporabnik v preteklosti odobril komentarje (vključuje sklic na prejšnji komentar)
- **ApprovedIsAdmin** – Komentar odobren, ker je uporabnik skrbnik
- **NotApprovedRequiresApproval** – Komentar zahteva ročno odobritev
- **NotApprovedLowTrustFactor** – Komentar ni odobren zaradi nizkega faktorja zaupanja uporabnika (vključuje vrednost faktorja zaupanja)

### Dogodki odobritve komentarjev na profilu
Ti dogodki se nanašajo posebej na komentarje na uporabniških profilih:
- **ApprovedProfileAutoApproveAll** – Komentar na profilu samodejno odobren, ker je lastnik profila omogočil samodejno odobritev vseh komentarjev
- **ApprovedProfileTrusted** – Komentar na profilu odobren, ker je komentator zaupanja vreden (vključuje sklic na komentar, ki je vzpostavil zaupanje)
- **NotApprovedProfileManualApproveAll** – Komentar na profilu zahteva ročno odobritev, ker je lastnik profila omogočil ročno odobritev
- **NotApprovedProfileNotTrusted** – Komentar na profilu ni odobren, ker komentator ni zaupanja vreden
- **NotApprovedProfileNewUser** – Komentar na profilu ni odobren, ker je komentator nov uporabnik

### Dogodki zaznavanja spama
- **IsSpam** – Komentar označen kot spam s strani zaznavnega motorja (vključuje, kateri motor je sprejel odločitev)
- **IsSpamDueToBadWords** – Komentar označen kot spam zaradi filtra za vulgarne besede
- **IsSpamFromLLM** – Komentar označen kot spam s strani AI/LLM motorja (vključuje ime motorja, odgovor in število žetonov)
- **IsSpamRepeatComment** – Komentar označen kot spam zaradi ponavljanja (vključuje, kateri motor je to zaznal)
- **NotSpamIsOnlyImage** – Komentar ni označen kot spam, ker vsebuje le slike
- **NotSpamIsOnlyReacts** – Komentar ni označen kot spam, ker vsebuje le reakcije
- **NotSpamNoLinkOrMention** – Komentar ni označen kot spam, ker ne vsebuje sumljivih povezav ali omemb
- **NotSpamPerfectTrustFactor** – Komentar ni označen kot spam, ker ima uporabnik visok faktor zaupanja
- **NotSpamTooShort** – Komentar ni označen kot spam, ker je prekratek za analizo
- **NotSpamSkipped** – Preverjanje spama je bilo preskočeno
- **NotSpamFromEngine** – Komentar je bil določen kot ne-spam s strani zaznavnega motorja (vključuje ime motorja in faktor zaupanja)

### Dogodki za vulgarne besede/profanost
- **BadWordsCheckFailed** – Preverjanje filtra za vulgarne besede je naletelo na napako
- **BadWordsFoundBadPhrase** – Filter za vulgarne besede je zaznal neprimerno frazo (vključuje frazo)
- **BadWordsFoundBadWord** – Filter za vulgarne besede je zaznal neprimerno besedo (vključuje besedo)
- **BadWordsNoDefinitionForLocale** – Za jezik komentarja ni na voljo definicij vulgarnih besed (vključuje jezikovno oznako)

### Dogodki preverjanja uporabnika
- **CommentMustBeVerifiedToApproveNotInVerifiedSession** – Komentar zahteva preverjanje, vendar uporabnik ni v preverjeni seji
- **CommentMustBeVerifiedToApproveNotVerifiedYet** – Komentar zahteva preverjanje, vendar uporabnik še ni preverjen
- **InVerifiedSession** – Uporabnik, ki objavlja komentar, je v preverjeni seji
- **SentVerificationEmailNoSession** – Potrditveno e-pošto je poslano nepreverjenemu uporabniku
- **SentWelcomeEmail** – Pozdravno e-pošto je poslano novemu uporabniku

### Dogodki zaupanja in varnosti
- **TrustFactorChanged** – Faktor zaupanja uporabnika je bil spremenjen (vključuje vrednosti pred in po)
- **SpamFilterDisabledBecauseAdmin** – Filtriranje spama je bilo obiti za skrbniškega uporabnika
- **TenantSpamFilterDisabled** – Filtriranje spama je onemogočeno za celotnega najemnika
- **RepeatCommentCheckIgnored** – Preverjanje ponavljajočih se komentarjev je bilo obiti (vključuje razlog)
- **UserIsAdmin** – Uporabnik je identificiran kot skrbnik
- **UserIsAdminParentTenant** – Uporabnik je identificiran kot skrbnik nadrejene najemniške enote
- **UserIsAdminViaSSO** – Uporabnik je identificiran kot skrbnik prek SSO
- **UserIsMod** – Uporabnik je identificiran kot moderator

### Spremembe statusa komentarja
Dogodki spremembe statusa vključujejo vrednosti pred in po ter uporabnika, ki je opravil spremembo:
- **ExpireStatusChanged** – Status izteka komentarja je bil spremenjen
- **ReviewStatusChanged** – Status pregleda komentarja je bil spremenjen
- **SpamStatusChanged** – Status spama komentarja je bil posodobljen
- **ApproveStatusChanged** – Status odobritve komentarja je bil spremenjen
- **TextChanged** – Besedilo komentarja je bilo urejeno (vključuje besedilo pred in po)
- **VotesChanged** – Število glasov komentarja je bilo posodobljeno (vključuje podroben razčlenitev glasov)
- **Flagged** – Komentar je bil označen s strani uporabnikov
- **UnFlagged** – Oznake komentarja so bile odstranjene

### Dejavnosti moderacije
- **Pinned** – Komentar je moderator pripeljal (vključuje, kdo ga je pripeljal)
- **UnPinned** – Komentar je moderator odpilil (vključuje, kdo ga je odpiljal)

### Dogodki obvestil
- **CreatedNotifications** – Za komentar so bila ustvarjena obvestila (vključuje število obvestil)
- **NotificationCreateFailure** – Ustvarjanje obvestil je spodletelo
- **BadgeAwarded** – Uporabniku je bila dodeljena značka za komentar (vključuje ime značke)

### Dogodki omemb in obvestil o odgovorih
Ti dogodki navajajo osebo, ki bi prejela e-pošto ali obvestilo. Ko nič ni bilo poslano, stolpec Details pojasni zakaj.
- **MentionEmailSent** – Uporabnik, omenjen v komentarju, je prejel e-pošto
- **MentionEmailSkipped** – Omenjenemu uporabniku e-pošta ni bila poslana (vključuje razlog)
- **MentionHeldForApproval** – E-pošta z omembo čaka, dokler komentar ni odobren
- **MentionNotificationCreated** – Omenjeni uporabnik je prejel obvestilo v aplikaciji
- **MentionNotificationSkipped** – Omenjeni uporabnik ni prejel obvestila v aplikaciji (vključuje razlog)
- **ReplyEmailSent** – Avtor komentarja, na katerega je odgovor, je prejel e-pošto o tem odgovoru
- **ReplyEmailSkipped** – Avtor komentarja, na katerega je odgovor, ni prejel e-pošte (vključuje razlog)
- **ReplyNotificationSkipped** – Avtor komentarja, na katerega je odgovor, ni prejel obvestila v aplikaciji (vključuje razlog)

Razlogi, prikazani, ko e-pošta ali obvestilo ni bilo poslano:
- Uporabnik ne obstaja več ali nima e-poštnega naslova
- Uporabnik je izklopil e-poštna obvestila ali izklopil obvestila za to nit
- Eden od dveh uporabnikov je blokiral drugega
- Uporabnika nista v isti SSO skupini
- E-poštni naslov uporabnika je na seznamu za zadrževanje po odbijanju ali pritožbi o spamu (glejte [Email Suppression Management](/guide-notifications.html#email-suppression-management))
- E-poštni naslov uporabnika je na example.com, ki ne more prejemati e-pošte
- Komentar je bil označen kot spam, izbrisan ali ni bil odobren v 7 dneh
- Komentar, na katerega je odgovor, je bil napisan anonimno
- Uporabnik je odgovoril na svoj komentar
- Uporabnik je bil omenjen v odgovoru, zato je prejel e-pošto z omembo namesto e-pošte z odgovorom
- Uporabnik je že imel obvestilo o odgovoru za komentar
- Pošiljanje je spodletelo 5-krat

Če pošiljanje ne uspe ali doseže omejitev pošiljanja, je e-pošta postavljena v čakalno vrsto za ponoven poskus in vnos v dnevniku to zabeleži.

### Dogodki objavljanja
- **PublishedLive** – Komentar je bil objavljen živim naročnikom (vključuje število naročnikov)

### Dogodki integracije
- **WebhookSynced** – Komentar je bil sinhroniziran prek webhooka

### Dogodki pravil spama
- **SpamRuleMatch** – Komentar se ujema s prilagojenim pravilom spama (vključuje podrobnosti pravila)

### Dogodki lokalizacije
- **LocaleDetectedFromText** – Jezikovna oznaka je bila samodejno zaznana iz besedila komentarja (vključuje zaznan jezik in oznako)

## Primeri uporabe dnevnikov komentarjev

Dnevniki komentarjev se samodejno ustvarijo in shranijo z vsakim komentarjem. Ponujajo dragocene vpoglede za:
- **Understanding moderation decisions** – Natančno vidite, zakaj je bil komentar odobren, zadržan za pregled ali označen kot spam
- **Debugging approval/spam issues** – Sledite logiki odločanja, ko komentarji ne delujejo po pričakovanjih
- **Tracking user behavior patterns** – Spremljajte spremembe faktorja zaupanja in statusa preverjanja
- **Auditing moderator actions** – Preglejte, katere ukrepe so moderatorji izvedli na določenih komentarjih
- **Investigating spam filter effectiveness** – Oglejte si, kateri zaznavni motorji ujamejo spam in kateri ne
- **Troubleshooting integrations** – Preverite sinhronizacije webhookov in dostavo obvestil

Ti dnevniki pomagajo ohranjati preglednost v procesu moderacije in pomagajo pri fino nastavljanju delovanja vašega sistema komentarjev.