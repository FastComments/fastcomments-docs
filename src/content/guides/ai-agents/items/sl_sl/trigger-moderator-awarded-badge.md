Sproži se, ko moderator podeli značko uporabniku.

### Kontekst, ki ga agent prejme

- **badge ID** podeljene značke.
- **triggering user ID** - moderator, ki je podelil značko.
- Neobvezni kontekst nit / zgodovina uporabnika / stran, kot je konfigurirano.

Na mestu sprožitve obvestilo o sprožilcu **ne** vključuje `commentId`, tudi če je bila značka podeljena v zvezi z določenim komentarjem.

### Kdo to sproži

Dejanje človeškega moderatorja.

### Pomembno

- Vključena je samo badge ID; agent ne prejme metapodatkov značke (name, image). Če mora agent ugotoviti, katera značka je bila podeljena, vključi ta kontekst v [initial prompt](#personality-prompt) ali [community guidelines](#community-guidelines).
- Sprožilec se aktivira enkrat za vsako podelitev značke, ne na uporabnika. Če isti znački dodelite istemu uporabniku dvakrat, se sprožilec aktivira dvakrat (vsaka podelitev je ločen dogodek).

### Pogoste uporabe

- **Vzajemno priznanje** - agent lahko objavi odgovor "hvala za odličen prispevek", ko je podeljena določena značka.
- **Zunanji potek priznanj** preko [Webhooks](#webhooks-overview) - zrcalite podelitve značk v vaš sistem za angažiranje uporabnikov.
- **Shranitev v spomin** - zapiski, kot na primer "ta uporabnik je priznan prispevalec", da bodo prihodnji moderacijski agenti to upoštevali pri svojih odločitvah.

---