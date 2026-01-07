Istnieje wiele aspektów bezpieczeństwa, gdy pozwalamy ludziom dodawać treści do strony internetowej, a następnie renderować te treści na wielu różnych typach urządzeń.

### Zapobieganie nadużyciom formatowania

Ludzie mogą pisać treści, które są celowo wizualnie rozpraszające i obniżają wartość dyskusji poprzez nadużywanie formatowania tekstu.

FastComments robi wiele rzeczy, aby zapobiec nadużyciom w zakresie formatowania:

- Duże powtarzające się kolejne znaki nowej linii są zwijane.
- Nie renderujemy nagłówków (stają się zwykłym tekstem).
- Nie zezwalamy na CSS ani niestandardowe kolory.

### Zapobieganie exploitom

Exploity mogą być tworzone w systemach renderujących HTML. FastComments robi kilka rzeczy, aby temu zapobiec:

- Zezwalamy tylko na jawnie zdefiniowany zestaw tagów HTML.
- Zezwalamy tylko na jawnie zdefiniowany zestaw atrybutów tagów HTML.
- Oczyszczamy i sanityzujemy wszystkie dane wejściowe.
  - Odbywa się to za pomocą bibliotek [DOMPurify](https://www.npmjs.com/package/dompurify) i [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Wybraliśmy te biblioteki jako dobrze przetestowane (z ponad 4 i 1 milionem pobrań tygodniowo, odpowiednio).

Oznacza to, że użytkownicy nie mogą robić rzeczy takich jak pisanie tagów `<script>` lub `<style>`, ani próbować dodawać skryptów typu `onload=alert()` do obrazów lub innych treści.

Tagi HTML, które zezwalamy, są następujące:

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
