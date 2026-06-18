Постоји више аспеката безбедности када дозволите корисницима да додају садржај на веб-сајт и потом приказујете тај садржај на разним типовима уређаја.

### Спречавање злоупотребе форматирања

Људи могу писати садржај који је намерно визуелно ометајући и умањује вредност дискусија злоупотребом форматирања текста.

FastComments предузима низ мера да спречи злоупотребу у вези са форматирањем:

- Велики понављајући узастопни нови редови се сабијају.
- Не приказујемо наслове (они постају обичан текст).
- Не дозвољавамо CSS или прилагођене боје.

### Спречавање експлоатација

Експлоатације могу бити креиране у системима који приказују HTML. FastComments предузима неколико мера да то спречи:

- Дозвољавамо само експлицитно дефинисан скуп HTML тагова.
- Дозвољавамо само експлицитно дефинисан скуп атрибута HTML тагова.
- Прочишћавамо и санитизујемо све уносе.
  - Ово се ради помоћу библиотека [DOMPurify](https://www.npmjs.com/package/dompurify) и [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Ове библиотеке смо одабрали као добро тестиране (DOMPurify преко 4 милиона, sanitizeHtml преко 1 милиона преузимања недељно).

То значи да корисници не могу радити ствари као што су писање `<script>` или `<style>` тагова, или покушај додавања скрипти типа `onload=alert()` на слике или други садржај.

The HTML tags we allow are as follows:

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

The `<iframe>` tag is not allowed by default. If you turn on Allow Media Embeds, iframes are also permitted, but only when their source is one of a built-in list of trusted providers (such as YouTube, Vimeo, SoundCloud, and Spotify) or a hostname you have explicitly added. Iframes from any other source are removed.