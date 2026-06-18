Існує кілька аспектів безпеки при дозволі людям додавати вміст на вебсайт
і потім відображати цей вміст на різних типах пристроїв.

### Запобігання зловживанню форматуванням

Люди можуть писати вміст, який навмисно візуально відволікає
та знижує цінність обговорень шляхом зловживання текстовим форматуванням.

FastComments робить низку заходів для запобігання зловживанням форматуванням:

- Великі повторювані послідовні порожні рядки стискаються.
- Ми не відображаємо заголовки (вони стають звичайним текстом).
- Ми не дозволяємо CSS або користувацькі кольори.

### Запобігання експлойтам

Експлойти можуть виникати в системах, які рендерять HTML. FastComments робить кілька речей, щоб цьому запобігти:

- Ми дозволяємо лише явно визначений набір HTML-тегів.
- Ми дозволяємо лише явно визначений набір атрибутів HTML-тегів.
- Ми очищуємо та санітуємо всі вхідні дані.
  - Це здійснюється за допомогою бібліотек [DOMPurify](https://www.npmjs.com/package/dompurify) та [sanitizeHtml](https://www.npmjs.com/package/sanitize-html).
  - Ми обрали ці бібліотеки як добре протестовані (мають понад 4 та 1 мільйон завантажень на тиждень відповідно).

Це означає, що користувачі не можуть робити такі речі, як писати `<script>` або `<style>` теги, або намагатися додати скрипти типу `onload=alert()` до зображень чи іншого вмісту.

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