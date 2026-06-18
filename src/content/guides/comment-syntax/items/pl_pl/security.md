Istnieje wiele aspektów związanych z bezpieczeństwem, gdy pozwalasz użytkownikom dodawać treści na stronę internetową
a następnie renderujesz te treści na różnych typach urządzeń.

### Zapobieganie nadużyciom formatowania

Użytkownicy mogą pisać treści, które są celowo wizualnie rozpraszające
i obniżają wartość dyskusji przez nadużywanie formatowania tekstu.

FastComments wykonuje szereg działań, aby zapobiegać nadużyciom związanym z formatowaniem:

- Długie powtarzające się kolejne nowe linie są łączone.
- Nie renderujemy nagłówków (stają się zwykłym tekstem).
- Nie zezwalamy na CSS ani niestandardowe kolory.

### Zapobieganie exploitom

Eksploity mogą powstawać w systemach, które renderują HTML. FastComments wykonuje kilka działań, aby temu zapobiec:

- Zezwalamy tylko na wyraźnie zdefiniowany zbiór tagów HTML.
- Zezwalamy tylko na wyraźnie zdefiniowany zbiór atrybutów tagów HTML.
- Oczyszczamy i sanitizujemy wszystkie dane wejściowe.
  - Odbywa się to za pomocą bibliotek [DOMPurify](https://www.npmjs.com/package/dompurify) i [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Wybraliśmy te biblioteki, ponieważ są dobrze przetestowane (mają odpowiednio ponad 4 i 1 milion pobrań tygodniowo).

To oznacza, że użytkownicy nie mogą robić rzeczy takich jak pisać tagów `<script>` lub `<style>`, ani próbować dodawać skryptów typu `onload=alert()` do obrazów lub innych treści.

Zezwalane tagi HTML to:

- `<b>`
- `<em>`
- `<u>`
- `<i>`
- `<strike>`
- `<pre>`
- `<span>`
- `<code>`
- `<img>`
- `<a>`
- `<strong>`
- `<ul>`
- `<ol>`
- `<li>`
- `<br>`

Tag `<iframe>` nie jest domyślnie dozwolony. Jeśli włączysz opcję Zezwalaj na osadzanie mediów, tagi `<iframe>` są również dozwolone, ale tylko wtedy, gdy ich źródło pochodzi z wbudowanej listy zaufanych dostawców (takich jak YouTube, Vimeo, SoundCloud i Spotify) lub z nazwy hosta, którą wyraźnie dodałeś. Tagi `<iframe>` z innych źródeł są usuwane.