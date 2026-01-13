Morda boste opazili, da naše metrike Analytics prikazujejo nekoliko drugačne številke kot npr. Google Ads © ali podobni izdelki.

Za spletne strani z enim pripomočkom za komentarje na stran so številke, ki jih zagotavlja FastComments Analytics, zelo natančne, in če so napačne, bodo **nižje** od dejanske vrednosti, vendar ne več.

Če imate SPA, boste morda opazili, da so številke FastComments Analytics višje od tistih, ki jih poročajo vaši marketinški izdelki. To je zato, ker marketinški izdelek morda sledi samo, ko stran ni naložena, ne pa vsakič, ko uporabnik naredi nekaj na strani, kar bi lahko sprožilo prikaz nove niti komentarjev, kar bi se štelo kot nalaganje strani za FastComments Analytics.

#### Tehnične informacije

FastComments Analytics sledi vsakemu nalaganju strani in se ne zanaša na naključnost kot optimizacijo. Vsako nalaganje strani povzroči posodobitev števca v pomnilniku v vsaki niti na vsakem strežniku, ki se nato občasno shrani v bazo podatkov prek atomske operacije.
