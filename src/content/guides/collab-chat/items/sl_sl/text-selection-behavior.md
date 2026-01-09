### Kako deluje izbira besedila

Ko uporabniki izberejo besedilo znotraj kontejnerja Collab Chat, pripomoček zajame to izbiro in jim omogoči začetek razprave. Izbira je lahko tako majhna kot ena beseda ali pa obsega več odstavkov, ki segajo čez različne elemente.

### Zamik pri izbiri

Na namiznih napravah je 3,5-sekundni zamik med trenutkom, ko uporabnik izbere besedilo, in trenutkom, ko se prikaže poziv za razpravo. To prepreči utripanje uporabniškega vmesnika, ko uporabniki besedilo le označujejo za kopiranje ali branje. Na mobilnih napravah se poziv prikaže takoj, saj je izbira besedila na zaslonih na dotik bolj premišljena.

### Enolični ID-ji pogovorov

Vsak pogovor dobi enoličen `urlId`, ki združuje URL strani, pot DOM elementa in serializirano območje besedila. To zagotavlja, da vsaka izbira besedila ustvari ločen pogovor, ki ga je mogoče kasneje znova najti.

Če v svoji konfiguraciji navedete po meri `urlId`, bo ta združen s potjo elementa in območjem besedila za ustvarjanje končnega identifikatorja.

### Vizualni poudarki

Ko za določeno izbiro besedila obstaja razprava, to besedilo dobi vizualni poudarek. Poudarek je izveden z ozadinskimi barvami in se prikaže ob premiku miške ali ko je ustrezno okno klepeta odprto.

Sistem poudarjanja deluje tako, da izbrano besedilo zavije v poseben element, ki ga je mogoče stilizirati. Ta pristop zagotavlja, da poudarki ostanejo natančni tudi, ko je osnovna HTML struktura zapletena.

### Pozicioniranje okna klepeta

Ko uporabnik klikne na poudarek ali ustvari novo označbo, se ob izbranem besedilu prikaže okno klepeta. Pripomoček samodejno izračuna najboljši položaj za to okno glede na razpoložljiv prostor v pogledu.

Sistem pozicioniranja uporablja CSS razrede, kot so `to-right`, `to-left`, `to-top` in `to-bottom`, da označi, v katero smer se mora okno klepeta razširiti glede na poudarek. Na mobilnih napravah (zasloni širši manj kot 768px) se okno klepeta vedno prikaže celozaslonsko za boljšo uporabnost.

### Dimenzije okna klepeta

Okna klepeta so na namizju široka 410px z razmikom 20px in 16px vizualno puščico, usmerjeno k poudarjenemu besedilu. Te dimenzije so fiksne, da se zagotovi dosledno vedenje, vendar lahko videz prilagodite s CSS.

### Izbire, ki segajo čez več elementov

Uporabniki lahko izberejo besedilo, ki sega čez več HTML elementov, na primer označevanje od sredine enega odstavka do začetka drugega. Sistem serializacije obsega to pravilno obravnava in bo poudaril vse izbrano besedilo, tudi čez meje elementov.

### Združljivost brskalnikov

Sistem izbire besedila uporablja standardni API `window.getSelection()`, ki ga podpirajo vsi sodobni brskalniki. Za starejše različice Internet Explorerja se za združljivost uporablja `document.selection`.

### Vzdrževanje označb

Ko je za izbiro besedila ustvarjen pogovor, ta označba ostane tudi po ponovnem nalaganju strani. Serializirano območje in pot DOM omogočata pripomočku, da ob povratku uporabnikov na stran znova obnovi poudarke na točno istem mestu.

To deluje zanesljivo, dokler vsebina vaše strani ostane stabilna. Če spremenite besedilno vsebino ali prestrukturirate svoj HTML, obstoječe označbe morda ne bodo več pravilno poravnane z besedilom. Zato je najbolje, da se izognete večjim spremembam vsebine na straneh z aktivnimi označbami ali pa razmislite o migraciji označb, kadar so spremembe vsebine nujne.