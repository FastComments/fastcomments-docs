Skript za vsako razširitev se pridobi in zažene, preden pripomoček za komentarje začne pridobivati prvi niz komentarjev in upodabljati uporabniški vmesnik.

Ob prvem nalaganju bodo na objekt razširitve pripeti naslednji podatki:

- `config` - Sklic na objekt `config`.
- `translations` - Sklic na objekt `translations`.
- `commentsById` - Sklic na vse komentarje po id.
- `root` - Sklic na korensko DOM vozlišče.

Razširitve naj preglasijo želene funkcije, ki jih bo pripomoček za komentarje poklical ob ustreznih časih.